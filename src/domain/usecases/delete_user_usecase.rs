use std::sync::Arc;
use uuid::Uuid;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::domain::shared::UsecaseSpecification;

pub struct DeleteUserUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl DeleteUserUsecase {
    pub fn new(user_domain_repository: Arc<dyn UserDomainRepository>) -> Self {
        Self {
            user_domain_repository,
        }
    }
}

impl UsecaseSpecification<Uuid, Result<bool, ()>> for DeleteUserUsecase {
    fn execute(
        &self,
        id: Uuid,
    ) -> Result<bool, ()> {
        Ok(
            self.user_domain_repository
                .soft_delete(id)
        )
    }
}
