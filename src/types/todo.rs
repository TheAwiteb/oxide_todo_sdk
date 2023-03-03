use crate::{
    api_helper::Endpoints,
    errors::{Result as OxideResult, SDKError},
};
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};
use uuid::Uuid;

/// The todo status.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    /// The todo are completed.
    Completed,
    /// The todo are in progress.
    Progress,
    /// The todo are pending.
    Pending,
    /// The todo are cancelled.
    Cancelled,
}

/// Todo type is flexible. You can await it directly. How its works?
/// - If the todo you awaited it has a uuid, it will update the todo on the server if you set the title or status, else will get the todo from the server.
/// - If the todo you awaited it has no uuid, it will create a new todo on the server.
///
/// For example you want to create a new todo:
/// ```rust |no_run
/// use oxide_todo_sdk::Client;
/// use oxide_todo_sdk::types::TodoStatus;
/// use oxide_todo_sdk::errors::Result as OxideResult;
///
/// #[tokio::main]
/// async fn main() -> OxideResult<()> {
///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
///     let todo = user.create_todo("My new todo").set_status(TodoStatus::Progress).await?;
///     Ok(())
/// }
/// ```
/// Just like that. You can also update a todo by uuid:
/// ```rust |no_run
/// use oxide_todo_sdk::Client;
/// use oxide_todo_sdk::types::TodoStatus;
/// use oxide_todo_sdk::errors::Result as OxideResult;
/// use uuid::Uuid;
///
/// #[tokio::main]
/// async fn main() -> OxideResult<()> {
///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
///     let todo = user.todo_by_uuid(Uuid::new_v4()).set_status(TodoStatus::Progress).await?;
///     Ok(())
/// }
/// ```
/// And to get the todo, you will do same thing as above. just don't set the status or title.
/// ```rust |no_run
/// use oxide_todo_sdk::Client;
/// use oxide_todo_sdk::errors::Result as OxideResult;
/// use uuid::Uuid;
///
/// #[tokio::main]
/// async fn main() -> OxideResult<()> {
///     let user = Client::new("http://localhost:8080").login_by_token("YOUR_TOKEN");
///     let todo = user.todo_by_uuid(Uuid::new_v4()).await?;
///     Ok(())
/// }
/// ```
/// You can also delete the todo by calling `Todo::delete`:
/// ```rust |no_run
/// use oxide_todo_sdk::Client;
/// use oxide_todo_sdk::errors::Result as OxideResult;
///
/// #[tokio::main]
/// async fn main() -> OxideResult<()> {
///     let todo = Client::new("http://localhost:8080")
///         .login_by_token("YOUR_TOKEN")
///         .create_todo("My new todo")
///         .await?;
///     todo.delete().await
/// }
/// ```
/// Easy right?
#[derive(serde::Deserialize, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[must_use]
pub struct Todo {
    /// The base url.
    #[serde(skip)]
    pub(crate) base_url: String,
    #[serde(skip)]
    /// The client token.
    pub(crate) token: String,
    /// The todo uuid.
    pub(crate) uuid: Option<Uuid>,
    /// The todo title.
    pub(crate) title: Option<String>,
    /// Todo creation time.
    pub(crate) created_at: Option<u64>,
    /// Last todo update time.
    pub(crate) updated_at: Option<u64>,
    /// The todo status.
    pub(crate) status: Option<TodoStatus>,
}

impl Todo {
    /// Delete the todo. This will delete the todo from the server.
    /// If the todo has no uuid, it will return an error.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo_sdk::Client;
    /// use oxide_todo_sdk::errors::Result as OxideResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     // Create todo
    ///     let todo = Client::new("http://localhost:8080")
    ///         .login_by_token("YOUR_TOKEN")
    ///         .create_todo("My new todo")
    ///         .await?;
    ///     // Delete todo
    ///     todo.delete().await
    /// }
    /// ```
    pub async fn delete(self) -> OxideResult<()> {
        if let Some(uuid) = self.uuid {
            Endpoints::DeleteTodo {
                base_url: &self.base_url,
                token: &self.token,
                uuid: &uuid,
            }
            .await
            .map(|_| ())
        } else {
            Err(SDKError::MissingField("`uuid` is required to delete a todo.".to_owned()).into())
        }
    }

    /// Returns a UUID of the todo, if the todo is created. Else it will return `None`.
    pub fn uuid(&self) -> Option<Uuid> {
        self.uuid
    }

    /// Set the title of the todo.
    pub fn set_title(self, title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    /// Returns the title of the todo.
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    /// Set the status of the todo.
    pub fn set_status(self, status: TodoStatus) -> Self {
        Self {
            status: Some(status),
            ..self
        }
    }

    /// Returns the status of the todo.
    pub fn status(&self) -> Option<&TodoStatus> {
        self.status.as_ref()
    }

    /// Returns the creation time of the todo.
    pub fn created_at(&self) -> Option<u64> {
        self.created_at
    }

    /// Returns the last update time of the todo.
    pub fn updated_at(&self) -> Option<u64> {
        self.updated_at
    }
}

impl ToString for TodoStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::Progress => "progress",
            Self::Pending => "pending",
        }
        .to_owned()
    }
}

impl IntoFuture for Todo {
    type Output = OxideResult<Todo>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            // There is tow scenarios here:
            // 1. The todo is created, and we want to update it.
            // 2. The todo is not created, and we want to create it.
            // 3. The todo is created, and we want to get it.
            // We can know if the todo is created by checking if thay have a uuid or not.
            if let Some(uuid) = self.uuid {
                // The todo is created, we want to update it.
                // Also maybe the user want to get the todo, so we need to check if all fields are None or not.
                if self.status.is_none() && self.title.is_none() {
                    // The user want to get the todo.
                    Endpoints::GetTodo {
                        base_url: &self.base_url,
                        token: &self.token,
                        uuid: &uuid,
                    }
                    .await
                    .map(|v| Todo {
                        base_url: self.base_url,
                        token: self.token,
                        ..serde_json::from_value(v).unwrap()
                    })
                } else {
                    // The user want to update the todo.
                    Endpoints::UpdateTodo {
                        base_url: &self.base_url,
                        token: &self.token,
                        uuid: &uuid,
                        title: self.title.as_deref(),
                        status: self.status,
                    }
                    .await
                    .map(|v| Todo {
                        base_url: self.base_url,
                        token: self.token,
                        ..serde_json::from_value(v).unwrap()
                    })
                }
            } else {
                // The todo is not created, we want to create it.
                Endpoints::CreateTodo {
                    base_url: &self.base_url,
                    token: &self.token,
                    title: &self.title.ok_or_else(|| SDKError::missing_field("`title` needed to create a todo"))?,
                    status: self
                        .status
                        .ok_or_else(|| SDKError::missing_field(
                                "`status` you cannot create a todo without a status, use `Todo::set_status` to set the status"
                            ))?,
                }
                .await
                .map(|v| Todo {
                    base_url: self.base_url,
                    token: self.token,
                    ..serde_json::from_value(v).unwrap()
                })
            }
        })
    }
}
