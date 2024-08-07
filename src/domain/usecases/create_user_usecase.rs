use std::any::type_name;
use std::error::Error;
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

impl UsecaseSpecification<NewUserDomainEntity, Result<UserDomainEntity, Box<dyn Error>>> for CreateUserUsecase {
    fn execute(
        &self,
        user_domain_entity: NewUserDomainEntity,
    ) -> Result<UserDomainEntity, Box<dyn Error>> {
        self.logger.info(
            type_name::<CreateUserUsecase>().to_string(),
            "execute".to_string(),
            line!(),
            "iniciando a execução do método execute do CreateUserUsecase".to_string()
        );

        Ok(
            self.user_domain_repository
                .create_user(user_domain_entity)?
        )
    }
}