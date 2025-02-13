use std::{any::Any, fmt::Display};
use ntex::{http::{self, StatusCode}, web};


#[derive(Debug)]
pub struct ObtError {
    desc: String,
    status_code: http::StatusCode
}

impl Display for ObtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.desc)
    }
}

impl ObtError {
    pub fn new(status_code: impl Into<StatusCode>, desc: &str) -> Self {
        ObtError { desc: desc.to_owned(), status_code: status_code.into() }
    }
}

// Use default implementation for `error_response()` method
impl web::error::WebResponseError for ObtError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let status_code = self.status_code;
        web::HttpResponse::build(status_code)
            .set_header("content-type", "text/html; charset=utf-8")
            .body(ERROR_PAGE.replace("{status_code}", &(status_code.to_string()))
                            .replace("{desc}", &self.to_string()))
    }

    fn status_code(&self) -> http::StatusCode {
        self.status_code
    }
}

impl<E: std::error::Error + Any> From<E> for ObtError  {
    fn from(value: E) -> Self {
        Self { status_code:if (&value as &dyn Any).is::<std::io::Error>() { 
            http::StatusCode::NOT_FOUND 
        } else {
            http::StatusCode::INTERNAL_SERVER_ERROR
        }, desc: value.to_string() }
    }
}

pub(crate) static ERROR_PAGE: &str = "<html>
dy>
<center><h1>{status_code}</h1></center>
<hr><center>{desc}</center>
</body>
</html>";