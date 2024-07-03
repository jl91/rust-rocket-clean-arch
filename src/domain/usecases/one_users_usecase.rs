use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::UserDomainEntity;
use crate::domain::shared::repositories::UserDomainRepository;
use crate::domain::shared::UsecaseSpecification;

pub struct OneUsersUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl OneUsersUsecase {
    pub fn new(user_domain_repository: Arc<dyn UserDomainRepository>) -> Self {
        Self {
            user_domain_repository,
        }
    }
}

 impl UsecaseSpecification<Uuid, Result<UserDomainEntity, ()>> for OneUsersUsecase {
     fn execute(
         &self,
         id: Uuid,
     ) -> Result<UserDomainEntity, ()> {
         Ok(
             self.user_domain_repository
                 .find_by_id(id)
         )
     }
 }
