use std::fmt::Display;

use crate::NSObject;

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

impl From<NSObject> for LAError {
    fn from(ptr: NSObject) -> Self {
        let message = unsafe {
            let obj = _sys::NSError::localizedDescription(ptr) as NSObject;
            let ptr = _sys::NSString_NSStringExtensionMethods::UTF8String(obj);
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
        .to_string();

        let code = unsafe { _sys::NSError::code(ptr) as i64 };
        LAError { code, message }
    }
}
