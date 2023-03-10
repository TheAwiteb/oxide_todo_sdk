use super::{Todo, Todos};
use crate::{api_helper::Endpoints, errors::Result as OxideResult};
use uuid::Uuid;

/// A oxide todo user. This is the user which is registered and logged in to the server.
///
/// You can create a new user by using [`Client::register`], [`Client::login`] or [`Client::login_by_token`], and you can revoke the token by using [`User::revoke_token`].
/// You can create a new todo by using [`User::create_todo`] and you can get a todo by using [`User::todo_by_uuid`].
///
/// [`Client::register`]: crate::Client::register
/// [`Client::login`]: crate::Client::login
/// [`Client::login_by_token`]: crate::Client::login_by_token
#[derive(serde::Deserialize, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[must_use]
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
    /// You cannot create a todo without a status. So you need to set the status of the todo after this.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo_sdk::Client;
    /// use oxide_todo_sdk::errors::Result as OxideResult;
    /// use oxide_todo_sdk::types::TodoStatus;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
    ///     let todo = user.create_todo("My new todo")
    ///         .status(TodoStatus::Completed) // Need to set the status of the todo before sending the request
    ///         .await?;
    ///     Ok(())
    /// }
    pub fn create_todo(&self, title: impl Into<String>) -> Todo {
        Todo {
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            title: Some(title.into()),
            ..Default::default()
        }
    }
    /// Returns a todo by uuid. await the future after this to get the todo. Or await it after you set the status or title to update the todo on the server.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo_sdk::Client;
    /// use oxide_todo_sdk::errors::Result as OxideResult;
    /// use oxide_todo_sdk::types::TodoStatus;
    /// use uuid::Uuid;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
    ///     let todo = user.todo_by_uuid(Uuid::new_v4()) // Get a todo by uuid
    ///         .status(TodoStatus::Completed); // Update the status of the todo
    ///         .await?; // Send the update request to the server
    ///     Ok(())
    /// }
    /// ```
    pub fn todo_by_uuid(&self, uuid: Uuid) -> Todo {
        Todo {
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            uuid: Some(uuid),
            ..Default::default()
        }
    }

    /// Revokes the token of the user.
    /// ## Note
    /// this will return a new user with a new token. So you need to update the user.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo_sdk::Client;
    /// use oxide_todo_sdk::errors::Result as OxideResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     let client = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
    ///     let user = user.revoke_token().await?;
    ///     // Just the token has been revoked
    ///     Ok(())
    /// }
    /// ```
    pub async fn revoke_token(self) -> OxideResult<Self> {
        let user = Endpoints::RevokeToken {
            base_url: &self.base_url,
            token: &self.token,
        }
        .await?;
        Ok(Self {
            base_url: self.base_url,
            ..serde_json::from_value(user).unwrap()
        })
    }

    /// Returns the todos of the user.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo_sdk::Client;
    /// use oxide_todo_sdk::errors::Result as OxideResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
    ///     let todos = user.todos().limit(3).offset(1).await?;
    ///     // Will return the first 3 todos after the first todo (2, 3, 4)
    ///     Ok(())
    /// }
    /// ```
    pub fn todos(&self) -> Todos {
        Todos::new(&self.base_url, &self.token)
    }

    /// Deletes all the todos of the user.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo_sdk::Client;
    /// use oxide_todo_sdk::errors::Result as OxideResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
    ///     // Delete all the todos of the user
    ///     user.delete_all_todos().await
    /// }
    /// ```
    pub async fn delete_all_todos(&self) -> OxideResult<()> {
        Endpoints::DeleteTodos {
            base_url: &self.base_url,
            token: &self.token,
        }
        .await?;
        Ok(())
    }
}
