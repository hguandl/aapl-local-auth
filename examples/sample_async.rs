#[cfg(feature = "async")]

use aapl_local_auth::{LAContext, LAPolicy};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let context = LAContext::new();

    match context.can_evaluate_policy(LAPolicy::DeviceOwnerAuthentication) {
        Ok(true) => (),
        Ok(false) => panic!("This device cannot evaluate policies"),
        Err(err) => panic!("Error: {}", err),
    }

    match context.evaluate_policy_async(LAPolicy::DeviceOwnerAuthentication, "Hello Async").await {
        Ok(true) => (),
        Ok(false) => panic!("This device cannot evaluate policies"),
        Err(err) => panic!("Error: {}", err),
    };
}
