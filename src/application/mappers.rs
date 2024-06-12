use crate::domain::entities::UserDomainEntity;
use crate::infrastructure::database::entities::UserDatabaseEntity;

pub fn fromUserDatabaseEntityToDomain(entity: UserDatabaseEntity) -> UserDomainEntity {
    UserDomainEntity {
        id: entity.external_id.to_string(),
        username: entity.username,
        password: entity.password,
        created_at: entity.created_at.format("%Y-%m-%d").to_string(),
        updated_at: match entity.updated_at {
            Some(date) => date.format("%Y-%m-%d").to_string(),
            None => String::from(""),
        },
    }
}