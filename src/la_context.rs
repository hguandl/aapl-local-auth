use std::cell::Cell;
use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

use block::ConcreteBlock;
use objc::{class, msg_send, sel, sel_impl};

use crate::{la_error::LAError, ns_string::NSString, LAPolicy, NSObject};

#[derive(Debug)]
pub struct LAContext {
    ptr: NSObject,
}

impl LAContext {
    pub fn new() -> Self {
        let cls = class!(LAContext);
        let obj: NSObject = unsafe {
            let obj: NSObject = msg_send![cls, alloc];
            msg_send![obj, init]
        };
        LAContext { ptr: obj }
    }
}

impl Drop for LAContext {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.ptr, release];
        }
    }
}

impl LAContext {
    pub fn can_evaluate_policy(&self, policy: LAPolicy) -> Result<bool, LAError> {
        let policy = policy as _sys::LAPolicy;
        let mut error = MaybeUninit::<NSObject>::uninit().as_mut_ptr();
        let available =
            unsafe { _sys::LAContext::canEvaluatePolicy_error_(self.ptr, policy, &mut error) };
        if error.is_null() {
            Ok(available)
        } else {
            Err(LAError::from(error as NSObject))
        }
    }

    #[cfg(feature = "async")]
    pub async fn evaluate_policy_async(
        &self,
        policy: LAPolicy,
        localized_reason: &str,
    ) -> Result<bool, LAError> {
        let fut = EvaluateFuture::new();
        let inner = fut.inner.clone();
        self.evaluate_policy(policy, localized_reason, move |result| {
            let guard = inner.result.lock().unwrap();
            guard.set(Some(result));
            loop {
                if let Some(waker) = inner.waker.take() {
                    waker.wake();
                    break;
                }
                // the callback usually needs some time to be call (user need time to respond),
                // during that period the waker should already set.
            }
        });
        fut.await
    }

    pub fn evaluate_policy<F>(&self, policy: LAPolicy, localized_reason: &str, reply: F)
    where
        F: Fn(Result<bool, LAError>) + 'static,
    {
        let policy = policy as _sys::LAPolicy;
        let reason = NSString::from(localized_reason);

        let callback = ConcreteBlock::new(move |success: bool, error: *mut NSObject| {
            let error = error as NSObject;
            if error.is_null() {
                reply(Ok(success));
            } else {
                reply(Err(LAError::from(error)));
            }
        })
        .copy();

        unsafe {
            _sys::LAContext::evaluatePolicy_localizedReason_reply_(
                self.ptr,
                policy,
                reason.into(),
                &*callback,
            );
        }
    }

    pub fn invalidate(&self) {
        unsafe {
            _sys::LAContext::invalidate(self.ptr);
        }
    }

    pub fn set_localized_cancel_title(&self, title: &str) {
        let title = NSString::from(title);
        unsafe {
            _sys::LAContext::setLocalizedCancelTitle_(self.ptr, title.into());
        }
    }

    pub fn set_localized_fallback_title(&self, title: &str) {
        let title = NSString::from(title);
        unsafe {
            _sys::LAContext::setLocalizedFallbackTitle_(self.ptr, title.into());
        }
    }
}

struct EvaluateFuture {
    inner: Rc<EvaluateFutureInner>,
}

#[derive(Default)]
struct EvaluateFutureInner {
    result: Mutex<Cell<Option<Result<bool, LAError>>>>,
    waker: Cell<Option<Waker>>,
}

impl EvaluateFuture {
    fn new() -> EvaluateFuture {
        EvaluateFuture {
            inner: Rc::new(EvaluateFutureInner::default()),
        }
    }
}

impl Future for EvaluateFuture {
    type Output = Result<bool, LAError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let guard = self.inner.result.lock().unwrap();
        self.inner.waker.set(Some(cx.waker().clone()));
        if let Some(result) = guard.take() {
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}
