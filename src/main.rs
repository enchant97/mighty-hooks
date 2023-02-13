pub use mighty_hooks_config::Config;

fn main() {
    let config_path = std::env::var("MIGHTY_HOOKS_CONFIG_PATH").unwrap_or("config.yaml".to_owned());
    let config = Config::from_yaml_file(&config_path).unwrap();
    println!("{:#?}", config);
}
