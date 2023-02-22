use uuid::Uuid;

/// The todo status.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, serde::Deserialize, Default)]
pub struct Todo {
    /// The base url.
    #[serde(skip)]
    pub(crate) base_url: String,
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
