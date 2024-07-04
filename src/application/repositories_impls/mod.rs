use std::sync::Arc;
use uuid::Uuid;
use crate::application::mappers::from_user_database_entity_to_domain;
use crate::domain::entities::{NewUserDomainEntity, UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::UserDomainRepository;
use crate::infrastructure::database::entities::UserDatabaseEntity;
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

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> UserDomainEntity {
        from_user_database_entity_to_domain(
            self.user_database_repository
                .create(
                    UserDatabaseEntity {
                        username: user_domain_entity.username,
                        password: user_domain_entity.password,
                        ..Default::default()
                    }
                )
                .unwrap()
        )
    }

    fn update_user(&self, user_domain_entity: UpdateUserDomainEntity) -> UserDomainEntity {
        from_user_database_entity_to_domain(
            self.user_database_repository
                .update(
                    UserDatabaseEntity {
                        external_id: user_domain_entity.id,
                        username: user_domain_entity.username,
                        password: user_domain_entity.password,
                        ..Default::default()
                    }
                )
                .unwrap()
        )
    }

    fn soft_delete(&self, id: Uuid) -> bool {
        self.user_database_repository
            .soft_delete(id)
    }
}