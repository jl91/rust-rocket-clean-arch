use std::sync::Arc;
use shaku::Component;
use crate::application::mappers::fromUserDatabaseEntityToDomain;
use crate::domain::entities::UserDomainEntity;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::infrastructure::database::repositories::{DatabaseRepository, UserDatabaseRepository};


#[derive(Component)]
#[shaku(interface = UserDomainRepository)]
pub struct UserDomainRepositoryImpl {
    user_database_repository: Arc<UserDatabaseRepository>,
}

impl UserDomainRepository for UserDomainRepositoryImpl {
    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> Vec<UserDomainEntity> {
        self.user_database_repository
            .find_all(
                size,
                page,
            )
            .map(fromUserDatabaseEntityToDomain)
            .collect()
    }
}