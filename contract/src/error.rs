use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    InsufficientAmount = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}
