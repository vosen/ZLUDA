use crate::schema::modules;
use arrayvec::ArrayString;
use diesel::{connection::SimpleConnection, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::time::Duration;

pub(crate) mod models;
pub(crate) mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct ModuleKey<'a> {
    pub hash: ArrayString<64>,
    pub compiler_version: &'static str,
    pub zluda_version: &'static str,
    pub device: &'a str,
    pub backend_key: String,
    pub last_access: i64,
}

pub struct ModuleCache(SqliteConnection);

impl ModuleCache {
    pub fn create_cache_dir_and_get_path() -> Option<String> {
        let mut cache_dir = dirs::cache_dir()?;
        cache_dir.extend(["zluda", "ComputeCache"]);
        // We ensure that the cache directory exists
        std::fs::create_dir_all(&cache_dir).ok()?;
        // No need to create the file, it will be created by SQLite on first access
        cache_dir.push("zluda.db");
        Some(cache_dir.to_string_lossy().into())
    }

    pub fn open(file_path: &str) -> Option<Self> {
        let mut conn = SqliteConnection::establish(file_path).ok()?;
        conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
            .ok()?;
        conn.run_pending_migrations(MIGRATIONS).ok()?;
        Some(Self(conn))
    }

    pub fn get_module_binary(&mut self, key: &ModuleKey) -> Option<Vec<u8>> {
        diesel::update(modules::dsl::modules)
            .set(modules::last_access.eq(key.last_access))
            .filter(modules::hash.eq(key.hash.as_str()))
            .filter(modules::compiler_version.eq(&key.compiler_version))
            .filter(modules::zluda_version.eq(key.zluda_version))
            .filter(modules::device.eq(key.device))
            .filter(modules::backend_key.eq(&key.backend_key))
            .returning(modules::binary)
            .get_result(&mut self.0)
            .ok()
    }

    pub fn insert_module(&mut self, key: &ModuleKey, binary: &[u8]) {
        diesel::insert_into(modules::dsl::modules)
            .values(models::AddModule {
                hash: key.hash.as_str(),
                compiler_version: &key.compiler_version,
                zluda_version: key.zluda_version,
                device: key.device,
                backend_key: &key.backend_key,
                last_access: key.last_access,
                binary,
            })
            .execute(&mut self.0)
            .ok();
    }

    pub fn time_now() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        schema::{globals::dsl::*, modules::dsl::*},
        ModuleCache,
    };
    use arrayvec::ArrayString;
    use diesel::prelude::*;

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::modules)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Module {
        pub id: i64,
        pub hash: String,
        pub binary: Vec<u8>,
        pub last_access: i64,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::globals)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Global {
        pub key: String,
        pub value: i64,
    }

    #[test]
    fn empty_db_returns_no_module() {
        let mut db = ModuleCache::open(":memory:").unwrap();
        let module_binary = db.get_module_binary(&super::ModuleKey {
            hash: ArrayString::from("test_hash").unwrap(),
            compiler_version: "1.0.0",
            zluda_version: "1.0.0",
            device: "test_device",
            backend_key: "{}".to_string(),
            last_access: 123,
        });
        assert!(module_binary.is_none());
        let all_modules = modules.select(Module::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_modules.len(), 0);
        let all_globals: Vec<Global> = globals.select(Global::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_globals[0].key, "total_size");
        assert_eq!(all_globals[0].value, 0);
    }

    #[test]
    fn newly_inserted_module_increments_total_size() {
        let mut db = ModuleCache::open(":memory:").unwrap();
        db.insert_module(
            &super::ModuleKey {
                hash: ArrayString::from("test_hash1").unwrap(),
                compiler_version: "1.0.0",
                zluda_version: "1.0.0",
                device: "test_device",
                backend_key: "{}".to_string(),
                last_access: 123,
            },
            &[1, 2, 3, 4, 5],
        );
        db.insert_module(
            &super::ModuleKey {
                hash: ArrayString::from("test_hash2").unwrap(),
                compiler_version: "1.0.0",
                zluda_version: "1.0.0",
                device: "test_device",
                backend_key: "{}".to_string(),
                last_access: 124,
            },
            &[1, 2, 3],
        );
        let mut all_modules = modules.select(Module::as_select()).load(&mut db.0).unwrap();
        all_modules.sort_by_key(|m: &Module| m.id);
        assert_eq!(all_modules.len(), 2);
        assert_eq!(all_modules[0].hash, "test_hash1");
        assert_eq!(all_modules[0].last_access, 123);
        assert_eq!(all_modules[0].binary, &[1, 2, 3, 4, 5]);
        assert_eq!(all_modules[1].hash, "test_hash2");
        assert_eq!(all_modules[1].last_access, 124);
        assert_eq!(all_modules[1].binary, &[1, 2, 3]);
        let all_globals = globals.select(Global::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_globals[0].key, "total_size");
        assert_eq!(all_globals[0].value, 8);
    }

    #[test]
    fn get_bumps_last_access() {
        let mut db = ModuleCache::open(":memory:").unwrap();
        db.insert_module(
            &super::ModuleKey {
                hash: ArrayString::from("test_hash").unwrap(),
                compiler_version: "1.0.0",
                zluda_version: "1.0.0",
                device: "test_device",
                backend_key: "{}".to_string(),
                last_access: 123,
            },
            &[1, 2, 3, 4, 5],
        );
        let module_binary = db
            .get_module_binary(&super::ModuleKey {
                hash: ArrayString::from("test_hash").unwrap(),
                compiler_version: "1.0.0",
                zluda_version: "1.0.0",
                device: "test_device",
                backend_key: "{}".to_string(),
                last_access: 124,
            })
            .unwrap();
        let all_modules = modules.select(Module::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_modules.len(), 1);
        assert_eq!(all_modules[0].last_access, 124);
        assert_eq!(module_binary, &[1, 2, 3, 4, 5]);
        assert_eq!(all_modules[0].binary, &[1, 2, 3, 4, 5]);
        let all_globals = globals.select(Global::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_globals[0].key, "total_size");
        assert_eq!(all_globals[0].value, 5);
    }

    #[test]
    fn double_insert_does_not_override() {
        let mut db = ModuleCache::open(":memory:").unwrap();
        db.insert_module(
            &super::ModuleKey {
                hash: ArrayString::from("test_hash").unwrap(),
                compiler_version: "1.0.0",
                zluda_version: "1.0.0",
                device: "test_device",
                backend_key: "{}".to_string(),
                last_access: 123,
            },
            &[1, 2, 3, 4, 5],
        );
        db.insert_module(
            &super::ModuleKey {
                hash: ArrayString::from("test_hash").unwrap(),
                compiler_version: "1.0.0",
                zluda_version: "1.0.0",
                device: "test_device",
                backend_key: "{}".to_string(),
                last_access: 124,
            },
            &[5, 4, 3, 2, 1],
        );
        let all_modules = modules.select(Module::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_modules.len(), 1);
        assert_eq!(all_modules[0].last_access, 123);
        assert_eq!(all_modules[0].binary, &[1, 2, 3, 4, 5]);
        let all_globals = globals.select(Global::as_select()).load(&mut db.0).unwrap();
        assert_eq!(all_globals[0].key, "total_size");
        assert_eq!(all_globals[0].value, 5);
    }
}
