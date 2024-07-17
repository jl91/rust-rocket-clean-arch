use uuid::Uuid;
use crate::application::entrypoints::rest::users_requests_handlers::{NewUserRequestDTO, UpdateUserRequestDTO};
use crate::domain::entities::{NewUserDomainEntity, UpdateUserDomainEntity, UserDomainEntity};
use crate::infrastructure::database::entities::UserDatabaseEntity;

pub fn from_user_database_entity_to_domain(entity: UserDatabaseEntity) -> UserDomainEntity {
    UserDomainEntity {
        id: entity.external_id.to_string(),
        username: entity.username,
        password: entity.password,
        created_at: Option::from(entity.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
        updated_at: match entity.updated_at {
            Some(date) => Option::from(date.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Option::from(String::from("")),
        },
    }
}

pub fn new_user_from_dto_to_domain(dto: NewUserRequestDTO) -> NewUserDomainEntity {
    NewUserDomainEntity {
        username: dto.username,
        password: dto.password
    }
}


pub fn update_user_from_dto_to_domain(id: Uuid, dto: UpdateUserRequestDTO) -> UpdateUserDomainEntity {
    UpdateUserDomainEntity {
        id,
        username: dto.username,
        password: dto.password
    }
}