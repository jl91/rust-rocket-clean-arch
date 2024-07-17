mod rest {}

pub mod users_requests_handlers;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse<T> {
    pub data: T,
}