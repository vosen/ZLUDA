use itertools::Itertools;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use rusqlite::Connection;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::{self, SystemTime};

pub trait KernelExtendedData {
    const INPUT_COLUMNS: &'static [[&'static str; 2]];
}

pub struct KernelRepository<T: KernelExtendedData> {
    cache_file: Option<PathBuf>,
    insert_kernel: String,
    update_last_used: String,
    select_kernel: String,
    _marker: PhantomData<T>,
}

// We do all that string concatenation during run time, because there is no
// good alternative. const_format crate implements const-time format!(),
// but it does not work with generic types. Maybe it works with &'static str
// const generics, but &'static str const generics are currently illegal
impl<T: KernelExtendedData> KernelRepository<T> {
    pub fn new(cache_file: Option<PathBuf>) -> rusqlite::Result<Self> {
        let create_tables = {
            let input_columns = Self::columns_comma(T::INPUT_COLUMNS);
            let input_columns_with_type = Self::columns_comma_type(T::INPUT_COLUMNS);
            format!("
            CREATE TABLE IF NOT EXISTS kernels (
                id INTEGER PRIMARY KEY NOT NULL,
                hash TEXT NOT NULL,
                compiler_version TEXT NOT NULL,
                git_hash TEXT NOT NULL,
                device TEXT NOT NULL,
                is_debug INTEGER NOT NULL,
                is_windows BOOLEAN NOT NULL,
                binary BLOB NOT NULL,
                last_used INTEGER NOT NULL
                {input_columns_with_type}
            );
            CREATE UNIQUE INDEX IF NOT EXISTS kernels_index ON kernels (hash, compiler_version, git_hash, device, is_windows, is_debug {input_columns});
            CREATE TABLE IF NOT EXISTS globals (
                key TEXT PRIMARY KEY,
                value INTEGER NOT NULL
            ) WITHOUT ROWID;
            CREATE TRIGGER IF NOT EXISTS update_size_on_delete 
            AFTER 
                DELETE ON kernels FOR EACH ROW BEGIN 
            UPDATE 
                globals 
            SET 
                value = value - length(OLD.binary) 
            WHERE 
                key = 'total_binary_size';
            END;
            CREATE TRIGGER IF NOT EXISTS update_size_on_insert
            AFTER 
                INSERT ON kernels FOR EACH ROW BEGIN 
            UPDATE 
                globals 
            SET 
                value = value + length(NEW.binary) 
            WHERE 
                key = 'total_binary_size';
            END;
            INSERT OR IGNORE INTO globals (key, value) VALUES ('total_binary_size', 0);")
        };
        let insert_kernel = {
            let input_columns = Self::columns_comma(T::INPUT_COLUMNS);
            let arg_markers = (0..T::INPUT_COLUMNS.len())
                .into_iter()
                .format_with("", |_, f| f(&", ?"));
            format!("
            INSERT INTO
                kernels (last_used, hash, compiler_version, git_hash, device, is_windows, is_debug, binary {input_columns})
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8 {arg_markers}) ON CONFLICT DO
            UPDATE
            SET
                last_used = ?1;")
        };
        let update_last_used = {
            let input_columns = Self::columns_and(T::INPUT_COLUMNS);
            format!("
            UPDATE
                kernels
            SET
                last_used = ?1
            WHERE
                hash = ?2 AND compiler_version = ?3 AND git_hash = ?4 AND device = ?5 AND is_windows = ?6 AND is_debug = ?7 {input_columns};")
        };
        let select_kernel = {
            let input_columns = Self::columns_and(T::INPUT_COLUMNS);
            format!("
            SELECT
                id, binary
            FROM 
                kernels
            WHERE
                hash = ?1 AND compiler_version = ?2 AND git_hash = ?3 AND device = ?4 AND is_windows = ?5 AND is_debug = ?6 {input_columns};")
        };
        let result = Self {
            cache_file,
            insert_kernel,
            update_last_used,
            select_kernel,
            _marker: PhantomData,
        };
        let mut connection = result.connect()?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "synchronous", "normal")?;
        // Deferred transaction here can lead to SQLITE_BUSY errors
        let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        {
            tx.set_db_config(
                rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY,
                true,
            )?;
            tx.set_db_config(
                rusqlite::config::DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER,
                true,
            )?;
            tx.execute_batch(&create_tables)?;
        }
        tx.commit()?;
        Ok(result)
    }

    pub fn connect(&self) -> rusqlite::Result<Connection> {
        match self.cache_file {
            Some(ref file) => rusqlite::Connection::open(file),
            None => rusqlite::Connection::open_in_memory(),
        }
    }

    pub fn save_program(
        &self,
        now: i64,
        hash: &str,
        compiler_version: &str,
        git_hash: &str,
        device: &CStr,
        binary: &[u8],
        input_values: &[&dyn rusqlite::ToSql],
    ) -> rusqlite::Result<()> {
        let connection = self.connect()?;
        let tx = connection.unchecked_transaction()?;
        {
            let mut insert_kernel = tx.prepare(&self.insert_kernel)?;
            let common_values = rusqlite::params![
                now,
                hash,
                compiler_version,
                git_hash,
                SqlCStrRef(device),
                cfg!(windows),
                cfg!(debug_assertions),
                binary
            ];
            insert_kernel.execute(rusqlite::params_from_iter(
                IntoIterator::into_iter([common_values, input_values]).flatten(),
            ))?;
        }
        tx.commit()
    }

    pub fn try_load_program(
        &self,
        now: i64,
        hash: &str,
        compiler_version: &str,
        git_hash: &str,
        device: &CStr,
        input_values: &[&dyn rusqlite::ToSql],
    ) -> rusqlite::Result<Option<Vec<u8>>> {
        let connection = self.connect()?;
        let tx = connection.unchecked_transaction()?;
        let result;
        {
            let common_params = rusqlite::params![
                now,
                hash,
                compiler_version,
                git_hash,
                SqlCStrRef(device),
                cfg!(windows),
                cfg!(debug_assertions)
            ];
            let mut statement = tx.prepare(&self.update_last_used)?;
            let rows_affected = statement.execute(rusqlite::params_from_iter(
                IntoIterator::into_iter([common_params, input_values]).flatten(),
            ))?;
            if rows_affected == 0 {
                return Ok(None);
            }
            let mut select_kernel = tx.prepare(&self.select_kernel)?;
            let mut kernel_rows = select_kernel.query(rusqlite::params_from_iter(
                IntoIterator::into_iter([&common_params[1..], input_values]).flatten(),
            ))?;
            if let Some(row) = kernel_rows.next()? {
                let binary = row.get::<_, Vec<u8>>(1)?;
                result = Some(binary);
            } else {
                result = None;
            }
        }
        tx.commit()?;
        Ok(result)
    }

    fn columns_comma(columns: &'static [[&'static str; 2]]) -> impl Display {
        columns
            .iter()
            .format_with("", |[name, _], f| f(&format_args!(", {name}")))
    }

    fn columns_comma_type(columns: &'static [[&'static str; 2]]) -> impl Display {
        columns
            .iter()
            .format_with("", |[name, type_], f| f(&format_args!(", {name} {type_}")))
    }

    fn columns_and(columns: &'static [[&'static str; 2]]) -> impl Display {
        columns
            .iter()
            .format_with("", |[name, _], f| f(&format_args!(" AND {name} = ?")))
    }

    pub fn now() -> Result<i64, time::SystemTimeError> {
        let now = SystemTime::now().duration_since(time::UNIX_EPOCH)?;
        Ok(now.as_millis() as i64)
    }
}

pub struct SqlCStrRef<'a>(pub &'a CStr);

impl<'a> rusqlite::ToSql for SqlCStrRef<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Borrowed(
            rusqlite::types::ValueRef::Text(self.0.to_bytes()),
        ))
    }
}

pub struct SqlCString(pub CString);

impl FromSql for SqlCString {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let str = value.as_str()?;
        Ok(SqlCString(
            CString::new(str.to_string()).map_err(|err| FromSqlError::Other(Box::new(err)))?,
        ))
    }
}
