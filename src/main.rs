use anyhow::Result;
use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use chrono::{SecondsFormat, Utc};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

const STARTUP_SECONDS: u64 = 8;
const READY_SECONDS: u64 = 20;
const LOADING_SECONDS: u64 = 10;

struct AppState {
    start: Instant,
    ready: AtomicBool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let state = Arc::new(AppState {
        start: Instant::now(),
        ready: AtomicBool::new(false),
    });

    // Simulate: after startup -> READY -> (READY 20s -> LOADING 10s) repeat
    let cycle_state = Arc::clone(&state);
    tokio::spawn(async move {
        // Sleep exactly until startup completes.
        let startup_dur = Duration::from_secs(STARTUP_SECONDS);
        let elapsed = cycle_state.start.elapsed();
        if elapsed < startup_dur {
            sleep(startup_dur - elapsed).await;
        }

        loop {
            cycle_state.ready.store(true, Ordering::SeqCst);
            sleep(Duration::from_secs(READY_SECONDS)).await;

            cycle_state.ready.store(false, Ordering::SeqCst);
            sleep(Duration::from_secs(LOADING_SECONDS)).await;
        }
    });

    // Logging task
    let log_state = Arc::clone(&state);
    tokio::spawn(async move {
        let mut counter: u64 = 0;
        loop {
            sleep(Duration::from_secs(1)).await;
            let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

            if !startup_complete(&log_state) {
                println!(r#"{{"timestamp":"{}","phase":"starting up"}}"#, timestamp);
                continue;
            }

            let phase = if log_state.ready.load(Ordering::SeqCst) {
                counter += 1;
                "ready"
            } else {
                "loading"
            };

            println!(
                r#"{{"timestamp":"{}","phase":"{}","counter":"{}"}}"#,
                timestamp, phase, counter
            );
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
    // Liveness stays OK through startup + reload cycles
    "ok"
}

async fn ready(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if !startup_complete(&state) {
        return (StatusCode::SERVICE_UNAVAILABLE, "starting");
    }

    if state.ready.load(Ordering::SeqCst) {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "loading")
    }
}

async fn startup(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Startup probe should only care about initial boot, not later reload cycles
    if startup_complete(&state) {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "starting")
    }
}

fn startup_complete(state: &AppState) -> bool {
    state.start.elapsed() >= Duration::from_secs(STARTUP_SECONDS)
}
