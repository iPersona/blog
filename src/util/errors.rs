#[derive(Debug, Serialize)]
pub enum ErrorCode {
    // User not exist
    UserNotExist,
    // User not activated
    UserNotActivated,
    // Invalid content data
    InvalidContent,
    // Invalid token
    InvalidToken,
    // Token expired
    TokenExpired,
    // Token associated errors
    TokenError,
    // Encode error
    EncodeError,
    // Permission denied
    PermissionDenied,
    // Email is not verified
    EmailNotVerified,
    // Login failed,
    LoginFailed,
    // Parse error
    ParseError,
    // Database error
    DbError,
    // Redis Error
    RedisError,
    // Unknown error
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: ErrorCode,
    pub detail: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code: {:?}, detail: {:?}", self.code, self.detail)
    }
}
