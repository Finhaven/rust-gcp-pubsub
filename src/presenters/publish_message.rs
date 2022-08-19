use crate::presenters::EncodedMessage;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishMessage {
  pub messages: Vec<EncodedMessage>,
}

impl PublishMessage {
  pub fn from<T: serde::Serialize>(data: &T, ordering_key: String) -> Self {
    Self {
      messages: vec![EncodedMessage::new(data, ordering_key)],
    }
  }
}
