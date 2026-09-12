use std::fmt::Display;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Critical => {
                write!(f, "CRIT")
            }
            Priority::High => {
                write!(f, "HIGH")
            }
            Priority::Medium => {
                write!(f, "MED")
            }
            Priority::Low => {
                write!(f, "LOW")
            }
        }
    }
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    mpsc::channel()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    thread::spawn(move || {
        for mut message in messages {
            if message.content.contains("ERROR") {
                message.priority = Priority::Critical;
            } else if message.content.contains("WARNING") {
                message.priority = Priority::High;
            } else if message.content.contains("DEBUG") {
                message.priority = Priority::Medium;
            } else {
                message.priority = Priority::Low;
            }

            tx.send(message).unwrap();
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    thread::spawn(move || {
        let mut result = Vec::new();

        for message in rx {
            result.push(format!(
                "[{}|{}] {}",
                message.priority, message.sender_id, message.content
            ));
        }

        result
    })
}

pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
