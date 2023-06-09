use thiserror::Error;

#[derive(Error, Debug)]
pub enum DHT11Error {
    #[error("DHT11 initialization failed. {0}")]
    DHT11InitializationError(String),   
}
