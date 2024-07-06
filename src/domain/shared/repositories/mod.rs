use uuid::Uuid;
use crate::domain::entities::{NewUserDomainEntity, UpdateUserDomainEntity, UserDomainEntity};


pub trait UserDomainRepository {
    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> Vec<UserDomainEntity>;

    fn find_by_id(&self, id: Uuid) -> UserDomainEntity;

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> UserDomainEntity;

    fn update_user(&self, user_domain_entity: UpdateUserDomainEntity) -> UserDomainEntity;

    fn soft_delete(&self, id: Uuid) -> bool;

}


pub trait Logger {
    fn info(&self, message: &str);
    fn error(&self, message: &str);
    fn warn(&self, message: &str);
    fn debug(&self, message: &str);
}