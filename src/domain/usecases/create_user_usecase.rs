use std::sync::Arc;
use crate::domain::shared::repositories::UserDomainRepository;

pub struct CreateUserUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl CreateUserUsecase {
    pub fn new(user_domain_repository: Arc<dyn UserDomainRepository>) -> Self {
        Self {
            user_domain_repository
        }
    }
}

// impl UsecaseSpecification<UserDomainEntity, Result<UserDomainEntity, ()>> for CreateUserUsecase {
//     fn execute(
//         &self,
//         user_domain_entity: UserDomainEntity,
//     ) -> Result<UserDomainEntity, ()> {
//         Ok(
//             self.user_domain_repository
//                 .create_user(user_domain_entity)
//         )
//     }
// }