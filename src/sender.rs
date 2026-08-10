use async_trait::async_trait;
use mv64e_mtb_model::models::PatientRecord;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub type DynMtbFileSender = Arc<dyn MtbFileSender + Send + Sync>;

#[derive(Serialize, Deserialize)]
struct RecordKey {
    #[serde(rename = "pid")]
    patient_id: String,
}

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

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait MtbFileSender {
    async fn send(
        &self,
        mtb: PatientRecord,
        method: RequestMethod,
        request_id: Option<String>,
    ) -> Result<String, ()>;

    async fn send_empty(
        &self,
        method: RequestMethod,
        request_id: Option<String>,
    ) -> Result<String, ()>;
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
    async fn send(
        &self,
        mtb: PatientRecord,
        method: RequestMethod,
        request_id: Option<String>,
    ) -> Result<String, ()> {
        match serde_json::to_string(&mtb) {
            Ok(json) => {
                self.send_message(&json, &mtb.patient.id, method, request_id)
                    .await
            }
            Err(_) => Err(()),
        }
    }

    async fn send_empty(
        &self,
        method: RequestMethod,
        request_id: Option<String>,
    ) -> Result<String, ()> {
        self.send_message("{}", "", method, request_id).await
    }
}

impl DefaultMtbFileSender {
    async fn send_message(
        &self,
        payload: &str,
        patient_id: &str,
        method: RequestMethod,
        request_id: Option<String>,
    ) -> Result<String, ()> {
        let request_id = request_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let record_key = RecordKey {
            patient_id: patient_id.to_string(),
        };

        let record_headers = OwnedHeaders::default()
            .insert(Header {
                key: "requestId",
                value: Some(&request_id),
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

        self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .key(&record_key)
                    .headers(record_headers)
                    .payload(payload),
                Duration::from_secs(1),
            )
            .await
            .map_err(|_| ())
            .map(|_| ())?;
        Ok(request_id)
    }
}
