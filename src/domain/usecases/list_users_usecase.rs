use std::sync::{Arc};
use crate::domain::shared::UsecaseSpecification;
use crate::domain::entities::{GenericQueryDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::UserDomainRepository;

pub struct ListUsersUsecase {
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl ListUsersUsecase {
    pub fn new(user_domain_repository: Arc<dyn UserDomainRepository>) -> Self {
        Self {
            user_domain_repository
        }
    }
}

impl UsecaseSpecification<GenericQueryDomainEntity, Result<Vec<UserDomainEntity>, ()>> for ListUsersUsecase {
    fn execute(
        &self,
        query_request_domain_entity: GenericQueryDomainEntity,
    ) -> Result<Vec<UserDomainEntity>, ()> {
        Ok(
            self.user_domain_repository
                .find_all(
                    query_request_domain_entity.size,
                    query_request_domain_entity.page,
                )
        )
    }
}