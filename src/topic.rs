use serde::{Deserialize, Serialize};
use surf::http::Method;

use crate::Client;
use crate::Error;
use crate::presenters::PublishMessage;

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub(crate) client: crate::Client,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicMessageResponse {
    pub message_ids: Vec<String>,
}

impl Topic {
    pub fn new(client: Client, name: &str) -> Self {
        Self {
            name: format!("projects/{}/topics/{}", client.project(), name),
            client,
        }
    }

    pub async fn publish<T: Serialize>(&self, data: T) -> Result<TopicMessageResponse, Error> {
        let url = format!("https://pubsub.googleapis.com/v1/{}:publish", self.name);
        let payload = PublishMessage::from(&data);
        let request_builder = self
            .client
            .base_request(Method::Post, &url)?
            .body_json(&payload)?;
        let mut response = request_builder.send().await?;
        if response.status().is_success() {
            let response: TopicMessageResponse = response.body_json().await?;
            Ok(response)
        } else {
            response
                .body_string()
                .await
                .map_err(|err| Error::Unexpected(format!("{}", err)))
                .and_then(|json| Err(Error::PubSub(json)))
        }
    }
}
