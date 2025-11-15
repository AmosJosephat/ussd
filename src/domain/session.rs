use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// USSD Session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier
    pub id: String,

    /// User's phone number
    pub phone_number: String,

    /// Service code (e.g., *123#)
    pub service_code: String,

    /// Current menu state
    pub current_state: String,

    /// Session context/variables
    pub context: HashMap<String, String>,

    /// Session creation time
    pub created_at: DateTime<Utc>,

    /// Last activity time
    pub updated_at: DateTime<Utc>,

    /// Session language
    pub language: String,

    /// User ID (if authenticated)
    pub user_id: Option<Uuid>,

    /// Session metadata
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(phone_number: String, service_code: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            phone_number,
            service_code,
            current_state: "START".to_string(),
            context: HashMap::new(),
            created_at: now,
            updated_at: now,
            language: "en".to_string(),
            user_id: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn update_state(&mut self, new_state: String) {
        self.current_state = new_state;
        self.updated_at = Utc::now();
    }

    pub fn set_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
        self.updated_at = Utc::now();
    }

    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language;
        self.updated_at = Utc::now();
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

/// USSD Request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UssdRequest {
    /// Session ID (provided by telecom gateway)
    pub session_id: String,

    /// User's phone number
    pub phone_number: String,

    /// User input
    pub input: String,

    /// Service code
    pub service_code: String,

    /// Network operator
    pub network: Option<String>,
}

/// USSD Response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UssdResponse {
    /// Response message to display
    pub message: String,

    /// Session should continue
    pub continue_session: bool,

    /// Session ID
    pub session_id: String,
}

impl UssdResponse {
    pub fn new_continue(session_id: String, message: String) -> Self {
        Self {
            message,
            continue_session: true,
            session_id,
        }
    }

    pub fn new_end(session_id: String, message: String) -> Self {
        Self {
            message,
            continue_session: false,
            session_id,
        }
    }
}
