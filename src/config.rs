use std::env;

use crate::errors::Error;

#[derive(Debug)]
pub struct Config {
    pub webhook: String,
    pub error_webhook: String,
    pub alert_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        return Ok(Config {
            webhook: env::var("WEBHOOK")?,
            error_webhook: env::var("ERROR_WEBHOOK")?,
            alert_url: env::var("ALERT_URL")?,
        });
    }
}
