use std::fmt::Display;

use _sys::INSError;

use crate::ns_string::NSString;

#[derive(Debug)]
pub struct NSError {
    pub code: i64,
    pub localized_description: String,
}

impl Display for NSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.localized_description)
    }
}

impl From<_sys::NSError> for NSError {
    fn from(error: _sys::NSError) -> Self {
        let code = unsafe { error.code() };

        let localized_description = unsafe {
            let description: NSString = error.localizedDescription().into();
            description.into()
        };

        NSError {
            code,
            localized_description,
        }
    }
}
