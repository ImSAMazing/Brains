use axum::{
    extract::{rejection::JsonRejection, State},
    http::{response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use std::str::FromStr;
use std::{
    error::Error,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use rand::{thread_rng, Rng};
use serde::Deserialize;
use serde::Serialize;
use shared::Brainfart;
use shared::CreateBrainfart;

use sqlx::{postgres::PgPoolOptions, Pool};

mod error_responders;
// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    ///set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

type ConnectionPool = Pool<Postgres>;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    //Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    //enable console logging
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://webspelletjes:webspelletjes@localhost/webspelletjes")
        .await
        .expect("Setting up database pool failed");
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/createbrainfart", post(create_brain_fart))
        .merge(axum_extra::routing::SpaRouter::new(
            "/assets",
            opt.static_dir,
        ))
        .fallback(handle_404)
        .with_state(pool)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
async fn hello() -> impl IntoResponse {
    "hello from server!"
}

async fn create_brain_fart(
    result: Result<Json<CreateBrainfart>, JsonRejection>,
    State(pool): State<ConnectionPool>,
) -> impl IntoResponse {
    match result {
        Ok(Json(payload)) => {
            let brainfart = Brainfart::create_from_request(thread_rng().gen(), payload);
            Ok((StatusCode::CREATED, Json(brainfart)))
        }
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}
