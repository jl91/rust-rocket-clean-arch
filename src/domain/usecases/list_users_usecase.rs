use std::sync::{Arc, Mutex};
use crate::domain::shared::UsecaseSpecification;
use crate::domain::entities::{QueryRequestDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::UserDomainRepository;

#[derive(Clone)]
pub struct ListUsersUsecase {
    user_domain_repository: Arc<Mutex<dyn UserDomainRepository>>,
}

impl ListUsersUsecase {
    pub fn new(user_domain_repository: Arc<Mutex<dyn UserDomainRepository>>) -> Self {
        ListUsersUsecase { user_domain_repository }
    }
}

impl UsecaseSpecification<QueryRequestDomainEntity, Result<Vec<UserDomainEntity>, ()>> for ListUsersUsecase {
    fn execute(
        &self,
        query_request_domain_entity: QueryRequestDomainEntity,
    ) -> Result<Vec<UserDomainEntity>, ()> {
        Ok(
            self.user_domain_repository
                .lock()
                .unwrap()
                .find_all(
                    query_request_domain_entity.size,
                    query_request_domain_entity.page,
                )
        )
    }
}