use autentisering::producera_jwt;
use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use databank::{create_models::ProduceraFrånFörfrågan, losenord_verifiera::verifiera_lösenord};
use shared::{
    DemonstreraBesittarHjärnaFörfrågon, Fantasiforster, Hjärna, ProduceraFantasiforsterFörfrågan,
    RegistreraHjärnaFörfrågan,
};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

use dotenv::dotenv;

mod autentisering;
mod databank;
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
        .route("/api/createbrainfart", post(producera_fantasiforster))
        .route("/api/registerbrain", post(registrera_hjärna))
        .route("/api/loginasbrain", post(demonstrera_jag_besittar_hjärnan))
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

async fn producera_fantasiforster(
    State(pool): State<ConnectionPool>,
    result: Result<Json<ProduceraFantasiforsterFörfrågan>, JsonRejection>,
) -> impl IntoResponse {
    let uppfinnare_id = Uuid::parse_str("5f4664af-31e7-4d71-a3ad-f9a990b22212").unwrap();
    match result {
        Ok(Json(payload)) => match payload.producera(pool, uppfinnare_id).await {
            Some(reaktion) => {
                let forster = Fantasiforster::producera(
                    reaktion.uuid,
                    payload,
                    uppfinnare_id,
                    reaktion.födelsedag,
                );
                Ok((StatusCode::CREATED, Json(forster)))
            }
            None => Err((StatusCode::INTERNAL_SERVER_ERROR, "Producera ".to_string())),
        },
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}

async fn registrera_hjärna(
    State(pool): State<ConnectionPool>,
    result: Result<Json<RegistreraHjärnaFörfrågan>, JsonRejection>,
) -> impl IntoResponse {
    match result {
        Ok(Json(payload)) => match payload.producera(pool, Uuid::nil()).await {
            Some(reaktion) => {
                let hjärna = Hjärna::registrera(
                    reaktion.uuid,
                    payload,
                    reaktion.födelsedag,
                    reaktion.tillägen_information.unwrap(),
                );
                Ok((StatusCode::CREATED, Json(hjärna)))
            }
            None => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong creating the brainfart!".to_string(),
            )),
        },
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}

async fn demonstrera_jag_besittar_hjärnan(
    State(pool): State<ConnectionPool>,
    result: Result<Json<DemonstreraBesittarHjärnaFörfrågon>, JsonRejection>,
) -> impl IntoResponse {
    match result {
        Ok(Json(result)) => {
            if let Some(success_status) = verifiera_lösenord(pool, &result).await {
                let token = producera_jwt((&result.skaffa_mig_ditt_namn()).to_string());
                if success_status {
                    Ok((StatusCode::ACCEPTED, Json(token)))
                } else {
                    Ok((
                        StatusCode::UNAUTHORIZED,
                        Json("Invalid password!".to_string()),
                    ))
                }
            } else {
                Ok((StatusCode::UNAUTHORIZED, Json("Unknown brain!".to_string())))
            }
        }
        Err(err) => Err(error_responders::post_error_responder(err)),
    }
}
