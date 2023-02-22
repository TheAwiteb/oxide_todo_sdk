use std::future::IntoFuture;

use crate::errors::{Error, ErrorMessage};
use reqwest::RequestBuilder;
use serde_json::json;
use uuid::Uuid;

/// Add the token to the request if the token is not `None`.
pub async fn add_token(request: RequestBuilder, client_token: Option<String>) -> RequestBuilder {
    if let Some(token) = client_token {
        request.header("Authorization", format!("Bearer {token}"))
    } else {
        request
    }
}

/// Returns the response result from response.
/// This will convert the response to `T` if the response is successful. else it will return the error message.
pub async fn response_result<T: for<'a> serde::Deserialize<'a>>(
    response: reqwest::Response,
) -> Result<T, Error> {
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
    /// The get todo endpoint. This endpoint is used to get a todo by uuid. (GET)
    GetTodo {
        base_url: &'a str,
        token: &'a str,
        uuid: &'a Uuid,
    },
}

impl<'a> Endpoints<'a> {
    /// Returns the uri of the endpoint.
    pub fn uri(&self) -> String {
        match self {
            Endpoints::Register { base_url, .. } => format!("{base_url}/api/auth/register"),
            Endpoints::Login { base_url, .. } => format!("{base_url}/api/auth/login"),
            Endpoints::GetTodo { base_url, uuid, .. } => format!("{base_url}/api/todo/{uuid}"),
        }
    }
    /// Returns the method of the endpoint.
    pub fn method(&self) -> reqwest::Method {
        match self {
            Endpoints::Register { .. } => reqwest::Method::POST,
            Endpoints::Login { .. } => reqwest::Method::POST,
            Endpoints::GetTodo { .. } => reqwest::Method::GET,
        }
    }
}

impl<'a> IntoFuture for Endpoints<'a> {
    type Output = Result<serde_json::Value, Error>;
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let req = reqwest::Client::new().request(self.method(), &self.uri());
            let req = match self {
                Endpoints::Register {
                    username, password, ..
                } => req.json(&json!({
                    "username": username,
                    "password": password,
                })),
                Endpoints::Login {
                    username, password, ..
                } => req.json(&json!({
                    "username": username,
                    "password": password,
                })),
                _ => req,
            };
            // All the endpoints require the user to be logged in except the register and login endpoints.
            response_result(
                add_token(
                    req,
                    match self {
                        Endpoints::Register { .. } | Endpoints::Login { .. } => None,
                        Endpoints::GetTodo { token, .. } => Some(token.to_string()),
                    },
                )
                .await
                .send()
                .await
                .map_err(Error::ReqwestError)?,
            )
            .await
        })
    }
}
