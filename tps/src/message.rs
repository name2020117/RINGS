use chrono::{offset::Utc, serde::ts_nanoseconds_option, DateTime, Duration};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_number: u32,
    pub sender_ip: String,
    pub broker_ip: String,
    pub receiver_ip: String,
    topic: String,
    #[serde(with = "ts_nanoseconds_option")]
    pub send_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_nanoseconds_option")]
    pub receive_time: Option<DateTime<Utc>>,
}

impl Message {
    pub fn new(
        message_number: u32,
        sender_ip: String,
        broker_ip: String,
        topic: String,
        send_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            message_number,
            sender_ip,
            send_time,
            receive_time: None,
            topic,
            broker_ip,
            receiver_ip: String::new(),
        }
    }

    pub fn new_from_string(data: &str) -> Result<Self> {
        serde_json::from_str(data)
    }

    pub fn to_string(&self) -> Result<String> {
        serde_json::to_string(&self)
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.receive_time, self.send_time) {
            (None, Some(_)) | (Some(_), None) | (None, None) => None,
            (Some(x), Some(y)) => Some(x - y),
        }
    }

    pub fn add_recieve_time(&mut self, recieve_time: DateTime<Utc>) {
        self.receive_time = Some(recieve_time);
    }

    pub fn add_reciever_ip(&mut self, reciever_ip: String) {
        self.receiver_ip = reciever_ip;
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Message {} sent at {} from {} -> {}.\nCollected message at {}",
            self.message_number,
            self.send_time.unwrap(),
            self.sender_ip,
            self.broker_ip,
            self.receiver_ip
        )
    }
}
