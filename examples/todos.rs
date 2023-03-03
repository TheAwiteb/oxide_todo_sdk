use oxide_todo_sdk::errors::Result as OxideResult;
use oxide_todo_sdk::types::{TodoStatus, TodoOrder};
use oxide_todo_sdk::Client;

#[tokio::main]
async fn main() -> OxideResult<()> {
    // Login the user with username and password.
    let user = Client::new("http://localhost:8080")
        .login("username", "password")
        .await?;
    // Print the todos that in progress.
    println!("- Todos in progress: ");
    user.todos()
        .status(TodoStatus::Progress)
        .await?
        .iter()
        .for_each(|todo| {
            println!(" - {}", todo.title().unwrap());
        });
    // Print the homeworks that in progress.
    println!("- Homeworks in progress: ");
    user.todos()
        .status(TodoStatus::Progress)
        .title("Homework")
        .await?
        .iter()
        .for_each(|todo| {
            println!(" - {}", todo.title().unwrap());
        });
    // Print the todos that are pinding, and contains 'issue' in the title, ordered from older to newer.
    println!("- Todos in progress with 'issue' in the title: ");
    user.todos()
        .status(TodoStatus::Pending)
        .title("issue")
        .order(TodoOrder::Older)
        .await?
        .iter()
        .for_each(|todo| {
            println!(" - {}", todo.title().unwrap());
        });

    Ok(())
}
