use crate::application::mappers::fromUserDatabaseEntityToDomain;
use crate::domain::entities::UserDomainEntity;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::infrastructure::database::repositories::{DatabaseRepository, UserDatabaseRepository};

pub struct UserDomainRepositoryImpl {
    user_database_repository: UserDatabaseRepository,
}

impl UserDomainRepositoryImpl {
    pub fn new(
        user_database_repository: UserDatabaseRepository
    ) -> UserDomainRepositoryImpl {
        UserDomainRepositoryImpl {
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
            .map(fromUserDatabaseEntityToDomain)
            .collect()
    }
}