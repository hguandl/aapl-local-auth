#![allow(non_upper_case_globals)]

use _sys::{
    LAError_LAErrorAppCancel, LAError_LAErrorAuthenticationFailed,
    LAError_LAErrorBiometryDisconnected, LAError_LAErrorBiometryLockout,
    LAError_LAErrorBiometryNotAvailable, LAError_LAErrorBiometryNotEnrolled,
    LAError_LAErrorBiometryNotPaired, LAError_LAErrorInvalidContext,
    LAError_LAErrorInvalidDimensions, LAError_LAErrorNotInteractive, LAError_LAErrorPasscodeNotSet,
    LAError_LAErrorSystemCancel, LAError_LAErrorUserCancel, LAError_LAErrorUserFallback,
    LAError_LAErrorWatchNotAvailable,
};

use crate::ns_error::NSError;

#[derive(Debug, thiserror::Error)]
pub enum LAError {
    #[error("{0}")]
    AuthenticationFailed(NSError),
    #[error("{0}")]
    UserCancel(NSError),
    #[error("{0}")]
    UserFallback(NSError),
    #[error("{0}")]
    SystemCancel(NSError),
    #[error("{0}")]
    PasscodeNotSet(NSError),
    #[error("{0}")]
    AppCancel(NSError),
    #[error("{0}")]
    InvalidContext(NSError),
    #[error("{0}")]
    BiometryNotAvailable(NSError),
    #[error("{0}")]
    BiometryNotEnrolled(NSError),
    #[error("{0}")]
    BiometryLockout(NSError),
    #[error("{0}")]
    NotInteractive(NSError),
    #[error("{0}")]
    WatchNotAvailable(NSError),
    #[error("{0}")]
    BiometryNotPaired(NSError),
    #[error("{0}")]
    BiometryDisconnected(NSError),
    #[error("{0}")]
    InvalidDimensions(NSError),
}

impl From<NSError> for LAError {
    fn from(e: NSError) -> Self {
        match e.code {
            LAError_LAErrorAuthenticationFailed => LAError::AuthenticationFailed(e),
            LAError_LAErrorUserCancel => LAError::UserCancel(e),
            LAError_LAErrorUserFallback => LAError::UserFallback(e),
            LAError_LAErrorSystemCancel => LAError::SystemCancel(e),
            LAError_LAErrorPasscodeNotSet => LAError::PasscodeNotSet(e),
            LAError_LAErrorAppCancel => LAError::AppCancel(e),
            LAError_LAErrorInvalidContext => LAError::InvalidContext(e),
            LAError_LAErrorBiometryNotAvailable => LAError::BiometryNotAvailable(e),
            LAError_LAErrorBiometryNotEnrolled => LAError::BiometryNotEnrolled(e),
            LAError_LAErrorBiometryLockout => LAError::BiometryLockout(e),
            LAError_LAErrorNotInteractive => LAError::NotInteractive(e),
            LAError_LAErrorWatchNotAvailable => LAError::WatchNotAvailable(e),
            LAError_LAErrorBiometryNotPaired => LAError::BiometryNotPaired(e),
            LAError_LAErrorBiometryDisconnected => LAError::BiometryDisconnected(e),
            LAError_LAErrorInvalidDimensions => LAError::InvalidDimensions(e),
            _ => panic!("Unknown LAError code: {}", e.code),
        }
    }
}

impl From<_sys::NSError> for LAError {
    fn from(e: _sys::NSError) -> Self {
        NSError::from(e).into()
    }
}
