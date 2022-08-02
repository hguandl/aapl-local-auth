use aapl_local_auth::{LAContext, LAPolicy};
use std::sync::mpsc::channel;

fn main() {
    let context = LAContext::default();

    match context.can_evaluate_policy(LAPolicy::DeviceOwnerAuthentication) {
        Ok(true) => (),
        Ok(false) => panic!("This device cannot evaluate policies"),
        Err(err) => panic!("Error: {}", err),
    };

    let (tx, rx) = channel();

    context.evaluate_policy(
        LAPolicy::DeviceOwnerAuthentication,
        "Hello from Rust",
        move |result| {
            match result {
                Ok(true) => println!("Success!"),
                Ok(_) => println!("Failure!"),
                Err(err) => println!("Error: {}", err),
            };
            tx.send(()).unwrap();
        },
    );

    rx.recv().unwrap();
}
