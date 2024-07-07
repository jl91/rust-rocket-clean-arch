use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use log::Level;
use rocket::yansi::Paint;
use serde_json::json;
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
    fn find_all(&self, size: Option<u64>, page: Option<u64>) -> Vec<UserDomainEntity> {
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

pub struct DefaultLogger {
    logger: JsonLogger,
}


impl DefaultLogger {
    pub fn new(
        logger: JsonLogger
    ) -> Self {
        Self {
            logger
        }
    }

    fn fabricate_log(
        level: Level,
        object: String,
        method: String,
        line: u32,
        message: String
    ) -> LogStruct {
        LogStruct {
            _time: Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: level.to_string(),
            message,
            metadata: LogMetadata {
                correlation_id: Uuid::new_v4().to_string(),
                trace_id: Uuid::new_v4().to_string(),
                headers: HashMap::new(),
                object,
                method,
                line,
            },
        }
    }
}
impl Logger for DefaultLogger{
    fn info(&self, object: String, method: String, line: u32, message: String) {
        self.logger.info(
            Self::fabricate_log(Level::Info, object, method, line, message)
        )
    }

    fn error(&self, object: String, method: String, line: u32, message: String) {
        self.logger.error(
            Self::fabricate_log(Level::Error, object, method, line, message)
        )
    }

    fn warn(&self, object: String, method: String, line: u32, message: String) {
        self.logger.warn(
            Self::fabricate_log(Level::Warn, object, method, line, message)
        )
    }

    fn debug(&self, object: String, method: String, line: u32, message: String) {
        self.logger.debug(
            Self::fabricate_log(Level::Debug, object, method, line, message)
        )
    }
}

pub struct JsonLogger;


impl JsonLogger {

    pub(crate) fn new() -> Self {
        Self
    }

    fn info(&self, log: LogStruct) {
        info!("{}", json!(log));
    }

    fn error(&self, log: LogStruct) {
        error!("{}", json!(log));
    }

    fn warn(&self, log: LogStruct) {
        warn!("{}", json!(log));
    }

    fn debug(&self, log: LogStruct) {
        debug!("{}", json!(log));
    }
}