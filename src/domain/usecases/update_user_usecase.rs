use std::any::type_name;
use std::sync::Arc;
use crate::domain::entities::{UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::domain::shared::UsecaseSpecification;

pub struct UpdateUserUsecase {
    logger: Arc<dyn Logger>,
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl UpdateUserUsecase {
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

impl UsecaseSpecification<UpdateUserDomainEntity, Result<UserDomainEntity, ()>> for UpdateUserUsecase {
    fn execute(
        &self,
        update_user_domain_entity: UpdateUserDomainEntity,
    ) -> Result<UserDomainEntity, ()> {
        self.logger.info(
            type_name::<UpdateUserUsecase>().to_string(),
            "execute".to_string(),
            line!(),
            "iniciando a execução do método execute do UpdateUserUsecase".to_string(),
        );
        Ok(
            self.user_domain_repository
                .update_user(update_user_domain_entity)
        )
    }
}
