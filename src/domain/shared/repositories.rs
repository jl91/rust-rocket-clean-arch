use uuid::Uuid;
use crate::domain::entities::{NewUserDomainEntity, UpdateUserDomainEntity, UserDomainEntity};


pub trait UserDomainRepository {
    fn find_all(&self, size: Option<u64>, page: Option<u64>) -> Vec<UserDomainEntity>;

    fn find_by_id(&self, id: Uuid) -> UserDomainEntity;

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> UserDomainEntity;

    fn update_user(&self, user_domain_entity: UpdateUserDomainEntity) -> UserDomainEntity;

    fn soft_delete(&self, id: Uuid) -> bool;
}


pub trait Logger {
    fn info(&self, object: String, method: String, line: u32, message: String);
    fn error(&self, object: String, method: String, line: u32, message: String);
    fn warn(&self, object: String, method: String, line: u32, message: String);
    fn debug(&self, object: String, method: String, line: u32, message: String);
}