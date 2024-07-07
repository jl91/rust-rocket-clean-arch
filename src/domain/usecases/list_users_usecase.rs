use std::any::type_name;
use std::error::Error;
use std::sync::{Arc};
use crate::domain::shared::UsecaseSpecification;
use crate::domain::entities::{GenericQueryDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::{Logger, UserDomainRepository};

pub struct ListUsersUsecase {
    logger: Arc<dyn Logger>,
    user_domain_repository: Arc<dyn UserDomainRepository>,
}

impl ListUsersUsecase {
    pub fn new(
        logger: Arc<dyn Logger>,
        user_domain_repository: Arc<dyn UserDomainRepository>,
    ) -> Self {
        Self {
            logger,
            user_domain_repository,
        }
    }
}

impl UsecaseSpecification<GenericQueryDomainEntity, Result<Vec<UserDomainEntity>, Box<dyn Error>>> for ListUsersUsecase {
    fn execute(
        &self,
        query_request_domain_entity: GenericQueryDomainEntity,
    ) -> Result<Vec<UserDomainEntity>, Box<dyn Error>> {
        self.logger.info(
            type_name::<ListUsersUsecase>().to_string(),
            "execute".to_string(),
            line!(),
            "Iniciando listagem de usuários.".to_string(),
        );

        self.user_domain_repository
            .find_all(
                query_request_domain_entity.size,
                query_request_domain_entity.page,
            )
    }
}