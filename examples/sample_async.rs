use aapl_local_auth::{LAContext, LAPolicy};

#[tokio::main]
async fn main() {
    let context = LAContext::default();

    match context.can_evaluate_policy(LAPolicy::DeviceOwnerAuthentication) {
        Ok(true) => (),
        Ok(false) => panic!("This device cannot evaluate policies"),
        Err(err) => panic!("Error: {}", err),
    }

    match context
        .evaluate_policy_async(LAPolicy::DeviceOwnerAuthentication, "Hello with Async")
        .await
    {
        Ok(true) => println!("Success!"),
        Ok(_) => println!("Failure!"),
        Err(err) => println!("Error: {}", err),
    };
}
