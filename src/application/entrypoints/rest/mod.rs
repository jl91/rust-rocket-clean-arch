mod rest {}

pub mod users_requests_handlers;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserRequestDTO {
    pub username: String,
    pub password: String,
}