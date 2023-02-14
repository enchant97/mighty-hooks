use clap::Parser;
use mighty_hooks_config::{Config, EnvVarConfig};
use mighty_hooks_server::run_server;

mod args;

/// Load config from file, return config and path to config file
fn read_config(env_config: &EnvVarConfig) -> (Config, String) {
    let config_path = env_config
        .config_path
        .clone()
        .unwrap_or_else(|| "config.yaml".to_owned());
    let config = Config::from_yaml_file(&config_path).expect("Failed to load config file");
    (config, config_path)
}

#[tokio::main]
pub async fn main() {
    let env_config = EnvVarConfig::from_env().expect("Failed to load config from env vars");
    // Setup logging
    std::env::set_var(
        "RUST_LOG",
        env_config
            .log_level
            .clone()
            .unwrap_or_else(|| log::Level::Info.to_string()),
    );
    env_logger::init();

    let args = args::Args::parse();
    match args.cmd {
        args::Command::Serve => {
            // Load config
            let (config, config_path) = read_config(&env_config);
            log::info!("Loading config from '{}'", &config_path);
            log::debug!("config = {:#?}", &config);
            // log listening address
            match config.https {
                Some(_) => {
                    log::info!("listening on https://{}:{}", &config.host, &config.port);
                }
                None => log::info!("listening on http://{}:{}", &config.host, &config.port),
            }
            // Run server
            run_server(&config).await;
        }
        args::Command::Config(config_args) => match config_args.cmd {
            args::ConfigCommand::Show => {
                let (config, config_path) = read_config(&env_config);
                log::info!("Loading config from '{}'", &config_path);
                println!("{:#?}", &config);
            }
            args::ConfigCommand::Find => {
                let (_, config_path) = read_config(&env_config);
                println!("{}", &config_path);
            }
        },
        args::Command::Version => println!("Mighty Hooks {}", env!("CARGO_PKG_VERSION")),
    }
}
