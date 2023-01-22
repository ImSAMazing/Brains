use authentication::{create_jwt, JwtDataHolder};
use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use database::{
    create_models::CreateFromRequest, get_models, password_verification::verify_password,
};
use jwt_simple::prelude::ES384KeyPair;
use shared::{
    Brain, Brainfart, BrainfartFilter, CreateBrainfartRequest, NotifyAboutMindExplosionRequest,
    NotifyAboutMindImplosionRequest, ProveOwnsBrainRequest, RegisterBrainRequest,
};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

use dotenv::dotenv;

mod authentication;
mod database;
mod error_responders;

type ConnectionPool = Pool<Postgres>;
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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let opt = Opt::parse();

    //Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    //enable console logging
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&(std::env::var("DATABASE_URL").expect("Environmental variable not set")))
        .await
        .expect("Setting up database pool failed");
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/createbrainfart", post(create_brainfarts))
        .route("/api/registerbrain", post(register_brain))
        .route("/api/getbrainfarts", get(get_some_brainfarts))
        .route("/api/loginasbrain", post(show_i_own_brain))
        .route("/api/registermindexplosion", post(register_mind_explosion))
        .route("/api/registermindimplosion", post(register_mind_implosion))
        .merge(axum_extra::routing::SpaRouter::new(
            "/assets",
            opt.static_dir,
        ))
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

async fn hello() -> impl IntoResponse {
    let key_pair = ES384KeyPair::generate();
    format!(
        "{:?}, {:?}",
        key_pair.public_key().to_pem(),
        key_pair.to_pem()
    )
}

async fn create_brainfarts(
    State(pool): State<ConnectionPool>,
    claims: JwtDataHolder,
    result: Result<Json<CreateBrainfartRequest>, JsonRejection>,
) -> impl IntoResponse {
    let jwt_information = claims.information;
    let mastermind_id = Uuid::parse_str(&jwt_information.id).unwrap();
    match result {
        Ok(Json(payload)) => match payload.create(pool, mastermind_id).await {
            Some(response) => {
                let brainfart = Brainfart::create(
                    response.uuid.to_string(),
                    payload,
                    mastermind_id.to_string(),
                    response.birthdate,
                );
                Ok((StatusCode::CREATED, Json(brainfart)))
            }
            None => Err((StatusCode::INTERNAL_SERVER_ERROR, "Create ".to_string())),
        },
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}

async fn get_some_brainfarts(
    State(pool): State<ConnectionPool>,
    _claims: JwtDataHolder,
    result: Result<Json<BrainfartFilter>, JsonRejection>,
) -> impl IntoResponse {
    let filter = if let Ok(Json(payload)) = result {
        payload
    } else {
        BrainfartFilter::default()
    };
    if let Some(brainfarts) = get_models::get_brainfarts_using_filter(pool, filter).await {
        Ok((StatusCode::OK, Json(brainfarts)))
    } else {
        Err((StatusCode::NOT_FOUND, "Error".to_string()))
    }
}

async fn register_mind_explosion(
    State(pool): State<ConnectionPool>,
    claims: JwtDataHolder,
    result: Result<Json<NotifyAboutMindExplosionRequest>, JsonRejection>,
) -> impl IntoResponse {
    match result {
        Ok(Json(payload)) => match payload
            .create(pool, Uuid::parse_str(&claims.information.id).unwrap())
            .await
        {
            Some(_) => Ok((StatusCode::CREATED, "")),
            None => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong registering the mindexplosion".to_string(),
            )),
        },
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}

async fn register_mind_implosion(
    State(pool): State<ConnectionPool>,
    claims: JwtDataHolder,
    result: Result<Json<NotifyAboutMindImplosionRequest>, JsonRejection>,
) -> impl IntoResponse {
    match result {
        Ok(Json(payload)) => match payload
            .create(pool, Uuid::parse_str(&claims.information.id).unwrap())
            .await
        {
            Some(_) => Ok((StatusCode::CREATED, "")),
            None => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong registering the mindimplosion".to_string(),
            )),
        },
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}

async fn register_brain(
    State(pool): State<ConnectionPool>,
    result: Result<Json<RegisterBrainRequest>, JsonRejection>,
) -> impl IntoResponse {
    match result {
        Ok(Json(payload)) => match payload.create(pool, Uuid::nil()).await {
            Some(response) => {
                let brain = Brain::register(
                    response.uuid.to_string(),
                    payload,
                    response.birthdate,
                    response.extra_information.unwrap(),
                );
                Ok((
                    StatusCode::CREATED,
                    Json(create_jwt(
                        Uuid::parse_str(brain.get_id()).unwrap(),
                        brain.get_name().to_string(),
                    )),
                ))
            }
            None => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong creating the brainfart!".to_string(),
            )),
        },
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}

async fn show_i_own_brain(
    State(pool): State<ConnectionPool>,
    result: Result<Json<ProveOwnsBrainRequest>, JsonRejection>,
) -> impl IntoResponse {
    match result {
        Ok(Json(result)) => {
            if let Some(id) = verify_password(pool, &result).await {
                let token = create_jwt(id, (&result.get_name()).to_string());
                Ok((StatusCode::ACCEPTED, Json(token)))
            } else {
                Ok((StatusCode::UNAUTHORIZED, Json("Unknown brain!".to_string())))
            }
        }
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}
