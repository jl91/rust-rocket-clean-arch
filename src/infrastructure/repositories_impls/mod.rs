use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use json_log::JsonLogger;
use rocket::yansi::Paint;
use uuid::Uuid;
use crate::application::mappers::from_user_database_entity_to_domain;
use crate::domain::entities::{NewUserDomainEntity, UpdateUserDomainEntity, UserDomainEntity};
use crate::domain::shared::repositories::{Logger, UserDomainRepository};
use crate::infrastructure::database::entities::UserDatabaseEntity;
use crate::infrastructure::database::repositories::{DatabaseRepository, UserDatabaseRepository};
use crate::infrastructure::logger::{LogMetadata, LogStruct};

pub struct UserDomainRepositoryImpl {
    user_database_repository: Arc<UserDatabaseRepository>,
}

impl UserDomainRepositoryImpl {
    pub fn new(user_database_repository: Arc<UserDatabaseRepository>) -> Self {
        Self {
            user_database_repository,
        }
    }
}

impl UserDomainRepository for UserDomainRepositoryImpl {
    fn find_all(&self, size: Option<i64>, page: Option<i64>) -> Vec<UserDomainEntity> {
        self.user_database_repository
            .find_all(
                size,
                page,
            )
            .map(from_user_database_entity_to_domain)
            .collect()
    }

    fn find_by_id(&self, id: Uuid) -> UserDomainEntity {
        from_user_database_entity_to_domain(
            self.user_database_repository
                .find_one_by_id(id)
                .unwrap()
        )
    }

    fn create_user(&self, user_domain_entity: NewUserDomainEntity) -> UserDomainEntity {
        from_user_database_entity_to_domain(
            self.user_database_repository
                .create(
                    UserDatabaseEntity {
                        username: user_domain_entity.username,
                        password: user_domain_entity.password,
                        ..Default::default()
                    }
                )
                .unwrap()
        )
    }

    fn update_user(&self, user_domain_entity: UpdateUserDomainEntity) -> UserDomainEntity {
        from_user_database_entity_to_domain(
            self.user_database_repository
                .update(
                    UserDatabaseEntity {
                        external_id: user_domain_entity.id,
                        username: user_domain_entity.username,
                        password: user_domain_entity.password,
                        ..Default::default()
                    }
                )
                .unwrap()
        )
    }

    fn soft_delete(&self, id: Uuid) -> bool {
        self.user_database_repository
            .soft_delete(id)
    }
}

pub struct DefaultLogger<'a>{
    logger: &'a  JsonLogger
}


impl<'a> DefaultLogger<'a> {
    pub fn new(
        logger: &'a JsonLogger
    ) -> Self {
        Self{
            logger
        }
    }

    fn fabricate_log(message: &str) -> LogStruct {
        LogStruct {
            time: Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
            message: String::from(message),
            metadata: LogMetadata {
                correlation_id: Uuid::new_v4().to_string(),
                trace_id: Uuid::new_v4().to_string(),
                method: String::from("test"),
                headers: HashMap::new(),
            },
        }
    }
}
impl Logger for DefaultLogger<'_> {
    fn info(&self, message: &str) {
        self.logger.info(
            Self::fabricate_log(message)
        )
    }

    fn error(&self, message: &str) {
        let logger = json_log::get_default_logger();

        self.logger.error(
            Self::fabricate_log(message)
        )
    }

    fn warn(&self, message: &str) {
        let logger = json_log::get_default_logger();

        self.logger.warn(
            Self::fabricate_log(message)
        )
    }

    fn debug(&self, message: &str) {
        let logger = json_log::get_default_logger();

        self.logger.debug(
            Self::fabricate_log(message)
        )
    }
}