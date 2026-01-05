use anyhow::Result;
use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use chrono::{SecondsFormat, Utc};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const STARTUP_SECONDS: u64 = 8;

struct AppState {
    start: Instant,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let state = Arc::new(AppState {
        start: Instant::now(),
    });

    let log_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut counter: u64 = 0;
        loop {
            sleep(Duration::from_secs(1)).await;
            let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
            if log_state.start.elapsed() < Duration::from_secs(STARTUP_SECONDS) {
                println!(
                    r#"{{"timestamp":"{}","message":"starting up..."}}"#,
                    timestamp
                );
            } else {
                counter += 1;
                println!(r#"{{"timestamp":"{}","counter":"{}"}}"#, timestamp, counter);
            }
        }
    });

    let app = Router::new()
        .route("/healthz/live", get(live))
        .route("/healthz/ready", get(ready))
        .route("/healthz/startup", get(startup))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn live() -> &'static str {
    "ok"
}

async fn ready(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if startup_complete(&state) {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "starting")
    }
}

async fn startup(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if startup_complete(&state) {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "starting")
    }
}

fn startup_complete(state: &AppState) -> bool {
    state.start.elapsed() >= Duration::from_secs(STARTUP_SECONDS)
}
