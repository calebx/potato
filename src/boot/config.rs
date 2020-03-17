use lazy_static;
use serde::Deserialize;
use std::fs;
use toml;

lazy_static::lazy_static! {
    pub static ref CONFIG_PATH: &'static str = "/usr/local/etc/potato.toml";
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_config_debug")]
    debug: bool,

    #[serde(default = "default_config_redis_url")]
    redis_path: String,
}

fn default_config_debug() -> bool {
    true
}

fn default_config_redis_url() -> String {
    String::from("127.0.0.1:6379")
}

impl Default for Config {
    fn default() -> Self {
        Config {
            debug: default_config_debug(),
            redis_path: default_config_redis_url(),
        }
    }
}

impl Config {
    fn load() -> Self {
        if let Ok(toml_string) = fs::read_to_string(*CONFIG_PATH) {
            toml::from_str(&toml_string).unwrap()
        } else {
            Self::default()
        }
    }
}

pub fn init() {
    let config_clone = CONFIG.clone();
    println!("config loaded: {:?}", config_clone);
}
