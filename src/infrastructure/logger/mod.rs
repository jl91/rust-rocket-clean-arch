use std::collections::HashMap;
use log::Level;
use serde::Serialize;

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