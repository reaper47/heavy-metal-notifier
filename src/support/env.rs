use std::env;

use crate::error::{Error, Result};

/// Retrieves an environment variable.
///
/// # Returns
///
/// A `Result` containing the value of the environment variable as a `String` if successful,
/// or an `Error::MissingEnv` error if the environment variable is not found.
///
pub fn get_env(name: &'static str) -> Result<String> {
    match env::var(name) {
        Ok(v) => {
            let trimmed = v.trim_matches('"');
            if trimmed.is_empty() {
                Err(Error::MissingEnv(name))
            } else {
                Ok(trimmed.to_string())
            }
        },
        Err(_) => Err(Error::MissingEnv(name)),
    }
}
