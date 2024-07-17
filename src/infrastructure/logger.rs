use std::collections::HashMap;
use serde::Serialize;
use chrono::Utc;
use log::Level;
use serde_json::json;
use uuid::Uuid;
use crate::domain::shared::repositories::Logger;

#[derive(Serialize, Debug)]
pub struct LogStruct {
    pub _time: String,
    pub level: String,
    pub message: String,
    pub metadata: LogMetadata,
}

#[derive(Serialize, Debug)]
pub struct LogMetadata {
    pub correlation_id: String,
    pub trace_id: String,
    pub headers: HashMap<String, String>,
    pub object: String,
    pub method: String,
    pub line: u32,
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
            _time: Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S.%3f").to_string(),
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