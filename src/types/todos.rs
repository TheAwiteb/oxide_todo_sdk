use super::{Todo, TodoStatus};
use crate::{api_helper::Endpoints, errors::Result as OxideResult};
use serde::{Deserialize, Serialize};
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

/// The todo order, this is used to order the todos. (`newer`, `older`)
#[derive(Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(rename_all = "lowercase")]
pub enum TodoOrder {
    /// The newer order. This means that the newest todos are first. (default)
    #[default]
    Newer,
    /// The older order. This means that the oldest todos are first.
    Older,
}

/// The todo order by, this is used to order the todos by. (`created_at`, `updated_at`)
#[derive(Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(rename_all = "lowercase")]
pub enum TodoOrderBy {
    /// Order by created at. (default)
    #[default]
    CreatedAt,
    /// Order by updated at.
    UpdatedAt,
}

/// The Todos type. This type is used to represent a list of todos.
/// ### Example
/// See  [todos example](https://github.com/TheAwiteb/oxide_todo_sdk/blob/master/examples/todos.rs).
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Todos {
    /// Base url of the server.
    pub(crate) base_url: String,
    /// The client token.
    pub(crate) token: String,
    /// The limit of the todos.
    /// This is the maximum amount of todos that can be in the list.
    pub(crate) limit: usize,
    /// The offset of the todos.
    /// This is the amount of todos that are skipped.
    pub(crate) offset: usize,
    /// The total amount of todos.
    /// This is the total amount of todos in the database with the given filter.
    pub(crate) total: usize,
    /// The order filter of the todos. (newer, older)
    pub(crate) order: TodoOrder,
    /// Todo order by filter, (created_at, updated_at)
    pub(crate) order_by: TodoOrderBy,
    /// Status filter of the todos.
    pub(crate) status: Option<TodoStatus>,
    /// Title filter of the todos.
    pub(crate) title: Option<String>,
}

impl Todos {
    /// Create a new Todos type.
    pub(crate) fn new(base_url: impl AsRef<str>, token: impl AsRef<str>) -> Self {
        Self {
            base_url: base_url.as_ref().to_owned(),
            token: token.as_ref().to_owned(),
            limit: 10,
            offset: 0,
            total: 0,
            order: TodoOrder::default(),
            order_by: TodoOrderBy::default(),
            status: None,
            title: None,
        }
    }

    /// Set the limit of the todos. (default: 10)
    /// This is the maximum amount of todos that can be in the list.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set the offset of the todos. (default: 0)
    /// This is the amount of todos that are skipped.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Set the order of the todos. (default: [`TodoOrder::Newer`]])
    /// This is the order of the todos.
    pub fn order(mut self, order: TodoOrder) -> Self {
        self.order = order;
        self
    }

    /// Set the order by of the todos. (default: [`TodoOrderBy::CreatedAt`]])
    /// This is the order by of the todos.
    pub fn order_by(mut self, order_by: TodoOrderBy) -> Self {
        self.order_by = order_by;
        self
    }

    /// Set the status of the todos.
    /// This is the status of the todos.
    pub fn status(mut self, status: TodoStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the title of the todos.
    /// This is the title of the todos.
    pub fn title(mut self, title: impl AsRef<str>) -> Self {
        self.title = Some(title.as_ref().to_owned());
        self
    }
}

impl ToString for TodoOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Newer => "newer".to_owned(),
            Self::Older => "older".to_owned(),
        }
    }
}

impl ToString for TodoOrderBy {
    fn to_string(&self) -> String {
        match self {
            Self::CreatedAt => "created_at".to_owned(),
            Self::UpdatedAt => "updated_at".to_owned(),
        }
    }
}

impl IntoFuture for Todos {
    type Output = OxideResult<Vec<Todo>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            Endpoints::GetTodos(&self)
                .await
                .map(|d| serde_json::from_value(d["data"].clone()).unwrap())
        })
    }
}
