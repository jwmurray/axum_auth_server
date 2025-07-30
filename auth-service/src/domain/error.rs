pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidEmail,
    InvalidCredentials, // Bad password, short password, etc.
    UnexpectedError,
}
