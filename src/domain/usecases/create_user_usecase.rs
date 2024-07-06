use std::sync::Arc;
use crate::domain::entities::{NewUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::domain::shared::UsecaseSpecification;

pub struct CreateUserUsecase {
    logger: Arc<dyn Logger>,
    user_domain_repository: Arc<dyn UserDomainRepository>
}

impl CreateUserUsecase {
    pub fn new(
        logger: Arc<dyn Logger>,
        user_domain_repository: Arc<dyn UserDomainRepository>,
    ) -> Self {
        Self {
            logger,
            user_domain_repository,
        }
    }
}

impl UsecaseSpecification<NewUserDomainEntity, Result<UserDomainEntity, ()>> for CreateUserUsecase {
    fn execute(
        &self,
        user_domain_entity: NewUserDomainEntity,
    ) -> Result<UserDomainEntity, ()> {
        self.logger.info("iniciando a execução do método execute do CreateUserUsecase");
        Ok(
            self.user_domain_repository
                .create_user(user_domain_entity)
        )
    }
}