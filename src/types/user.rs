use uuid::Uuid;

use super::Todo;

/// A oxide todo user. This is the user which is registered and logged in to the server.
#[derive(Debug, serde::Deserialize)]
pub struct User {
    /// The base url.
    #[serde(skip)]
    pub(crate) base_url: String,
    /// The username of the user. This is used to identify the user.
    /// This is `None` if the user is logged in by token.
    #[serde(rename = "username")]
    pub(crate) name: Option<String>,
    /// The user token, which is used to authenticate the user.
    pub(crate) token: String,
}

impl User {
    /// Rreturn the username of the user.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    /// Return the token of the user.
    /// This is used to authenticate the user.
    pub fn token(&self) -> &str {
        &self.token
    }
    /// Create new todo.
    /// ### Note
    /// You need to add a status to the todo before you await the future. Else it will return an error.
    pub fn create_todo(&self, title: impl Into<String>) -> Todo {
        Todo {
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            title: Some(title.into()),
            ..Default::default()
        }
    }
    /// Returns a todo by uuid. This will send a request to the server to get the todo.
    pub fn todo_by_uuid(&self, uuid: Uuid) -> Todo {
        Todo {
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            uuid: Some(uuid),
            ..Default::default()
        }
    }
}
