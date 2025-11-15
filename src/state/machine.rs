use crate::domain::{MenuItem, MenuType, Session, ValidationRules};
use crate::error::{Result, UssdError};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, info};

/// State machine for USSD navigation
#[derive(Clone)]
pub struct StateMachine {
    menus: Arc<RwLock<HashMap<String, MenuItem>>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            menus: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_menu(&self, menu: MenuItem) {
        let id = menu.id.clone();
        debug!(menu_id = %id, "Registering menu");
        self.menus.write().insert(id, menu);
    }

    pub fn register_menus(&self, menus: Vec<MenuItem>) {
        for menu in menus {
            self.register_menu(menu);
        }
    }

    pub fn get_menu(&self, menu_id: &str) -> Result<MenuItem> {
        self.menus
            .read()
            .get(menu_id)
            .cloned()
            .ok_or_else(|| UssdError::MenuNotFound(menu_id.to_string()))
    }

    pub fn get_current_menu(&self, session: &Session) -> Result<MenuItem> {
        self.get_menu(&session.current_state)
    }

    pub async fn process_input(
        &self,
        session: &mut Session,
        input: &str,
    ) -> Result<String> {
        let current_menu = self.get_current_menu(session)?;

        info!(
            session_id = %session.id,
            current_state = %session.current_state,
            input = %input,
            menu_type = ?current_menu.menu_type,
            "Processing input"
        );

        match current_menu.menu_type {
            MenuType::Menu => self.process_menu_input(session, &current_menu, input).await,
            MenuType::Input => self.process_text_input(session, &current_menu, input).await,
            MenuType::Response => Ok(current_menu.get_title(&session.language)),
            MenuType::Dynamic => {
                // Dynamic menus are handled by plugins
                Ok(current_menu.get_title(&session.language))
            }
        }
    }

    async fn process_menu_input(
        &self,
        session: &mut Session,
        menu: &MenuItem,
        input: &str,
    ) -> Result<String> {
        let option = menu
            .get_option(input.trim())
            .ok_or_else(|| UssdError::InvalidInput(format!("Invalid option: {}", input)))?;

        debug!(
            session_id = %session.id,
            option_key = %option.key,
            next_state = %option.next_state,
            "Valid option selected"
        );

        // Update session state
        session.update_state(option.next_state.clone());

        // Store selected option in context
        session.set_context(
            format!("menu_{}_selection", menu.id),
            option.key.clone(),
        );

        // Get next menu
        let next_menu = self.get_menu(&option.next_state)?;
        Ok(next_menu.format_menu(&session.language))
    }

    async fn process_text_input(
        &self,
        session: &mut Session,
        menu: &MenuItem,
        input: &str,
    ) -> Result<String> {
        // Validate input if rules exist
        if let Some(validation) = &menu.validation {
            self.validate_input(input, validation)?;
        }

        debug!(
            session_id = %session.id,
            menu_id = %menu.id,
            input_length = input.len(),
            "Storing text input"
        );

        // Store input in context
        session.set_context(format!("input_{}", menu.id), input.to_string());

        // Move to next state (first option's next_state)
        if let Some(option) = menu.options.first() {
            session.update_state(option.next_state.clone());
            let next_menu = self.get_menu(&option.next_state)?;
            Ok(next_menu.format_menu(&session.language))
        } else {
            Err(UssdError::InvalidStateTransition {
                from: menu.id.clone(),
                to: "unknown".to_string(),
            })
        }
    }

    fn validate_input(&self, input: &str, rules: &ValidationRules) -> Result<()> {
        if rules.required && input.trim().is_empty() {
            return Err(UssdError::InvalidInput("Input is required".to_string()));
        }

        if let Some(min_len) = rules.min_length {
            if input.len() < min_len {
                return Err(UssdError::InvalidInput(format!(
                    "Input must be at least {} characters",
                    min_len
                )));
            }
        }

        if let Some(max_len) = rules.max_length {
            if input.len() > max_len {
                return Err(UssdError::InvalidInput(format!(
                    "Input must be at most {} characters",
                    max_len
                )));
            }
        }

        if let Some(pattern) = &rules.pattern {
            let regex = regex::Regex::new(pattern)
                .map_err(|e| UssdError::InternalError(format!("Invalid regex: {}", e)))?;

            if !regex.is_match(input) {
                return Err(UssdError::InvalidInput(
                    "Input does not match required format".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn list_menus(&self) -> Vec<String> {
        self.menus.read().keys().cloned().collect()
    }

    pub fn clear_menus(&self) {
        self.menus.write().clear();
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{MenuOption, Session};

    #[tokio::test]
    async fn test_state_machine_navigation() {
        let sm = StateMachine::new();

        // Create test menus
        let main_menu = MenuItem {
            id: "START".to_string(),
            title: [("en".to_string(), "Welcome\n1. Option 1".to_string())]
                .into_iter()
                .collect(),
            options: vec![MenuOption {
                key: "1".to_string(),
                label: [("en".to_string(), "Option 1".to_string())]
                    .into_iter()
                    .collect(),
                next_state: "OPTION_1".to_string(),
                action: None,
            }],
            menu_type: MenuType::Menu,
            handler: None,
            parent: None,
            validation: None,
        };

        sm.register_menu(main_menu);

        let mut session = Session::new("+1234567890".to_string(), "*123#".to_string());

        // Test valid navigation
        let result = sm.process_input(&mut session, "1").await;
        assert!(result.is_ok());
        assert_eq!(session.current_state, "OPTION_1");
    }
}
