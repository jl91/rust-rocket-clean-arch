use shaku::Interface;
use crate::domain::entities::UserDomainEntity;


pub trait UserDomainRepository: Interface {
    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> Vec<UserDomainEntity>;
}