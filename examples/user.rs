use oxide_todo_sdk::errors::Result as OxideTodoResult;
use oxide_todo_sdk::Client;

#[tokio::main]
async fn main() -> OxideTodoResult<()> {
    // Create a new client with the base url.
    let client = Client::new("http://localhost:8080");
    // Register a new user.
    println!("Registering a new user...");
    let registered_user = client.register("username", "password").await?;
    println!("Registered a new user successfully. User: {registered_user:?}");
    // login the user with username and password.
    println!("Logining the user...");
    let logined_user = client.login("username", "password").await?;
    println!("Logined the user successfully. User: {logined_user:?}");
    // Login the user by token. (This will not send any request to the server, and will not check if the token is valid.)
    let by_token_user = client.login_by_token("token")?;
    println!("Logined the user by token successfully. User: {by_token_user:?}");
    Ok(())
}
