use crate::domain::Session;
use crate::error::{Result, UssdError};
use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands};
use std::time::Duration;
use tracing::{debug, info};

const SESSION_PREFIX: &str = "ussd:session:";
const PHONE_SESSIONS_PREFIX: &str = "ussd:phone_sessions:";

/// Session store trait
#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create(&self, session: Session) -> Result<Session>;
    async fn get(&self, session_id: &str) -> Result<Option<Session>>;
    async fn update(&self, session: Session) -> Result<Session>;
    async fn delete(&self, session_id: &str) -> Result<()>;
    async fn exists(&self, session_id: &str) -> Result<bool>;
    async fn get_by_phone(&self, phone_number: &str) -> Result<Vec<Session>>;
    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize>;
}

/// Redis-backed session store
#[derive(Clone)]
pub struct RedisSessionStore {
    client: ConnectionManager,
    ttl: u64, // Time to live in seconds
}

impl RedisSessionStore {
    pub async fn new(redis_url: &str, ttl: Duration) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;

        info!(redis_url = redis_url, ttl_secs = ttl.as_secs(), "Redis session store initialized");

        Ok(Self {
            client: manager,
            ttl: ttl.as_secs(),
        })
    }

    fn session_key(&self, session_id: &str) -> String {
        format!("{}{}", SESSION_PREFIX, session_id)
    }

    fn phone_sessions_key(&self, phone_number: &str) -> String {
        format!("{}{}", PHONE_SESSIONS_PREFIX, phone_number)
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn create(&self, session: Session) -> Result<Session> {
        let mut conn = self.client.clone();
        let key = self.session_key(&session.id);
        let phone_key = self.phone_sessions_key(&session.phone_number);

        let serialized = serde_json::to_string(&session)?;

        // Store session with TTL
        conn.set_ex::<_, _, ()>(&key, &serialized, self.ttl).await?;

        // Add to phone number index
        conn.sadd::<_, _, ()>(&phone_key, &session.id).await?;
        conn.expire::<_, ()>(&phone_key, self.ttl as i64).await?;

        info!(
            session_id = %session.id,
            phone_number = %session.phone_number,
            ttl = self.ttl,
            "Session created"
        );

        Ok(session)
    }

    async fn get(&self, session_id: &str) -> Result<Option<Session>> {
        let mut conn = self.client.clone();
        let key = self.session_key(session_id);

        let data: Option<String> = conn.get(&key).await?;

        match data {
            Some(json) => {
                let session: Session = serde_json::from_str(&json)?;
                debug!(session_id = %session_id, "Session retrieved");
                Ok(Some(session))
            }
            None => {
                debug!(session_id = %session_id, "Session not found");
                Ok(None)
            }
        }
    }

    async fn update(&self, session: Session) -> Result<Session> {
        let mut conn = self.client.clone();
        let key = self.session_key(&session.id);

        // Check if session exists
        let exists: bool = conn.exists(&key).await?;
        if !exists {
            return Err(UssdError::SessionNotFound(session.id.clone()));
        }

        let serialized = serde_json::to_string(&session)?;

        // Update session and reset TTL
        conn.set_ex::<_, _, ()>(&key, &serialized, self.ttl).await?;

        debug!(
            session_id = %session.id,
            state = %session.current_state,
            "Session updated"
        );

        Ok(session)
    }

    async fn delete(&self, session_id: &str) -> Result<()> {
        let mut conn = self.client.clone();

        // Get session to find phone number
        if let Some(session) = self.get(session_id).await? {
            let key = self.session_key(session_id);
            let phone_key = self.phone_sessions_key(&session.phone_number);

            // Remove session
            conn.del::<_, ()>(&key).await?;

            // Remove from phone number index
            conn.srem::<_, _, ()>(&phone_key, session_id).await?;

            info!(session_id = %session_id, "Session deleted");
        }

        Ok(())
    }

    async fn exists(&self, session_id: &str) -> Result<bool> {
        let mut conn = self.client.clone();
        let key = self.session_key(session_id);
        let exists: bool = conn.exists(&key).await?;
        Ok(exists)
    }

    async fn get_by_phone(&self, phone_number: &str) -> Result<Vec<Session>> {
        let mut conn = self.client.clone();
        let phone_key = self.phone_sessions_key(phone_number);

        let session_ids: Vec<String> = conn.smembers(&phone_key).await?;

        let mut sessions = Vec::new();
        for session_id in session_ids {
            if let Some(session) = self.get(&session_id).await? {
                sessions.push(session);
            }
        }

        Ok(sessions)
    }

    async fn cleanup_expired(&self, _max_age: Duration) -> Result<usize> {
        // Redis handles expiration automatically via TTL
        // This method is here for interface compatibility
        debug!("Redis auto-expires sessions, manual cleanup not needed");
        Ok(0)
    }
}

/// In-memory session store (for testing)
#[cfg(test)]
pub mod memory {
    use super::*;
    use dashmap::DashMap;
    use std::sync::Arc;

    #[derive(Clone)]
    pub struct MemorySessionStore {
        sessions: Arc<DashMap<String, Session>>,
    }

    impl MemorySessionStore {
        pub fn new() -> Self {
            Self {
                sessions: Arc::new(DashMap::new()),
            }
        }
    }

    #[async_trait]
    impl SessionStore for MemorySessionStore {
        async fn create(&self, session: Session) -> Result<Session> {
            self.sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }

        async fn get(&self, session_id: &str) -> Result<Option<Session>> {
            Ok(self.sessions.get(session_id).map(|s| s.clone()))
        }

        async fn update(&self, session: Session) -> Result<Session> {
            if !self.sessions.contains_key(&session.id) {
                return Err(UssdError::SessionNotFound(session.id.clone()));
            }
            self.sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }

        async fn delete(&self, session_id: &str) -> Result<()> {
            self.sessions.remove(session_id);
            Ok(())
        }

        async fn exists(&self, session_id: &str) -> Result<bool> {
            Ok(self.sessions.contains_key(session_id))
        }

        async fn get_by_phone(&self, phone_number: &str) -> Result<Vec<Session>> {
            Ok(self
                .sessions
                .iter()
                .filter(|entry| entry.value().phone_number == phone_number)
                .map(|entry| entry.value().clone())
                .collect())
        }

        async fn cleanup_expired(&self, _max_age: Duration) -> Result<usize> {
            Ok(0)
        }
    }
}
