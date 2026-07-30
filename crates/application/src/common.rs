use crate::error::ApplicationError;

#[derive(Debug)]
pub struct ApplicationResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

pub type ApplicationResult<T> = Result<ApplicationResponse<T>, ApplicationError>;

impl<T> ApplicationResponse<T> {
    pub fn ok<R>(data: T) -> ApplicationResult<R>
    where
        R: From<T>,
    {
        Ok(ApplicationResponse::<R> {
            code: 0,
            message: "success".into(),
            data: Some(data.into()),
        })
    }

    pub fn vec<R>(data: Vec<T>) -> ApplicationResult<Vec<R>>
    where
        R: From<T>,
    {
        let vec = data.into_iter().map(Into::into).collect::<Vec<R>>();
        Ok(ApplicationResponse::<Vec<R>> {
            code: 0,
            message: "success".to_string(),
            data: Some(vec),
        })
    }
}

impl ApplicationResponse<()> {
    pub fn empty_ok() -> ApplicationResult<()> {
        Self::ok(())
    }
}
