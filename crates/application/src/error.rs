use domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct BizError {
    pub code: u32,
    pub message: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    BizError(#[from] BizError),

    #[error(transparent)]
    DomainError(#[from] DomainError),
}
