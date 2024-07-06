use std::sync::Arc;
use crate::domain::entities::{UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::domain::shared::UsecaseSpecification;

pub struct UpdateUserUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
    logger: Arc<dyn Logger>
}

impl UpdateUserUsecase {
    pub fn new(
        user_domain_repository: Arc<dyn UserDomainRepository>,
        logger: Arc<dyn Logger>
    ) -> Self {
        Self {
            user_domain_repository,
            logger
        }
    }
}

impl UsecaseSpecification<UpdateUserDomainEntity, Result<UserDomainEntity, ()>> for UpdateUserUsecase {
    fn execute(
        &self,
        update_user_domain_entity: UpdateUserDomainEntity
    ) -> Result<UserDomainEntity, ()> {
        self.logger.info("iniciando a execução do método execute do UpdateUserUsecase");
        Ok(
            self.user_domain_repository
                .update_user(update_user_domain_entity)
        )
    }
}
