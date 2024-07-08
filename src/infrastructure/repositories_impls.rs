use std::error::Error;
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
    fn find_all(&self, size: Option<u64>, page: Option<u64>) -> Result<Vec<UserDomainEntity>, Box<dyn Error>> {
        let result = self.user_database_repository
            .find_all(
                size,
                page,
            )?;

        Ok(
            result
                .into_iter()
                .map(from_user_database_entity_to_domain)
                .collect()
        )
    }

    fn find_by_id(&self, id: Uuid) -> Result<UserDomainEntity, Box<dyn Error>> {
        Ok(
            from_user_database_entity_to_domain(
                self.user_database_repository
                    .find_one_by_id(id)?
            )
        )
    }

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> Result<UserDomainEntity, Box<dyn Error>> {
        Ok(
            from_user_database_entity_to_domain(
                self.user_database_repository
                    .create(
                        UserDatabaseEntity {
                            username: user_domain_entity.username,
                            password: user_domain_entity.password,
                            ..Default::default()
                        }
                    )?
            )
        )
    }

    fn update_user(&self, user_domain_entity: UpdateUserDomainEntity) -> Result<UserDomainEntity, Box<dyn Error>> {
        Ok(
            from_user_database_entity_to_domain(
                self.user_database_repository
                    .update(
                        UserDatabaseEntity {
                            external_id: user_domain_entity.id,
                            username: user_domain_entity.username,
                            password: user_domain_entity.password,
                            ..Default::default()
                        }
                    )?
            )
        )
    }

    fn soft_delete(&self, id: Uuid) -> Result<bool, Box<dyn Error>> {
        let result = self.user_database_repository
            .soft_delete(id)?;
        Ok(
            result > 0
        )
    }
}


