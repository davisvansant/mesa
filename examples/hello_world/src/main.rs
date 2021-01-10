// use lambda::{handler_fn, Context};
// use serde_json::Value;
//
// pub type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let runtime_api = "AWS_LAMBDA_RUNTIME_API";
//     std::env::set_var(runtime_api, "0.0.0.0:9001");
//     let function_name = "AWS_LAMBDA_FUNCTION_NAME";
//     std::env::set_var(function_name, "mesa_handler");
//     let memory_size = "AWS_LAMBDA_FUNCTION_MEMORY_SIZE";
//     std::env::set_var(memory_size, "3008");
//     let function_version = "AWS_LAMBDA_FUNCTION_VERSION";
//     std::env::set_var(function_version, "0.1.0");
//     let stream_name = "AWS_LAMBDA_LOG_STREAM_NAME";
//     std::env::set_var(stream_name, "logwatch stream name");
//     let group_name = "AWS_LAMBDA_LOG_GROUP_NAME";
//     std::env::set_var(group_name, "logwatch group name");
//
//     for (k, v) in std::env::vars() {
//         println!("{} {}", k, v);
//     }
//     lambda::run(handler_fn(mesa_handler)).await?;
//
//     Ok(())
// }
//
// async fn mesa_handler(event: Value, _: Context) -> Result<Value, Error> {
//     println!("{:#?}", event);
//     Ok(event)
// }

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
struct LambdaEvent {
    hi: String,
}

#[derive(Serialize)]
struct LambdaOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // simple_logger::init_with_level(log::Level::Debug)?;
    // let runtime_api = "AWS_LAMBDA_RUNTIME_API";
    // std::env::set_var(runtime_api, "0.0.0.0:9001");
    // let runtime_api = "RUNTIME_ENDPOINT_VAR";
    // std::env::set_var(runtime_api, "0.0.0.0:9001");
    // simple_logger::init().unwrap();
    simple_logger::SimpleLogger::new().init().unwrap();

    let runtime_api = "AWS_LAMBDA_RUNTIME_API";
    std::env::set_var(runtime_api, "0.0.0.0:9001");
    let function_name = "AWS_LAMBDA_FUNCTION_NAME";
    std::env::set_var(function_name, "mesa_handler");
    let memory_size = "AWS_LAMBDA_FUNCTION_MEMORY_SIZE";
    std::env::set_var(memory_size, "3008");
    let function_version = "AWS_LAMBDA_FUNCTION_VERSION";
    std::env::set_var(function_version, "0.1.0");
    let stream_name = "AWS_LAMBDA_LOG_STREAM_NAME";
    std::env::set_var(stream_name, "logwatch stream name");
    let group_name = "AWS_LAMBDA_LOG_GROUP_NAME";
    std::env::set_var(group_name, "logwatch group name");

    for (k, v) in std::env::vars() {
        println!("{} {}", k, v);
    }
    lambda!(mesa_handler);

    Ok(())
}

fn mesa_handler(event: LambdaEvent, _c: Context) -> Result<LambdaOutput, HandlerError> {
    Ok(LambdaOutput {
        message: format!("Hello, {}!", event.hi),
    })
}
