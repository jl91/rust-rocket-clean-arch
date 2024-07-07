use std::error::Error;
use uuid::Uuid;
use crate::domain::entities::{NewUserDomainEntity, UpdateUserDomainEntity, UserDomainEntity};


pub trait UserDomainRepository {
    fn find_all(&self, size: Option<u64>, page: Option<u64>) -> Result<Vec<UserDomainEntity>, Box<dyn Error>>;

    fn find_by_id(&self, id: Uuid) -> Result<UserDomainEntity, Box<dyn Error>>;

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> Result<UserDomainEntity, Box<dyn Error>>;

    fn update_user(&self, user_domain_entity: UpdateUserDomainEntity) -> Result<UserDomainEntity, Box<dyn Error>>;

    fn soft_delete(&self, id: Uuid) -> Result<bool, Box<dyn Error>>;
}


pub trait Logger {
    fn info(&self, object: String, method: String, line: u32, message: String);
    fn error(&self, object: String, method: String, line: u32, message: String);
    fn warn(&self, object: String, method: String, line: u32, message: String);
    fn debug(&self, object: String, method: String, line: u32, message: String);
}