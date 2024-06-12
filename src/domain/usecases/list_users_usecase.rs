use crate::domain::shared::UsecaseSpecification;
use crate::domain::entities::{QueryRequestDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::UserDomainRepository;

pub struct ListUsersUsecase {
    user_domain_repository: Box<dyn UserDomainRepository>,
}

impl ListUsersUsecase {
    pub fn new(user_domain_repository: Box<dyn UserDomainRepository>) -> ListUsersUsecase {
        ListUsersUsecase { user_domain_repository }
    }
}

impl UsecaseSpecification<QueryRequestDomainEntity, Vec<UserDomainEntity>> for ListUsersUsecase {
    fn execute(
        &self,
        query_request_domain_entity: QueryRequestDomainEntity,
    ) -> Vec<UserDomainEntity> {
        self.user_domain_repository.find_all(
            query_request_domain_entity.size,
            query_request_domain_entity.page,
        )
    }
}