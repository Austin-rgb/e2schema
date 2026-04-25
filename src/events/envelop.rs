use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetaData {
    pub event_id: Uuid,
    pub event_version: String,
    pub occurred_at: DateTime<Utc>,
    pub producer: String,
    pub correlation_id: Option<Uuid>,
    pub trace_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
}

impl EventMetaData {
    pub fn new(
        producer: impl Into<String>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_version: "v1".to_string(),
            occurred_at: Utc::now(),
            producer: producer.into(),
            correlation_id: None,
            trace_id: None,
            user_id: None,
            session_id: None,
        }
    }

    pub fn with_correlation_id(mut self, id: Uuid) -> Self {
        self.correlation_id = Some(id);
        self
    }

    pub fn with_trace_id(mut self, id: Uuid) -> Self {
        self.trace_id = Some(id);
        self
    }

    pub fn with_user_id(mut self, id: Uuid) -> Self {
        self.user_id = Some(id);
        self
    }

    pub fn with_session_id(mut self, id: Uuid) -> Self {
        self.session_id = Some(id);
        self
    }
}

