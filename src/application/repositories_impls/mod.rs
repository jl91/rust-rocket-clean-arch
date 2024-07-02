use std::sync::Arc;
use crate::application::mappers::from_user_database_entity_to_domain;
use crate::domain::entities::UserDomainEntity;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::infrastructure::database::repositories::{DatabaseRepository, UserDatabaseRepository};


pub struct UserDomainRepositoryImpl {
    user_database_repository: Arc<UserDatabaseRepository>,
}

impl UserDomainRepositoryImpl {
    pub fn new(user_database_repository: Arc<UserDatabaseRepository>) -> Self {
        Self {
            user_database_repository,
        }
    }
}

impl UserDomainRepository for UserDomainRepositoryImpl {
    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> Vec<UserDomainEntity> {
        self.user_database_repository
            .find_all(
                size,
                page,
            )
            .map(from_user_database_entity_to_domain)
            .collect()
    }

    fn create_user(&self, user_domain_entity: UserDomainEntity) -> UserDomainEntity {

        self.user_database_repository
            .create(
                from_user_database_entity_to_domain(user_domain_entity)
            )
            .unwrap()
    }
}