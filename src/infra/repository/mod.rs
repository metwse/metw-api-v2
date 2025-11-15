macro_rules! unwrap_execute {
    ($db:expr, $query:expr) => {
        match $query.execute($db).await {
            Ok(data) => Some(data),
            Err(err) => {
                tracing::error!(?err, "Unexpected sqlx error");
                None
            }
        }
    };
}

macro_rules! unwrap_fetch_one {
    ($db:expr, $query:expr) => {
        match $query.fetch_one($db).await {
            Ok(data) => Some(data),
            Err(err) => match err {
                sqlx::Error::RowNotFound => None,
                _ => {
                    tracing::error!(?err, "Unexpected sqlx error");
                    None
                }
            },
        }
    };
}

macro_rules! unwrap_fetch_all {
    ($db:expr, $query:expr) => {
        match $query.fetch_all($db).await {
            Ok(data) => data,
            Err(err) => {
                tracing::error!(?err, "Unexpected sqlx error");
                vec![]
            }
        }
    };
}

#[allow(missing_docs)]
mod user_repository;

#[allow(missing_docs)]
mod post_repository;

#[allow(missing_docs)]
mod thread_repository;

pub use post_repository::PostRepository;
pub use thread_repository::ThreadRepository;
pub use user_repository::UserRepository;
