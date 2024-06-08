pub struct ListUsersUsecase;

use rocket::yansi::Paint;
use uuid::Uuid;
use crate::domain::shared::UsecaseSpecification;
use crate::domain::entities::{QueryRequestDomainEntity, UserDomainEntity};
use crate::infrastructure::database::entities::User;
use crate::infrastructure::database::repositories::{Repository, UserRepository};

impl UsecaseSpecification<QueryRequestDomainEntity, Vec<UserDomainEntity>> for ListUsersUsecase {
    fn execute(
        &self,
        query_request_domain_entity: QueryRequestDomainEntity
    ) -> Vec<UserDomainEntity> {

        let mut user_repository = <UserRepository as Repository<User, Uuid>>::new();
        // let size = Option.new();



       user_repository
           .find_all(
               query_request_domain_entity.size,
               query_request_domain_entity.page
           )
            .map(|entity| UserDomainEntity {
                id: entity.external_id.to_string(),
                username: entity.username,
                password: entity.password,
                created_at: entity.created_at.format("%Y-%m-%d").to_string(),
                updated_at: match entity.updated_at {
                    Some(date) => date.format("%Y-%m-%d").to_string(),
                    None => String::from(""), // String "null" ou outro valor padrão
                },
            })
            .collect()
    }
}