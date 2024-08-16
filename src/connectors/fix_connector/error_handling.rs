pub async fn handle_message_rejection(reason: &str) -> Result<(), String> {
    match reason {
        "Session Not Active" => Err("Cannot send message: session not active".to_string()),
        "Sequence Number Mismatch" => Err("Sequence number mismatch: resending message".to_string()),
        _ => Ok(()),
    }
}

pub async fn check_sequence_number(expected: u64, received: u64) -> Result<(), String> {
    if received != expected {
        // Handle sequence mismatch
        Err(format!(
            "Sequence mismatch: expected {}, but received {}",
            expected, received
        ))
    } else {
        Ok(())
    }
}
