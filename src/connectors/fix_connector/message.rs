// src/connectors/fix_connector/message.rs

use std::collections::HashMap;

#[derive(Debug)]
pub struct FixMessage {
    pub msg_type: String,
    pub fields: std::collections::HashMap<String, String>,
}

impl FixMessage {
    pub fn new(msg_type: &str, fields: std::collections::HashMap<String, String>) -> Self {
        FixMessage {
            msg_type: msg_type.to_string(),
            fields,
        }
    }

    pub fn from_string(msg: String) -> Self {
        let mut fields = std::collections::HashMap::new();
        // Assume that the FIX message is in the format "8=FIX.4.2|35=D|49=ABC|..."
        for kv in msg.split('|') {
            if let Some((key, value)) = kv.split_once('=') {
                fields.insert(key.to_string(), value.to_string());
            }
        }
        let msg_type = fields.get("35").cloned().unwrap_or_else(|| "UNKNOWN".to_string());
        FixMessage {
            msg_type,
            fields,
        }
    }
}