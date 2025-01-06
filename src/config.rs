use std::env;

#[derive(Debug)]
pub struct Config {
    pub webhook: String,
    pub error_webhook: String,
    pub alert_url: String,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Ok(Config {
            webhook: env::var("WEBHOOK")?,
            error_webhook: env::var("ERROR_WEBHOOK")?,
            alert_url: env::var("ALERT_URL")?,
        })
    }
}
