use diesel::prelude::*;
use crate::schema::modules;

#[derive(Insertable)]
#[diesel(table_name = modules)]
pub(crate) struct AddModule<'a> {
    pub hash: &'a str,
    pub compiler_version: &'a str,
    pub zluda_version: &'a str,
    pub device: &'a str,
    pub flags: i64,
    pub binary: &'a [u8],
    pub last_access: i64,
}