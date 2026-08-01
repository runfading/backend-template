use crate::error::ApplicationError;
use api::ApiResponse;

pub type ApplicationResult<T> = Result<ApiResponse<T>, ApplicationError>;
