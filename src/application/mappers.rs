use crate::application::entrypoints::rest::UserRequestDTO;
use crate::domain::entities::UserDomainEntity;
use crate::infrastructure::database::entities::UserDatabaseEntity;

pub fn from_user_database_entity_to_domain(entity: UserDomainEntity) -> UserDomainEntity {
    UserDomainEntity {
        id: entity.external_id.to_string(),
        username: entity.username,
        password: entity.password,
        created_at: Option::from(entity.created_at.format("%Y-%m-%d").to_string()),
        updated_at: match entity.updated_at {
            Some(date) => Option::from(date.format("%Y-%m-%d").to_string()),
            None => Option::from(String::from("")),
        },
    }
}

pub fn from_user_dtoto_domain_entity(dto: UserRequestDTO) -> UserDomainEntity {
    UserDomainEntity {
        id: None,
        username: dto.username,
        password: dto.password,
        created_at: None,
        updated_at: None,
    }
}

pub fn from_domain_entity_to_database_entity(entity: UserDomainEntity) -> UserDatabaseEntity {
    UserDatabaseEntity {
        id: 0,
        external_id: ,
        username: entity.username,
        password: entity.password,
        created_at: chrono::NaiveDateTime::parse_from_str(entity.created_at.unwrap().as_str(), "%Y-%m-%d").unwrap(),
        updated_at: match entity.updated_at {
            Some(date) => Option::from(chrono::NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d").unwrap()),
            None => None,
        },
        deleted_at: None,
    }
}