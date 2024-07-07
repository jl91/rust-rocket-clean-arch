use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct EmailDomainEntity {
    email: String,
    created_at: DateTime<Utc>,
}

pub struct PhoneDomainEntity {
    phone: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserDomainEntity {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UpdateUserDomainEntity {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDomainEntity {
    pub id: String,
    pub username: String,
    pub password: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct GenericQueryDomainEntity {
    pub page: Option<u64>,
    pub size: Option<u64>
}