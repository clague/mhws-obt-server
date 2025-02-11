
use derive_more::Display;
use ntex::{http, web};

#[derive(Debug, Display)]
#[display("Error: {}", desc)]
pub struct ObtError {
    desc: String,
    status_code: http::StatusCode
}

impl ObtError {
    pub fn new(desc: &str, status_code: u16) -> Option<Self> {
        Some(ObtError { desc: desc.to_owned(), status_code: http::StatusCode::from_u16(status_code).ok()? })
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

impl<E: std::error::Error> From<E> for ObtError {
    fn from(value: E) -> Self {
        ObtError{ status_code: http::StatusCode::INTERNAL_SERVER_ERROR, desc: value.to_string()}
    }
}

pub static ERROR_PAGE: &str = "<html>
<head><title>{status_code}</title></head>
<body>
<center><h1>{status_code}</h1></center>
<hr><center>{desc}</center>
</body>
</html>";