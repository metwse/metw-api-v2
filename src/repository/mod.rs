macro_rules! unwrap_fetch_one {
    ($db:expr, $query:expr) => {
        match $query.fetch_one($db).await {
            Ok(data) => Some(data),
            Err(err) => match err {
                sqlx::Error::RowNotFound => None,
                _ => {
                    tracing::error!(?err, "Unexcepted sqlx error");
                    None
                },
            },
        }
    };
}

macro_rules! unwrap_fetch_all {
    ($db:expr, $query:expr) => {
        match $query.fetch_all($db).await {
            Ok(data) => data,
            Err(err) => {
                tracing::error!(?err, "Unexcepted sqlx error");
                vec![]
            },
        }
    };
}

mod user_repository;

mod post_repository;

pub use user_repository::UserRepository;

pub use post_repository::PostRepository;
