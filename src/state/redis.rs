use redis::aio::ConnectionManager;

/// Redis connection wrapper
#[derive(Clone)]
pub struct Redis {
    con: ConnectionManager,
}

impl Redis {
    /// Creates a new [`ConnectionManager`] wrapped with [`Redis`].
    pub async fn new(con_url: &str) -> Redis {
        let client = redis::Client::open(con_url).unwrap();
        let con = client.get_connection_manager().await.unwrap();
        tracing::trace!("Connected to the Redis database");

        Redis { con }
    }

    /// Returns a clone of underlying connection manager.
    pub fn client(&self) -> ConnectionManager {
        self.con.clone()
    }
}
