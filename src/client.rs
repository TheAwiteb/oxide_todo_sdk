use crate::{api_helper::Endpoints, errors::Result as OxideResult, types::User};

/// A client for the server.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Client {
    /// The base url of the server.
    base_url: String,
}

impl Client {
    /// Create a new client with the given base url.
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            base_url: base_url.as_ref().to_owned(),
        }
    }

    /// Login the user with username and password.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo::Client;
    /// use oxide_todo::errors::Result as OxideResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///    let client = Client::new("http://localhost:8080");
    ///   let user = client.login("username", "password").await?;
    ///   // Now you can use the user to create todos, etc.
    ///  Ok(())
    /// }
    /// ```
    pub async fn login(
        &self,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> OxideResult<User> {
        Endpoints::Login {
            base_url: &self.base_url,
            username: username.as_ref(),
            password: password.as_ref(),
        }
        .await
        .map(|v| User {
            base_url: self.base_url.clone(),
            ..serde_json::from_value(v).unwrap()
        })
    }
    /// Register the user with username and password.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo::Client;
    /// use oxide_todo::errors::Result as OxideResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> OxideResult<()> {
    ///     let client = Client::new("http://localhost:8080");
    ///     let user = client.register("username", "password").await?;
    ///     // Now you can use the user to create todos, etc.
    ///     Ok(())
    /// }
    /// ```
    pub async fn register(
        &self,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> OxideResult<User> {
        Endpoints::Register {
            base_url: &self.base_url,
            username: username.as_ref(),
            password: password.as_ref(),
        }
        .await
        .map(|v| User {
            base_url: self.base_url.clone(),
            ..serde_json::from_value(v).unwrap()
        })
    }

    /// Login the user by token.
    /// This will not make a request to the server. It will just create a new user with the given token.
    /// ### Example
    /// ```rust |no_run
    /// use oxide_todo::Client;
    /// let client = Client::new("http://localhost:8080");
    /// let user = client.login_by_token("YOUR_TOKEN");
    /// // Now you can use the user to create todos, etc.
    /// ```
    pub fn login_by_token(&self, token: impl AsRef<str>) -> User {
        User {
            base_url: self.base_url.clone(),
            name: None,
            token: token.as_ref().to_owned(),
        }
    }
}
