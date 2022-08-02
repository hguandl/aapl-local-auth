mod la_context;
mod la_error;
mod ns_error;
mod ns_string;

pub use la_context::LAContext;
pub use la_error::LAError;

#[derive(Debug)]
pub enum LAPolicy {
    DeviceOwnerAuthenticationWithBiometrics =
        _sys::LAPolicy_LAPolicyDeviceOwnerAuthenticationWithBiometrics as isize,
    DeviceOwnerAuthentication = _sys::LAPolicy_LAPolicyDeviceOwnerAuthentication as isize,
    DeviceOwnerAuthenticationWithWatch =
        _sys::LAPolicy_LAPolicyDeviceOwnerAuthenticationWithWatch as isize,
    DeviceOwnerAuthenticationWithBiometricsOrWatch =
        _sys::LAPolicy_LAPolicyDeviceOwnerAuthenticationWithBiometricsOrWatch as isize,
}
