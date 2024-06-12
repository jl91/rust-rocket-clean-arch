use chrono::NaiveDateTime;
use crate::infrastructure::database::schemas::{users, emails, phones};
use diesel::{Queryable, Insertable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserDatabaseEntity {
    pub id: i64,
    pub external_id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = emails)]
pub struct EmailDatabaseEntity {
    pub id: i64,
    pub user_id: i64,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = phones)]
pub struct PhoneDatabaseEntity {
    pub id: i64,
    pub user_id: i64,
    pub phone: String,
    pub created_at: NaiveDateTime,
}
