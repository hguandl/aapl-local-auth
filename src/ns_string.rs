use std::ffi::CString;

use objc::{class, msg_send, sel, sel_impl};

use crate::NSObject;

#[derive(Debug)]
pub struct NSString {
    ptr: NSObject,
}

impl From<&str> for NSString {
    fn from(s: &str) -> Self {
        let ptr = unsafe {
            let cstr = CString::new(s).unwrap();
            let cls = class!(NSString);
            let obj: NSObject = msg_send![cls, alloc];
            msg_send![obj, initWithUTF8String: cstr.as_ptr()]
        };
        NSString { ptr }
    }
}

impl Into<NSObject> for NSString {
    fn into(self) -> NSObject {
        self.ptr
    }
}
