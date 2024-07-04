use std::sync::Arc;
use crate::domain::entities::{UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::UserDomainRepository;
use crate::domain::shared::UsecaseSpecification;

pub struct UpdateUserUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl UpdateUserUsecase {
    pub fn new(user_domain_repository: Arc<dyn UserDomainRepository>) -> Self {
        Self {
            user_domain_repository,
        }
    }
}

impl UsecaseSpecification<UpdateUserDomainEntity, Result<UserDomainEntity, ()>> for UpdateUserUsecase {
    fn execute(
        &self,
        update_user_domain_entity: UpdateUserDomainEntity
    ) -> Result<UserDomainEntity, ()> {
        Ok(
            self.user_domain_repository
                .update_user(update_user_domain_entity)
        )
    }
}
