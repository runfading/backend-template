use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::convert::Into;
use std::fmt::Debug;
use tracing::{debug, error};
use utoipa::ToSchema;

#[derive(Debug, Clone)]
pub struct ApiErrorCode {
    code: u32,
    message: &'static str,
}

macro_rules! define_code {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $code:expr, $msg:expr);
        )+
    ) => {
        impl ApiErrorCode {
            $(
                $(#[$docs])*
                pub const $name: Self = Self {
                    code: $code,
                    message: $msg,
                };
            )+
        }

        // // 可选：提供一个从 u32 查找错误码的函数（类似 canonical_reason）
        // pub fn error_code_from_u32(code: u32) -> Option<&'static ApiErrorCode> {
        //     match code {
        //         $(
        //             $code => Some(&ApiErrorCode::$name),
        //         )+
        //         _ => None,
        //     }
        // }
    };
}

impl ApiErrorCode {
    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}

define_code! {
    /// 成功
    (SUCCESS, 0, "success");
    /// 未找到
    (NOT_FOUND, 4040, "not found");
    /// 未认证
    (UNAUTHORIZED, 4010, "auth failure");
    /// 服务器内部错误
    (INTERNAL_SERVER, 9999, "internal server error");
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Empty {}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: ApiErrorCode::SUCCESS.code(),
            message: ApiErrorCode::SUCCESS.message.to_string(),
            data: Some(data),
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ApiResponse<U> {
        ApiResponse {
            code: self.code,
            message: self.message,
            data: self.data.map(f),
        }
    }

    pub fn empty() -> Self {
        Self {
            code: ApiErrorCode::SUCCESS.code(),
            message: ApiErrorCode::SUCCESS.message.to_string(),
            data: None,
        }
    }
}

impl ApiResponse<Empty> {
    pub fn err(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn empty_ok() -> Self {
        Self {
            code: ApiErrorCode::SUCCESS.code(),
            message: ApiErrorCode::SUCCESS.message().to_string(),
            data: Some(Empty {}),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> Response {
        if tracing::enabled!(tracing::Level::DEBUG) {
            if let Ok(json) = serde_json::to_string(&self) {
                debug!("response: {}", json);
            } else {
                error!("failed to serialize response");
            }
        }

        Json(self).into_response()
    }
}
