###### Test Kaon Runtime

An example that uses `kaon` as a custom Lambda runtime and `mesa` to build a container and test locally

Note also the need (currently) to set the following in `Cargo.toml`

```
[[bin]]
name = "bootstrap"
path = "src/main.rs"
```

| survey - `cargo run --release --manifest-path ../../Cargo.toml --bin mesa survey`

| build - `cargo run --release --manifest-path ../../Cargo.toml --bin mesa build`

| view - `cargo run --release --manifest-path ../../Cargo.toml --bin mesa view`

Send an example request (requires `jq`)
```
curl -XPOST "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{"test_request": "hello mesa!"}' | jq
```

The response (note that currently `context` is set with a "fake" data via `kaon`, minus the `aws_request_id`)
```
{
  "test_response": "hello mesa!",
  "test_context": {
    "aws_request_id": "5c005447-79e3-4101-ac14-33b187466057",
    "invoked_function_arn": "Lambda-Runtime-Invoked-Function-Arn",
    "identity": "Lambda-Runtime-Cognito-Identity",
    "client_context": "Lambda-Runtime-Client-Context"
  }
}
```
Sending an invalid request

```
curl -XPOST "http://localhost:9000/2015-03-31/functions/function/invocations" -d '{"test_reques": "something awesome"}' | jq
```

The response

```
{
  "error_message": "missing field `test_request` at line 1 column 36",
  "error_type": "Unhandled",
  "stack_trace": "unused"
}
```

| erode - ```cargo run --release --manifest-path ../../Cargo.toml --bin mesa erode```

More to come!
