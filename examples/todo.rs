use oxide_todo_sdk::errors::Result as OxideTodoResult;
use oxide_todo_sdk::types::TodoStatus;
use oxide_todo_sdk::Client;

#[tokio::main]
async fn main() -> OxideTodoResult<()> {
    // The user we will use.
    let user = Client::new("http://localhost:8080")
        .login("username", "password")
        .await?;
    // Get todo by uuid.
    let todo = user
        .todo_by_uuid("a26a61cc-8c2e-4237-977d-4ce0195735c3".parse().unwrap())
        .await?;
    println!("Todo: {todo:#?}");

    // Create a new todo.
    let todo = user
        .create_todo("Some new todo24")
        .set_status(TodoStatus::Completed)
        .await?;
    println!("Todo created: {todo:#?}");

    Ok(())
}
