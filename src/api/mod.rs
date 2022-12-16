pub mod links;

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResult<T: Serialize> {
    pub ok: bool,
    pub err: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            ok: true,
            err: None,
            data: Some(data),
        }
    }

    pub fn error(err: String) -> Self {
        Self {
            ok: false,
            err: Some(err.to_string()),
            data: None,
        }
    }
}
