use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpecterError {
    #[error("SDR device not available: {0}")]
    SDRNotAvailable(String),

    #[error("SDR device error: {0}")]
    SDRDeviceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Channel change failed: {0}")]
    ChannelChangeFailed(String),

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Sensor not found: {0}")]
    SensorNotFound(String),

    #[error("Radio not found: {0}")]
    RadioNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("DSP processing error: {0}")]
    DSPError(String),

    #[error("Detection error: {0}")]
    DetectionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl From<toml::de::Error> for SpecterError {
    fn from(e: toml::de::Error) -> Self {
        SpecterError::ConfigError(e.to_string())
    }
}

impl From<serde_json::Error> for SpecterError {
    fn from(e: serde_json::Error) -> Self {
        SpecterError::SerializationError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SpecterError>;
