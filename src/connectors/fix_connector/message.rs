// src/connectors/fix_connector/message.rs

use std::collections::HashMap;

#[derive(Debug)]
pub struct FixMessage {
    pub msg_type: String,
    pub fields: HashMap<String, String>,
}

impl FixMessage {
    pub fn new(msg_type: &str, fields: HashMap<String, String>) -> Self {
        FixMessage {
            msg_type: msg_type.to_string(),
            fields,
        }
    }

    pub fn parse(raw_msg: &str) -> Option<Self> {
        let parts: Vec<&str> = raw_msg.split('|').collect();
        if parts.is_empty() {
            return None;
        }

        let mut fields = HashMap::new();
        let mut msg_type = String::new();

        for part in parts {
            let kv: Vec<&str> = part.split('=').collect();
            if kv.len() == 2 {
                fields.insert(kv[0].to_string(), kv[1].to_string());
                if kv[0] == "35" { // MsgType
                    msg_type = kv[1].to_string();
                }
            }
        }

        if msg_type.is_empty() {
            None
        } else {
            Some(FixMessage { msg_type, fields })
        }
    }

    pub fn to_string(&self) -> String {
        let mut msg = String::new();
        for (key, value) in &self.fields {
            msg.push_str(&format!("{}={}|", key, value));
        }
        msg
    }
}
