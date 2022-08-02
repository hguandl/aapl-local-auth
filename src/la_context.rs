use _sys::{ILAContext, INSObject};
use block::ConcreteBlock;

use crate::{la_error::LAError, ns_string::NSString, LAPolicy};

#[derive(Debug)]
pub struct LAContext {
    inner: _sys::LAContext,
}

impl LAContext {
    pub fn new() -> Self {
        let ptr = unsafe { _sys::LAContext::alloc().init() };
        LAContext {
            inner: _sys::LAContext(ptr),
        }
    }
}

impl LAContext {
    pub fn can_evaluate_policy(&self, policy: LAPolicy) -> Result<bool, LAError> {
        let policy = policy as _sys::LAPolicy;
        let mut error = _sys::NSError::alloc();
        let available = unsafe { self.inner.canEvaluatePolicy_error_(policy, &mut error) };

        if error.0.is_null() {
            Ok(available)
        } else {
            Err(LAError::from(error))
        }
    }

    #[cfg(feature = "async")]
    pub async fn evaluate_policy_async(
        &self,
        policy: LAPolicy,
        localized_reason: &str,
    ) -> Result<bool, LAError> {
        use tokio::sync::oneshot::channel;
        use tokio::sync::Mutex;
        let (tx, rx) = channel();
        let tx = Mutex::new(Some(tx));
        self.evaluate_policy(policy, localized_reason, move |result| {
            if let Some(tx) = tx.blocking_lock().take() {
                tx.send(result).unwrap();
            }
        });
        rx.await.unwrap()
    }

    pub fn evaluate_policy<F>(&self, policy: LAPolicy, localized_reason: &str, reply: F)
    where
        F: Fn(Result<bool, LAError>) + 'static,
    {
        let policy = policy as _sys::LAPolicy;
        let reason = NSString::from(localized_reason);

        let callback = ConcreteBlock::new(move |success: bool, error: _sys::NSError| {
            if error.0.is_null() {
                reply(Ok(success));
            } else {
                reply(Err(LAError::from(error)));
            }
        })
        .copy();

        unsafe {
            self.inner
                .evaluatePolicy_localizedReason_reply_(policy, reason.into(), &*callback);
        }
    }

    pub fn invalidate(&self) {
        unsafe {
            self.inner.invalidate();
        }
    }

    pub fn set_localized_cancel_title(&self, title: &str) {
        let title = NSString::from(title);
        unsafe {
            self.inner.setLocalizedCancelTitle_(title.into());
        }
    }

    pub fn set_localized_fallback_title(&self, title: &str) {
        let title = NSString::from(title);
        unsafe {
            self.inner.setLocalizedFallbackTitle_(title.into());
        }
    }
}
