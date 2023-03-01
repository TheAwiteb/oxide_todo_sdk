# Oxide Todo SDK
A asynchronous SDK (Software Development Kit) for [oxide_todo](https://github.com/TheAwiteb/oxide_todo) written in Rust.
It provides a simple API to interact with the server.

## MSRV (Minimum Supported Rust Version)
The minimum supported Rust version is 1.64.0. (Recommended during development)

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
oxide_todo_sdk = "0.1.0"
```

## Examples
This example shows how to create a new todo:
```rust
use oxide_todo_sdk::types::TodoStatus;
use oxide_todo_sdk::Client;
use oxide_todo_sdk::errors::Result as OxideTodoResult;

#[tokio::main]
async fn main() -> OxideTodoResult<()> {
    let todo = Client::new("http://localhost:8080")
        .login("username", "password")
        .await? // Type: oxide_todo_sdk::types::User
        .create_todo("title") // Type: oxide_todo_sdk::types::Todo
        .set_status(TodoStatus::Completed) // Type: oxide_todo_sdk::types::Todo
        .await?; // Type: oxide_todo_sdk::types::Todo
    println!("Todo created: {todo:?}");
    // ^ This need `debug` feature
    Ok(())
}
```
As you can see above, the SDK is flexible and you can await the Todo type directly.

Check out the [documentation](https://docs.rs/oxide_todo_sdk) for more information. Also check out the [examples](https://github.com/TheAwiteb/oxide_todo_sdk/tree/master/examples) for more examples.

## Features
- [x] Authentication
    - [x] Register
    - [x] Login
    - [x] Revoke Token
- [x] Todos
    - [x] Create Todo
    - [x] Get Todo
    - [x] Update Todo
    - [X] Delete Todo
    - [ ] List Todos (including search)
    - [ ] Delete All Todos
- [Server Metadata]
    - [x] Get Server Metadata

## Contributing
If you want to contribute to this project, feel free to open a pull request. If you want to add a new feature, please open an issue first. If you have any questions, feel free to open an issue.

## Code of Conduct
This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## License
This project is licensed under the terms of the MIT license. See [LICENSE](https://github.com/TheAwiteb/oxide_todo_sdk/blob/master/LICENSE) for more information.