use dotenv::dotenv;
use metw_api_v2::{app::create_router, state::{self, Config}};
use std::{env, net::SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=error", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let socket_address: SocketAddr = env::var("HOST")
        .unwrap_or(String::from("127.0.0.1:1186"))
        .parse()
        .unwrap();

    let listener = tokio::net::TcpListener::bind(&socket_address)
        .await
        .unwrap();

    let config = Config::from_env(Some(".env")).unwrap();

    axum::serve(
        listener,
        create_router(state::bootstrap(config).await).await
    )
    .await
    .unwrap();
}
