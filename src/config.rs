use std::sync::OnceLock;
use tracing::warn;
use crate::{error::Result, support::env::get_env};

/// Gets the current Config struct. It will be initialized if not already done.
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            eprintln!("Could not load configuration: {err:?}");
            std::process::exit(1);
        })
    })
}

/// Configuration struct for the application.
#[derive(PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub BASE_URL: String,
    pub DATABASE_URL: String,
    pub IS_PROD: bool,
    pub smtp: Option<SmtpConfig>,
}

/// Configuration struct for the email client.
#[derive(PartialEq, Debug)]
pub struct SmtpConfig {
    pub relay: String,
    pub username: String,
    pub password: String,
    pub email_admin: String,
}

impl Config {
    /// Populates the Config's fields from the environment variables.
    pub fn load_from_env() -> Result<Self> {
        let mut base_url = get_env("BASE_URL")?;
        if base_url == "http://localhost" {
            let port = get_env("SERVICE_PORT")?;
            base_url = format!("{}:{}", base_url, port);
        }

        let smtp_relay = get_env("SMTP_RELAY");
        let smtp_username = get_env("SMTP_USERNAME");
        let smtp_password = get_env("SMTP_PASSWORD");
        let smtp_email_admin = get_env("SMTP_EMAIL_ADMIN");

        let smtp = if smtp_relay.is_err() || smtp_username.is_err() || smtp_password.is_err()  || smtp_email_admin.is_err() {
            warn!("Sending emails is disabled because not all SMTP environment variables are set.");
            None
        } else {
            Some(SmtpConfig {
                relay: smtp_relay?,
                username: smtp_username?,
                password: smtp_password?,
                email_admin: smtp_email_admin?,
            })
        };

        Ok(Self {
            BASE_URL: base_url,
            DATABASE_URL: get_env("DATABASE_URL")?,
            IS_PROD: get_env("IS_PROD")? == "true",
            smtp,
        })
    }

    /// Returns the base address with the protocol.
    pub fn local_server_addr(&self) -> String {
        let base_addr = &self.BASE_URL;
        let base_addr = base_addr
            .strip_prefix("http://")
            .unwrap_or_else(|| {
                base_addr
                    .strip_prefix("https://")
                    .expect("Expected address to start with http:// or https://")
            })
            .to_string();

        if base_addr.starts_with("localhost") {
            base_addr
        } else {
            let port = get_env("SERVICE_PORT").unwrap_or("7125".to_string());
            format!("localhost:{port}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_load_from_env_base_url_localhost_ok() -> Result<()> {
        let _guard = set_env_localhost();

        let config = Config::load_from_env()?;

        pretty_assertions::assert_eq!(
            config,
            Config {
                BASE_URL: "http://localhost:7125".to_string(),
                DATABASE_URL: "sqlite://user:password@localhost:5432".to_string(),
                IS_PROD: true,
                smtp: Some(SmtpConfig {
                    relay: "smtp.gmail.com".to_string(),
                    username: "my@gmail.com".to_string(),
                    password: "my app pass word".to_string(),
                    email_admin: "admin@email.com".to_string(),
                }),
            }
        );
        Ok(())
    }

    #[test]
    fn test_load_from_env_base_url_not_localhost_ok() -> Result<()> {
        let _guard = set_env_hosted();

        let config = Config::load_from_env()?;

        pretty_assertions::assert_eq!(
            config,
            Config {
                BASE_URL: "https://www.metal-releases.com".to_string(),
                DATABASE_URL: "sqlite://user:password@localhost:5432".to_string(),
                IS_PROD: false,
                smtp: Some(SmtpConfig {
                    relay: "smtp.gmail.com".to_string(),
                    username: "my@gmail.com".to_string(),
                    password: "my app pass word".to_string(),
                    email_admin: "admin@email.com".to_string(),
                }),
            }
        );
        Ok(())
    }

    #[test]
    fn test_local_server_addr_localhost_ok() -> Result<()> {
        let _guard = set_env_localhost();
        let config = Config::load_from_env()?;

        let addr = config.local_server_addr();

        pretty_assertions::assert_eq!(addr, "localhost:7125");
        Ok(())
    }

    #[test]
    fn test_local_server_addr_hosted_ok() -> Result<()> {
        let _guard = set_env_hosted();
        let config = Config::load_from_env()?;

        let addr = config.local_server_addr();

        pretty_assertions::assert_eq!(addr, "localhost:7125");
        Ok(())
    }

    fn set_env_localhost() -> env_lock::EnvGuard<'static> {
        env_lock::lock_env([
            ("BASE_URL", Some("http://localhost")),
            ("SERVICE_PORT", Some("7125")),
            (
                "DATABASE_URL",
                Some("sqlite://user:password@localhost:5432"),
            ),
            ("IS_PROD", Some("true")),
            ("SMTP_RELAY", Some("smtp.gmail.com")),
            ("SMTP_USERNAME", Some("my@gmail.com")),
            ("SMTP_PASSWORD", Some("my app pass word")),
            ("SMTP_EMAIL_ADMIN", Some("admin@email.com")),
        ])
    }

    fn set_env_hosted() -> env_lock::EnvGuard<'static> {
        env_lock::lock_env([
            ("BASE_URL", Some("https://www.metal-releases.com")),
            ("SERVICE_PORT", Some("7125")),
            (
                "DATABASE_URL",
                Some("sqlite://user:password@localhost:5432"),
            ),
            ("IS_PROD", Some("false")),
            ("SMTP_RELAY", Some("smtp.gmail.com")),
            ("SMTP_USERNAME", Some("my@gmail.com")),
            ("SMTP_PASSWORD", Some("my app pass word")),
            ("SMTP_EMAIL_ADMIN", Some("admin@email.com")),
        ])
    }
}
