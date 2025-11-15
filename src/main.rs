use std::sync::Arc;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use ussd_advanced::{
    api::{create_router, AppState},
    config::Config,
    menu::load_default_menus,
    plugin::{BalanceCheckPlugin, HelpPlugin, Plugin, PluginRegistry, TransferPlugin},
    service::UssdService,
    state::StateMachine,
    storage::{Database, RedisSessionStore},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init_tracing()?;

    info!("🚀 Starting USSD Advanced Server...");

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|e| {
        warn!("Failed to load config from environment: {}. Using defaults.", e);
        Config::default()
    });

    info!(
        host = %config.server.host,
        port = config.server.port,
        "Server configuration loaded"
    );

    // Initialize database
    info!("Connecting to database...");
    let database = Arc::new(Database::new(&config.database.url).await?);
    info!("✓ Database connected");

    // Initialize Redis session store
    info!("Connecting to Redis...");
    let session_store = Arc::new(
        RedisSessionStore::new(&config.redis.url, config.session_timeout()).await?,
    );
    info!("✓ Redis connected");

    // Initialize state machine
    let state_machine = Arc::new(StateMachine::new());

    // Load default menus
    let menus = load_default_menus();
    info!("Loading {} menus...", menus.len());
    state_machine.register_menus(menus);
    info!("✓ Menus loaded");

    // Initialize plugin registry
    let plugin_registry = Arc::new(PluginRegistry::new());

    // Register built-in plugins
    info!("Registering plugins...");
    plugin_registry
        .register(Arc::new(BalanceCheckPlugin) as Arc<dyn Plugin>)
        .await?;
    plugin_registry
        .register(Arc::new(TransferPlugin) as Arc<dyn Plugin>)
        .await?;
    plugin_registry
        .register(Arc::new(HelpPlugin) as Arc<dyn Plugin>)
        .await?;
    info!("✓ {} plugins registered", plugin_registry.count());

    // Create USSD service
    let service = Arc::new(UssdService::new(
        session_store.clone(),
        state_machine.clone(),
        plugin_registry.clone(),
        database.clone(),
    ));

    // Create HTTP app state
    let app_state = AppState {
        service,
        config: Arc::new(config.clone()),
    };

    // Build HTTP router
    let app = create_router(app_state);

    // Start HTTP server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("🌐 Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("✅ USSD Advanced Server is running!");
    info!("📡 HTTP API: http://{}", addr);
    info!("📊 Metrics: http://{}/metrics", addr);
    info!("❤️  Health: http://{}/health", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,ussd_advanced=debug"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    Ok(())
}
