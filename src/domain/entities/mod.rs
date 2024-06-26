use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

pub struct EmailDomainEntity {
    email: String,
    created_at: DateTime<Utc>,
}

pub struct PhoneDomainEntity {
    phone: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDomainEntity {
    pub id: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Deserialize, Debug)]
pub struct GenericQueryDomainEntity {
    pub page: Option<i64>,
    pub size: Option<i64>
}