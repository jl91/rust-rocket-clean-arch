use std::any::type_name;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::UserDomainEntity;
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::domain::shared::UsecaseSpecification;

pub struct OneUsersUsecase {
    logger: Arc<dyn Logger>,
    user_domain_repository: Arc<dyn UserDomainRepository>
}

impl OneUsersUsecase {
    pub fn new(
        logger: Arc<dyn Logger>,
        user_domain_repository: Arc<dyn UserDomainRepository>
    ) -> Self {
        Self {
            logger,
            user_domain_repository
        }
    }
}

 impl UsecaseSpecification<Uuid, Result<UserDomainEntity, Box<dyn Error>>> for OneUsersUsecase {
     fn execute(
         &self,
         id: Uuid,
     ) -> Result<UserDomainEntity, Box<dyn Error>> {
            self.logger.info(
                type_name::<OneUsersUsecase>().to_string(),
                "execute".to_string(),
                line!(),
                "iniciando a execução do método execute do OneUsersUsecase".to_string()
            );
         self.user_domain_repository
             .find_by_id(id)
     }
 }
