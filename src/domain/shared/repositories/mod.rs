use uuid::Uuid;
use crate::domain::entities::{NewUserDomainEntity, UserDomainEntity};


pub trait UserDomainRepository {
    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> Vec<UserDomainEntity>;

    fn find_by_id(&self, id: Uuid) -> UserDomainEntity;

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> UserDomainEntity;

}