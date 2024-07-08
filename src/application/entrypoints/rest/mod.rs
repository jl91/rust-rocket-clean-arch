mod rest {}

pub mod users_requests_handlers;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewUserRequestDTO {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequestDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse<T> {
    pub data: T,
}