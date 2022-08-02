use std::ffi::CString;

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

impl Into<String> for NSString {
    fn into(self) -> String {
        unsafe {
            let ptr = self.inner.UTF8String();
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
        .to_string()
    }
}

impl Into<_sys::NSString> for NSString {
    fn into(self) -> _sys::NSString {
        self.inner
    }
}
