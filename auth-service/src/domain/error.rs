pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidEmail,
    InvalidCredentials,   // Bad password, short password, etc.
    IncorrectCredentials, // Bad password, short password, etc.
    InvalidToken,
    UnexpectedError,
}
