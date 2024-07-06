use std::sync::Arc;
use uuid::Uuid;
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::domain::shared::UsecaseSpecification;

pub struct DeleteUserUsecase {
    logger: Arc<dyn Logger>,
    user_domain_repository: Arc<dyn UserDomainRepository>
}

impl DeleteUserUsecase {
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
