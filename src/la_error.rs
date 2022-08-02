use std::fmt::Display;

use _sys::{INSError, NSError};

use crate::ns_string::NSString;

#[derive(Debug)]
pub struct LAError {
    code: i64,
    message: String,
}

impl Display for LAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl From<NSError> for LAError {
    fn from(error: NSError) -> Self {
        let code = unsafe { error.code() };

        let message = unsafe {
            let description: NSString = error.localizedDescription().into();
            description.into()
        };

        LAError { code, message }
    }
}
