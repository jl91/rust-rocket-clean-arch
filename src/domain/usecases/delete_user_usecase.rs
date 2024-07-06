use std::sync::Arc;
use uuid::Uuid;
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::domain::shared::UsecaseSpecification;

pub struct DeleteUserUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
    logger: Arc<dyn Logger>
}

impl DeleteUserUsecase {
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

impl UsecaseSpecification<Uuid, Result<bool, ()>> for DeleteUserUsecase {
    fn execute(
        &self,
        id: Uuid,
    ) -> Result<bool, ()> {
        self.logger.info("Iniciando a exclusão de um usuário");
        Ok(
            self.user_domain_repository
                .soft_delete(id)
        )
    }
}
