use thiserror::Error;

#[derive(Error, Debug)]
pub enum DHT11Error {
    #[error("DHT11 initialization failed. {0}")]
    DHT11InitializationError(String),   
    #[error("Sensor Time Out")]
    SensorReadingTimeOut,
    #[error("Checksum error")]
    ChecksumError,
    #[error(transparent)]
    FloatParseError(#[from] std::num::ParseFloatError)
}
