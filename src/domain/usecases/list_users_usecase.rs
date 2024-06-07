pub struct ListUsersUsecase;

use diesel::RunQueryDsl;
use crate::domain::shared::UsecaseSpecification;
use crate::domain::entities::UserDomainEntity;
use crate::infrastructure::database::connection::get_connection;
use crate::infrastructure::database::schemas::users;
use crate::infrastructure::database::entities::User;

impl UsecaseSpecification<UserDomainEntity, Vec<UserDomainEntity>> for ListUsersUsecase {
    fn execute(&self, _: UserDomainEntity) -> Vec<UserDomainEntity> {

        let connection =&mut get_connection();

        let mut result = users::table
            .load::<User>(connection)
            .expect("Error loading users");

       result
            .into_iter()
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