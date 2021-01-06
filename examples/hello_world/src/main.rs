use lambda::{handler_fn, Context};
use serde_json::Value;

pub type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler_fn(mesa_handler)).await?;

    Ok(())
}

async fn mesa_handler(event: Value, _: Context) -> Result<Value, Error> {
    println!("{:#?}", event);
    Ok(event)
}
