mod error;

use dotenvy::dotenv;
use std::sync::Arc;
use tokio::{net::TcpListener, signal};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use heavy_metal_notifier::model::{CalendarBmc, EntitiesBmc, FeedBmc};
use heavy_metal_notifier::web::AppState;
use heavy_metal_notifier::{config::config, jobs, web::routes, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_target(false).init();
    config();

    info!("Fetching and storing calendar");
    jobs::update_calendar(CalendarBmc).await?;

    info!("Scheduling jobs");
    let sched = JobScheduler::new().await?;
    sched
        .add(Job::new_async("0 0 0 * * 0", |_uuid, _l| {
            Box::pin({
                async move {
                    info!("Updating calendar");
                    if let Err(err) = jobs::update_calendar(CalendarBmc).await {
                        error!("Error updating calendar: {err}")
                    };
                    info!("Calendar updated")
                }
            })
        })?)
        .await?;
    sched.shutdown_on_ctrl_c();
    sched.start().await?;

    let base_addr = &config().local_server_addr();
    let listener = TcpListener::bind(base_addr).await?;
    info!("Serving at {base_addr}");

    let router = routes().await?.with_state(AppState::new(
        Arc::new(CalendarBmc),
        Arc::new(EntitiesBmc),
        Arc::new(FeedBmc),
    ));

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler.")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
