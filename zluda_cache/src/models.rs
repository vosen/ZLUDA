use crate::schema::modules;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = modules)]
pub(crate) struct AddModule<'a> {
    pub hash: &'a str,
    pub compiler_version: &'a str,
    pub zluda_version: &'a str,
    pub device: &'a str,
    pub backend_key: &'a str,
    pub binary: &'a [u8],
    pub last_access: i64,
}
