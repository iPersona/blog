#[derive(Debug, Serialize)]
pub enum ErrorCode {
    UserExist,
    PermissionDenied,
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct Error {
    code: ErrorCode,
    detail: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code: {:?}, detail: {:?}", self.code, self.detail)
    }
}
