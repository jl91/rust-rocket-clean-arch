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

 impl UsecaseSpecification<Uuid, Result<UserDomainEntity, ()>> for OneUsersUsecase {
     fn execute(
         &self,
         id: Uuid,
     ) -> Result<UserDomainEntity, ()> {
            self.logger.info("iniciando a execução do método execute do OneUsersUsecase");
         Ok(
             self.user_domain_repository
                 .find_by_id(id)
         )
     }
 }
