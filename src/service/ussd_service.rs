use crate::domain::{Session, UssdRequest, UssdResponse};
use crate::error::{Result, UssdError};
use crate::plugin::{PluginContext, PluginRegistry};
use crate::state::StateMachine;
use crate::storage::{Database, SessionStore};
use crate::metrics::{SESSIONS_TOTAL, ACTIVE_SESSIONS};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// USSD Service - Core business logic
#[derive(Clone)]
pub struct UssdService<S: SessionStore> {
    session_store: Arc<S>,
    state_machine: Arc<StateMachine>,
    plugin_registry: Arc<PluginRegistry>,
    database: Arc<Database>,
}

impl<S: SessionStore> UssdService<S> {
    pub fn new(
        session_store: Arc<S>,
        state_machine: Arc<StateMachine>,
        plugin_registry: Arc<PluginRegistry>,
        database: Arc<Database>,
    ) -> Self {
        Self {
            session_store,
            state_machine,
            plugin_registry,
            database,
        }
    }

    /// Process USSD request
    pub async fn process_request(&self, request: UssdRequest) -> Result<UssdResponse> {
        info!(
            session_id = %request.session_id,
            phone_number = %request.phone_number,
            input = %request.input,
            "Processing USSD request"
        );

        // Get or create session
        let mut session = self.get_or_create_session(&request).await?;

        // Ensure user exists in database
        let _user = self.database.get_or_create_user(&request.phone_number).await?;

        // If this is the first request (empty input), show main menu
        if request.input.is_empty() && session.current_state == "START" {
            let menu = self.state_machine.get_menu("START")?;
            let message = menu.format_menu(&session.language);

            session.touch();
            self.session_store.update(session.clone()).await?;

            return Ok(UssdResponse::new_continue(session.id, message));
        }

        // Process input through state machine
        let message = self.state_machine
            .process_input(&mut session, &request.input)
            .await?;

        // Check if we need to execute a plugin
        let current_menu = self.state_machine.get_current_menu(&session)?;
        let final_message = if let Some(handler) = &current_menu.handler {
            debug!(plugin = %handler, session_id = %session.id, "Executing plugin handler");

            let ctx = PluginContext {
                session: session.clone(),
                input: request.input.clone(),
                metadata: Default::default(),
            };

            match self.plugin_registry.execute(handler, &ctx).await {
                Ok(plugin_response) => plugin_response,
                Err(e) => {
                    warn!(error = %e, plugin = %handler, "Plugin execution failed");
                    return Err(e);
                }
            }
        } else {
            message
        };

        // Determine if session should continue
        let continue_session = current_menu.menu_type != crate::domain::MenuType::Response;

        // Update session
        session.touch();
        self.session_store.update(session.clone()).await?;

        // Create response
        let response = if continue_session {
            UssdResponse::new_continue(session.id, final_message)
        } else {
            // Session ended, optionally clean up
            UssdResponse::new_end(session.id, final_message)
        };

        Ok(response)
    }

    /// Get existing session or create new one
    async fn get_or_create_session(&self, request: &UssdRequest) -> Result<Session> {
        // Try to get existing session
        if let Some(session) = self.session_store.get(&request.session_id).await? {
            debug!(session_id = %request.session_id, "Existing session found");
            return Ok(session);
        }

        // Create new session
        let session = Session::new(request.phone_number.clone(), request.service_code.clone())
            .with_id(request.session_id.clone());

        let session = self.session_store.create(session).await?;

        // Update metrics
        SESSIONS_TOTAL
            .with_label_values(&[&request.service_code])
            .inc();
        ACTIVE_SESSIONS.inc();

        info!(session_id = %session.id, "New session created");

        Ok(session)
    }

    /// End a session
    pub async fn end_session(&self, session_id: &str) -> Result<()> {
        self.session_store.delete(session_id).await?;
        ACTIVE_SESSIONS.dec();
        info!(session_id = %session_id, "Session ended");
        Ok(())
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>> {
        self.session_store.get(session_id).await
    }

    /// Update session language
    pub async fn set_language(&self, session_id: &str, language: String) -> Result<()> {
        let mut session = self
            .session_store
            .get(session_id)
            .await?
            .ok_or_else(|| UssdError::SessionNotFound(session_id.to_string()))?;

        session.set_language(language);
        self.session_store.update(session).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::session_store::memory::MemorySessionStore;
    use crate::domain::MenuItem;

    async fn setup_test_service() -> UssdService<MemorySessionStore> {
        let session_store = Arc::new(MemorySessionStore::new());
        let state_machine = Arc::new(StateMachine::new());
        let plugin_registry = Arc::new(PluginRegistry::new());

        // Mock database - in real tests, use a test database
        let database = Arc::new(
            Database::new("postgresql://test").await.unwrap()
        );

        UssdService::new(
            session_store,
            state_machine,
            plugin_registry,
            database,
        )
    }

    #[tokio::test]
    async fn test_process_initial_request() {
        // This would require proper test setup
        // Skipping for now
    }
}
