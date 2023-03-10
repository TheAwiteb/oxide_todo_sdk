use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use crate::{
    errors::{Error, ErrorMessage, Result as OxideResult},
    types::{TodoStatus, Todos},
};
use reqwest::RequestBuilder;
use serde_json::json;
use uuid::Uuid;

/// Add the token to the request if the token is not `None`.
pub async fn add_token(
    request: RequestBuilder,
    client_token: Option<impl AsRef<str>>,
) -> RequestBuilder {
    if let Some(token) = client_token {
        request.header("Authorization", format!("Bearer {}", token.as_ref()))
    } else {
        request
    }
}

/// Returns the response result from response.
/// This will convert the response to `T` if the response is successful. else it will return the error message.
pub async fn response_result<T: for<'a> serde::Deserialize<'a>>(
    response: reqwest::Response,
) -> OxideResult<T> {
    if response.status().is_success() {
        response.json::<T>().await.map_err(From::from)
    } else {
        Err(response
            .json::<ErrorMessage>()
            .await
            .map_err(Error::ReqwestError)?
            .into())
    }
}

/// A list of all the endpoints of the server. With it's metadata.
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Endpoints<'a> {
    /// The register endpoint. This endpoint is used to register a new user. (POST)
    Register {
        base_url: &'a str,
        username: &'a str,
        password: &'a str,
    },
    /// The login endpoint. This endpoint is used to login a user. (POST)
    Login {
        base_url: &'a str,
        username: &'a str,
        password: &'a str,
    },
    /// The revoke token endpoint. This endpoint is used to revoke a token. (PATCH)
    RevokeToken { base_url: &'a str, token: &'a str },
    /// The get todo endpoint. This endpoint is used to get a todo by uuid. (GET)
    GetTodo {
        base_url: &'a str,
        token: &'a str,
        uuid: &'a Uuid,
    },
    /// The create todo endpoint. This endpoint is used to create a new todo. (POST)
    CreateTodo {
        base_url: &'a str,
        token: &'a str,
        title: &'a str,
        status: TodoStatus,
    },
    /// The update todo endpoint. This endpoint is used to update a todo. (PUT)
    /// Note: If you don't want to update the title or status, set it to `None`.
    UpdateTodo {
        base_url: &'a str,
        token: &'a str,
        uuid: &'a Uuid,
        title: Option<&'a str>,
        status: Option<TodoStatus>,
    },
    /// The delete todo endpoint. This endpoint is used to delete a todo. (DELETE)
    DeleteTodo {
        base_url: &'a str,
        token: &'a str,
        uuid: &'a Uuid,
    },
    /// The get todos endpoint. This endpoint is used to get all the todos. (GET)
    GetTodos(&'a Todos),
    /// The delete todos endpoint. This endpoint is used to delete all the todos. (DELETE)
    DeleteTodos { base_url: &'a str, token: &'a str },
}

impl<'a> Endpoints<'a> {
    /// Returns the uri of the endpoint.
    pub fn uri(&self) -> String {
        use Endpoints::*;
        match self {
            Register { base_url, .. } => format!("{base_url}/api/auth/register"),
            Login { base_url, .. } => format!("{base_url}/api/auth/login"),
            RevokeToken { base_url, .. } => format!("{base_url}/api/auth/revoke"),
            CreateTodo { base_url, .. } | DeleteTodos { base_url, .. } => {
                format!("{base_url}/api/todos")
            }
            GetTodos(Todos { base_url, .. }) => format!("{base_url}/api/todos"),
            GetTodo { base_url, uuid, .. }
            | UpdateTodo { base_url, uuid, .. }
            | DeleteTodo { base_url, uuid, .. } => format!("{base_url}/api/todos/{uuid}"),
        }
    }
    /// Returns the method of the endpoint.
    pub fn method(&self) -> reqwest::Method {
        use Endpoints::*;
        match self {
            Register { .. } | Login { .. } | CreateTodo { .. } => reqwest::Method::POST,
            RevokeToken { .. } => reqwest::Method::PATCH,
            UpdateTodo { .. } => reqwest::Method::PUT,
            GetTodo { .. } | GetTodos { .. } => reqwest::Method::GET,
            DeleteTodo { .. } | DeleteTodos { .. } => reqwest::Method::DELETE,
        }
    }
    /// Returns the user token if the endpoint requires the user to be logged in.
    /// This will return `None` if the endpoint does not require the user to be logged in.
    pub fn token(&self) -> Option<&str> {
        use Endpoints::*;
        match self {
            Register { .. } | Login { .. } => None,
            GetTodos(Todos { token, .. }) => Some(token),
            GetTodo { token, .. }
            | CreateTodo { token, .. }
            | UpdateTodo { token, .. }
            | DeleteTodo { token, .. }
            | DeleteTodos { token, .. }
            | RevokeToken { token, .. } => Some(token),
        }
    }

    /// Add a body to the request if the endpoint requires a body.
    pub fn add_body(&self, req: RequestBuilder) -> RequestBuilder {
        match self {
            Self::Register {
                username, password, ..
            }
            | Self::Login {
                username, password, ..
            } => req.json(&json!({
                "username": username,
                "password": password,
            })),
            Self::CreateTodo { title, status, .. } => req.json(&json!({
                "title": title,
                "status": status,
            })),
            Self::UpdateTodo { title, status, .. } => req.json(&json!({
                "title": title,
                "status": status,
            })),
            _ => req,
        }
    }

    /// Add a query to the request if the endpoint requires a query.
    /// This will return the request builder with the query added.
    pub fn add_query(&self, req: RequestBuilder) -> RequestBuilder {
        match self {
            Self::GetTodos(Todos {
                limit,
                offset,
                order,
                order_by,
                status,
                title,
                ..
            }) => {
                let mut req = req.query(&[
                    ("limit", limit.to_string()),
                    ("offset", offset.to_string()),
                    ("order", order.to_string()),
                    ("order_by", order_by.to_string()),
                ]);
                if let Some(status) = status {
                    req = req.query(&[("status", status.to_string())]);
                };
                if let Some(title) = title {
                    req = req.query(&[("title", title.to_string())]);
                };
                req
            }
            _ => req,
        }
    }
}

impl<'a> IntoFuture for Endpoints<'a> {
    type Output = OxideResult<serde_json::Value>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let req = self.add_body(reqwest::Client::new().request(self.method(), self.uri()));
            // All the endpoints require the user to be logged in except the register and login endpoints.
            response_result(
                add_token(self.add_query(req), self.token())
                    .await
                    .send()
                    .await
                    .map_err(Error::ReqwestError)?,
            )
            .await
        })
    }
}
