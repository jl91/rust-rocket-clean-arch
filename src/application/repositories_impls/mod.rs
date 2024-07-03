use std::sync::Arc;
use uuid::Uuid;
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

    fn find_by_id(&self, id: Uuid) -> UserDomainEntity {
        from_user_database_entity_to_domain(
            self.user_database_repository
                .find_one_by_id(id)
                .unwrap()
        )
    }

    // fn create_user(&self, user_domain_entity: UserDomainEntity) -> UserDomainEntity {
    //
    //     self.user_database_repository
    //         .create(
    //             from_user_database_entity_to_domain(user_domain_entity)
    //         )
    //         .unwrap()
    // }


}