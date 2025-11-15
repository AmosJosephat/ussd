use crate::domain::Session;
use crate::error::{Result, UssdError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, info};

/// Plugin context passed to handlers
#[derive(Debug, Clone)]
pub struct PluginContext {
    pub session: Session,
    pub input: String,
    pub metadata: HashMap<String, String>,
}

/// Plugin result
pub type PluginResult = Result<String>;

/// Plugin trait - implement this for custom business logic
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Handle the USSD request
    async fn handle(&self, ctx: &PluginContext) -> PluginResult;

    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin description
    fn description(&self) -> &str {
        "No description provided"
    }

    /// Initialize the plugin (optional)
    async fn init(&self) -> Result<()> {
        Ok(())
    }

    /// Shutdown the plugin (optional)
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

/// Plugin registry for managing plugins
#[derive(Clone)]
pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, Arc<dyn Plugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a plugin
    pub async fn register(&self, plugin: Arc<dyn Plugin>) -> Result<()> {
        let name = plugin.name().to_string();

        // Initialize plugin
        plugin.init().await?;

        // Store plugin
        self.plugins.write().insert(name.clone(), plugin);

        info!(plugin_name = %name, "Plugin registered");

        Ok(())
    }

    /// Unregister a plugin
    pub async fn unregister(&self, name: &str) -> Result<()> {
        let plugin = {
            self.plugins.write().remove(name)
        };

        if let Some(plugin) = plugin {
            plugin.shutdown().await?;
            info!(plugin_name = %name, "Plugin unregistered");
        }

        Ok(())
    }

    /// Execute a plugin by name
    pub async fn execute(&self, name: &str, ctx: &PluginContext) -> PluginResult {
        let plugin = self
            .plugins
            .read()
            .get(name)
            .cloned()
            .ok_or_else(|| UssdError::PluginError(format!("Plugin not found: {}", name)))?;

        debug!(plugin_name = %name, session_id = %ctx.session.id, "Executing plugin");

        plugin.handle(ctx).await
    }

    /// Check if plugin exists
    pub fn has(&self, name: &str) -> bool {
        self.plugins.read().contains_key(name)
    }

    /// List all registered plugins
    pub fn list(&self) -> Vec<String> {
        self.plugins.read().keys().cloned().collect()
    }

    /// Get plugin count
    pub fn count(&self) -> usize {
        self.plugins.read().len()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Built-in plugins

/// Balance check plugin (example)
pub struct BalanceCheckPlugin;

#[async_trait]
impl Plugin for BalanceCheckPlugin {
    async fn handle(&self, ctx: &PluginContext) -> PluginResult {
        // Example: retrieve balance from context or database
        let balance = ctx
            .session
            .get_context("balance")
            .unwrap_or(&"0.00".to_string())
            .clone();

        Ok(format!("Your balance is: ${}", balance))
    }

    fn name(&self) -> &str {
        "balance_check"
    }

    fn description(&self) -> &str {
        "Check user account balance"
    }
}

/// Transfer money plugin (example)
pub struct TransferPlugin;

#[async_trait]
impl Plugin for TransferPlugin {
    async fn handle(&self, ctx: &PluginContext) -> PluginResult {
        let recipient = ctx
            .session
            .get_context("transfer_recipient")
            .ok_or_else(|| UssdError::InvalidInput("Recipient not specified".to_string()))?;

        let amount = ctx
            .session
            .get_context("transfer_amount")
            .ok_or_else(|| UssdError::InvalidInput("Amount not specified".to_string()))?;

        // In production, this would interact with payment gateway
        Ok(format!(
            "Transfer of ${} to {} is being processed",
            amount, recipient
        ))
    }

    fn name(&self) -> &str {
        "transfer"
    }

    fn description(&self) -> &str {
        "Transfer money to another account"
    }
}

/// Help plugin
pub struct HelpPlugin;

#[async_trait]
impl Plugin for HelpPlugin {
    async fn handle(&self, _ctx: &PluginContext) -> PluginResult {
        Ok("For assistance, please call 1-800-HELP or visit our website.".to_string())
    }

    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Show help information"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_registration() {
        let registry = PluginRegistry::new();

        let plugin = Arc::new(HelpPlugin) as Arc<dyn Plugin>;
        registry.register(plugin).await.unwrap();

        assert!(registry.has("help"));
        assert_eq!(registry.count(), 1);
    }

    #[tokio::test]
    async fn test_plugin_execution() {
        let registry = PluginRegistry::new();

        let plugin = Arc::new(HelpPlugin) as Arc<dyn Plugin>;
        registry.register(plugin).await.unwrap();

        let ctx = PluginContext {
            session: Session::new("+1234567890".to_string(), "*123#".to_string()),
            input: "".to_string(),
            metadata: HashMap::new(),
        };

        let result = registry.execute("help", &ctx).await;
        assert!(result.is_ok());
    }
}
