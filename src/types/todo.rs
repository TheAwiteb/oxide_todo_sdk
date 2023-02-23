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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    /// The todo are completed.
    Completed,
    /// The todo are in progress.
    Progress,
    /// The todo are pending.
    Pending,
    /// The todo are canceled.
    Canceled,
}

/// The todo struct.
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

impl IntoFuture for Todo {
    type Output = OxideResult<Todo>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            // There is tow scenarios here:
            // 1. The todo is created, and we want to update it.
            // 2. The todo is not created, and we want to create it.
            // We can know if the todo is created by checking if thay have a uuid or not.
            if let Some(_uuid) = self.uuid {
                // The todo is created, we want to update it.
                unimplemented!()
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
