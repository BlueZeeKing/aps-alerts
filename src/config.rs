use std::env;

use dotenv::dotenv;

use crate::errors::Error;

pub struct Config {
    pub prod_webhook: String,
    pub dev_webhook: String,
    pub error_webhook: String,
    pub prod_alert_url: String,
    pub dev_alert_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        dotenv()?;

        return Ok(Config {
            prod_webhook: env::var("PROD_WEBHOOK")?,
            dev_webhook: env::var("DEV_WEBHOOK")?,
            error_webhook: env::var("ERROR_WEBHOOK")?,
            prod_alert_url: env::var("PROD_ALERT_URL")?,
            dev_alert_url: env::var("DEV_ALERT_URL")?,
        });
    }
}