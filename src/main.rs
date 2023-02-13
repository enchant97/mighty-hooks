use mighty_hooks_config::Config;
use mighty_hooks_server::run_server;

#[tokio::main]
pub async fn main() {
    // Setup logging
    std::env::set_var(
        "RUST_LOG",
        std::env::var("MIGHTY_HOOKS_LOG_LEVEL").unwrap_or(log::Level::Info.to_string()),
    );
    env_logger::init();

    // Load config
    let config_path = std::env::var("MIGHTY_HOOKS_CONFIG_PATH").unwrap_or("config.yaml".to_owned());
    let config = Config::from_yaml_file(&config_path).unwrap();
    log::debug!("{:#?}", &config);

    // Run server
    run_server(&config).await;
}
