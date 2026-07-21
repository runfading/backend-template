#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("converter error:{0}")]
    ConverterError(String),

    #[error("not found:{0}")]
    NotFound(String),

    #[error("repository error:{0}")]
    Repository(String),
}
