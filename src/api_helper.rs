use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use crate::errors::{Error, ErrorMessage, Result as OxideResult};
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
            Self::Register { base_url, .. } => format!("{base_url}/api/auth/register"),
            Self::Login { base_url, .. } => format!("{base_url}/api/auth/login"),
            Self::GetTodo { base_url, uuid, .. } => format!("{base_url}/api/todo/{uuid}"),
        }
    }
    /// Returns the method of the endpoint.
    pub fn method(&self) -> reqwest::Method {
        use Endpoints::*;
        match self {
            Register { .. } | Login { .. } => reqwest::Method::POST,
            GetTodo { .. } => reqwest::Method::GET,
        }
    }
    /// Returns the user token if the endpoint requires the user to be logged in.
    /// This will return `None` if the endpoint does not require the user to be logged in.
    pub fn token(&self) -> Option<&str> {
        use Endpoints::*;
        match self {
            Register { .. } | Login { .. } => None,
            GetTodo { token, .. } => Some(token),
        }
    }
}

impl<'a> IntoFuture for Endpoints<'a> {
    type Output = OxideResult<serde_json::Value>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + 'a>>;

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
                add_token(req, self.token())
                    .await
                    .send()
                    .await
                    .map_err(Error::ReqwestError)?,
            )
            .await
        })
    }
}
