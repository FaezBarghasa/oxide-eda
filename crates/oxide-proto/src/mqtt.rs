//! Embedded Local Offline MQTT Broker & Protocol Inspector.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// MQTT Quality of Service (QoS) Level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QosLevel {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

/// Recorded MQTT Message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MqttMessage {
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: QosLevel,
    pub retain: bool,
    pub timestamp_ms: u64,
}

impl MqttMessage {
    pub fn payload_as_str(&self) -> String {
        String::from_utf8_lossy(&self.payload).to_string()
    }
}

/// Lightweight In-Process Embedded MQTT Broker for offline simulation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmbeddedMqttBroker {
    /// Active subscriptions: topic_filter -> set of client_ids
    subscriptions: HashMap<String, HashSet<String>>,
    /// Retained messages by topic
    retained: HashMap<String, MqttMessage>,
    /// Chronological message log for the EDA Inspector panel
    pub message_log: Vec<MqttMessage>,
}

impl EmbeddedMqttBroker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Client subscribes to a topic pattern.
    pub fn subscribe(&mut self, client_id: impl Into<String>, topic_filter: impl Into<String>) {
        let client = client_id.into();
        let filter = topic_filter.into();
        self.subscriptions.entry(filter).or_default().insert(client);
    }

    /// Client unsubscribes from a topic pattern.
    pub fn unsubscribe(&mut self, client_id: &str, topic_filter: &str) {
        if let Some(clients) = self.subscriptions.get_mut(topic_filter) {
            clients.remove(client_id);
        }
    }

    /// Publishes a message through the broker.
    pub fn publish(&mut self, message: MqttMessage) -> Vec<String> {
        let mut recipients = HashSet::new();

        for (filter, clients) in &self.subscriptions {
            if topic_matches(filter, &message.topic) {
                for client in clients {
                    recipients.insert(client.clone());
                }
            }
        }

        if message.retain {
            self.retained.insert(message.topic.clone(), message.clone());
        }

        self.message_log.push(message);
        recipients.into_iter().collect()
    }

    /// Clears message log.
    pub fn clear_log(&mut self) {
        self.message_log.clear();
    }
}

/// Helper evaluating MQTT wildcard topic matching (`+` single level, `#` multi-level).
fn topic_matches(filter: &str, topic: &str) -> bool {
    if filter == "#" || filter == topic {
        return true;
    }

    let f_parts: Vec<&str> = filter.split('/').collect();
    let t_parts: Vec<&str> = topic.split('/').collect();

    let mut t_idx = 0;
    for (f_idx, &f) in f_parts.iter().enumerate() {
        if f == "#" {
            return true;
        }
        if t_idx >= t_parts.len() {
            return false;
        }
        if f == "+" || f == t_parts[t_idx] {
            t_idx += 1;
        } else {
            return false;
        }
        if f_idx == f_parts.len() - 1 && t_idx == t_parts.len() {
            return true;
        }
    }

    t_idx == t_parts.len()
}
