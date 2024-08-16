use secrecy::{ExposeSecret, Secret};
use crate::connectors::fix_connector::sequence::SequenceManager;

pub struct FixSession {
    pub session_id: Secret<String>,
    pub sequence_manager: SequenceManager,
}

impl FixSession {
    pub fn new(session_id: String, start_seq_num: u64) -> Self {
        FixSession {
            session_id: Secret::new(session_id),
            sequence_manager: SequenceManager::new(start_seq_num),
        }
    }

    pub fn get_session_id(&self) -> &str {
        self.session_id.expose_secret()
    }
}
