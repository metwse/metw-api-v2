macro_rules! unwrap_fetch_one {
    ($db:expr, $query:expr) => {
        match $query.fetch_one($db).await {
            Ok(user) => Some(user),
            Err(err) => match err {
                sqlx::Error::RowNotFound => None,
                _ => panic!("{}", err),
            },
        }
    };
}

mod user_repository;

pub use user_repository::UserRepository;
