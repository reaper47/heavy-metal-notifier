use std::sync::OnceLock;

use crate::{error::Result, support::env::get_env};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            eprintln!("Could not load configuration: {err:?}");
            std::process::exit(1);
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub BASE_URL: String,
    pub DATABASE_URL: String,
    pub IS_PROD: bool,
}

impl Config {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            BASE_URL: get_env("BASE_URL")?,
            DATABASE_URL: get_env("DATABASE_URL")?,
            IS_PROD: get_env("IS_PROD")? == "true",
        })
    }
}
