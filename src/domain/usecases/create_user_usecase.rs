use std::sync::Arc;
use crate::domain::entities::{NewUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::UserDomainRepository;
use crate::domain::shared::UsecaseSpecification;

pub struct CreateUserUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl CreateUserUsecase {
    pub fn new(user_domain_repository: Arc<dyn UserDomainRepository>) -> Self {
        Self {
            user_domain_repository
        }
    }
}

impl UsecaseSpecification<NewUserDomainEntity, Result<UserDomainEntity, ()>> for CreateUserUsecase {
    fn execute(
        &self,
        user_domain_entity: NewUserDomainEntity,
    ) -> Result<UserDomainEntity, ()> {
        Ok(
            self.user_domain_repository
                .create_user(user_domain_entity)
        )
    }
}