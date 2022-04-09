extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
pub struct StatusErrorJson {
    pub code: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct StatusJson<T> {
    pub succeeded: bool,
    pub result: Option<T>,
    pub error: Option<StatusErrorJson>,
}

pub enum StatusResult<T> {
    Ok(T),
    Error(usize, String),
}

impl<T> StatusResult<T>
    where
        T: Clone + serde::Serialize,
{

    pub fn from<E>(value: Result<T, E>) -> StatusResult<T> where E: Into<(usize, String)> {
        match value {
            Ok(v) => StatusResult::Ok(v),
            Err(e) => {
                let err: (usize, String) = e.into();
                StatusResult::Error(err.0, err.1)
            }
        }
    }

    pub fn as_json(&self) -> String {
        let obj = match self {
            StatusResult::Ok(ref t) => StatusJson {
                succeeded: true,
                result: Some(t.clone()),
                error: None,
            },
            StatusResult::Error(code, message) => StatusJson {
                succeeded: false,
                result: None,
                error: Some(StatusErrorJson {
                    code: *code,
                    message: message.clone(),
                }),
            },
        };
        let result = serde_json::to_string(&obj)
            .map_err(|_| "{\"succeeded\": false, \"error\": {\"code\": 0, \"message\": \"Failed to convert resulting JSON\"}}".to_string());
        match result {
            Ok(v) => v,
            Err(v) => v
        }
    }
}
