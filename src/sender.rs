use async_trait::async_trait;
use mv64e_mtb_dto::Mtb;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::RecordKey;

pub type DynMtbFileSender = Arc<dyn MtbFileSender + Send + Sync>;

#[derive(PartialEq, Debug)]
pub enum RequestMethod {
    Post,
    Delete,
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestMethod::Post => write!(f, "POST"),
            RequestMethod::Delete => write!(f, "DELETE"),
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait MtbFileSender {
    async fn send(&self, mtb: Mtb, method: RequestMethod) -> Result<String, ()>;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct DefaultMtbFileSender {
    topic: String,
    producer: FutureProducer,
}

impl DefaultMtbFileSender {
    pub fn new(topic: &str, producer: FutureProducer) -> Self {
        Self {
            topic: topic.to_string(),
            producer,
        }
    }
}

#[async_trait]
impl MtbFileSender for DefaultMtbFileSender {
    async fn send(&self, mtb: Mtb, method: RequestMethod) -> Result<String, ()> {
        let request_id = Uuid::new_v4();

        let record_key = RecordKey {
            patient_id: mtb.patient.id.clone(),
        };

        let record_headers = OwnedHeaders::default()
            .insert(Header {
                key: "requestId",
                value: Some(&request_id.to_string()),
            })
            .insert(Header {
                key: "requestMethod",
                value: Some(&method.to_string()),
            })
            .insert(Header {
                key: "contentType",
                value: Some("application/vnd.dnpm.v2.mtb+json"),
            });

        let record_key = serde_json::to_string(&record_key).map_err(|_| ())?;

        match serde_json::to_string(&mtb) {
            Ok(json) => {
                self.producer
                    .send(
                        FutureRecord::to(&self.topic)
                            .key(&record_key)
                            .headers(record_headers)
                            .payload(&json),
                        Duration::from_secs(1),
                    )
                    .await
                    .map_err(|_| ())
                    .map(|_| ())?;
                Ok(request_id.to_string())
            }
            Err(_) => Err(()),
        }
    }
}

impl DefaultMtbFileSender {}
