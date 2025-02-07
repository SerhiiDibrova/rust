use log::{info, error};
use crate::message_queue::{is_queue_empty, dequeue_message};
use crate::message_handlers::{handle_trade_message, handle_status_update, handle_error_message};

pub fn process_messages_from_queue() {
    let mut messages_processed = 0;

    if is_queue_empty() {
        info!("No messages to process");
        return;
    }

    while !is_queue_empty() {
        let message = dequeue_message();
        match message.message_type {
            MessageType::Trade => {
                if validate_trade_message(&message) {
                    handle_trade_message(message);
                } else {
                    error!("Invalid trade message: {:?}", message);
                }
            }
            MessageType::StatusUpdate => {
                if validate_status_update_message(&message) {
                    handle_status_update(message);
                } else {
                    error!("Invalid status update message: {:?}", message);
                }
            }
            MessageType::Error => {
                if validate_error_message(&message) {
                    handle_error_message(message);
                } else {
                    error!("Invalid error message: {:?}", message);
                }
            }
        }
        messages_processed += 1;
    }

    info!("Total messages processed: {}", messages_processed);
}

fn validate_trade_message(message: &Message) -> bool {
    // Validation logic for trade message
}

fn validate_status_update_message(message: &Message) -> bool {
    // Validation logic for status update message
}

fn validate_error_message(message: &Message) -> bool {
    // Validation logic for error message
}