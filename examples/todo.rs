use oxide_todo_sdk::errors::Result as OxideTodoResult;
use oxide_todo_sdk::types::TodoStatus;
use oxide_todo_sdk::Client;

#[tokio::main]
async fn main() -> OxideTodoResult<()> {
    let todo = Client::new("http://localhost:8080")
        .login("username", "password")
        .await?
        .create_todo("Some new todo")
        .set_status(TodoStatus::Completed)
        .await?;
    println!("Todo created: {todo:?}");
    Ok(())
}
