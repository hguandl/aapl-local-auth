use std::ffi::{CStr, CString};

use _sys::NSString_NSStringExtensionMethods;

#[derive(Debug)]
pub struct NSString {
    inner: _sys::NSString,
}

impl From<&str> for NSString {
    fn from(s: &str) -> Self {
        let ptr = unsafe {
            let cstr = CString::new(s).unwrap();
            _sys::NSString::alloc().initWithUTF8String_(cstr.as_ptr())
        };
        NSString {
            inner: _sys::NSString(ptr),
        }
    }
}

impl From<_sys::NSString> for NSString {
    fn from(s: _sys::NSString) -> Self {
        NSString { inner: s }
    }
}

impl From<NSString> for _sys::NSString {
    fn from(s: NSString) -> Self {
        s.inner
    }
}

impl From<NSString> for String {
    fn from(s: NSString) -> Self {
        unsafe {
            let cstr = s.inner.UTF8String();
            CStr::from_ptr(cstr).to_str().unwrap().to_string()
        }
    }
}
