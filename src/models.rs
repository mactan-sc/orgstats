use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewOrgStat {
    pub symbol: String,
    pub count: i32
}

#[derive(Debug, Queryable, Serialize)]
pub struct OrgStat {
    pub id: i64,
    pub symbol: String,
    pub count: i32,
    pub processdate: chrono::DateTime<chrono::Utc>,
}