use kaon::{Context, Kaon};
use serde::{Deserialize, Serialize};
use tracing_subscriber;

#[derive(Deserialize)]
struct TestRequest {
    test_request: String,
}

#[derive(Serialize)]
struct TestResponse {
    pub test_response: String,
    pub test_context: Context,
}

async fn test_handler_function(event: TestRequest, context: Context) -> Result<TestResponse, ()> {
    let response = TestResponse {
        test_response: event.test_request,
        test_context: context,
    };
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();

    let mut kaon = Kaon::charge().await;
    kaon.decay(test_handler_function).await;
    Ok(())
}
