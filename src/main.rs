mod error;

use dotenv::dotenv;
use std::env;
use tokio::{net::TcpListener, signal};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use heavy_metal_notifier::{jobs, web::routes, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Fetching and storing calendar");
    jobs::update_calendar().await?;

    info!("Scheduling jobs");
    let sched = JobScheduler::new().await?;
    sched
        .add(
            // At 12:00 AM, on day 1 of the month
            Job::new_async("0 0 0 1 * * *", |_uuid, _l| {
                Box::pin(async move {
                    info!("Updating calendar");
                    if let Err(err) = jobs::update_calendar().await {
                        error!("Error updating calendar: {err}")
                    };
                    info!("Calendar updated")
                })
            })?,
        )
        .await?;
    sched.shutdown_on_ctrl_c();
    sched.start().await?;

    let mut addr = String::from("localhost:");
    addr.push_str(&env::var("SERVICE_PORT").unwrap());

    let listener = TcpListener::bind(&addr).await?;
    info!("Serving at http://{addr}");

    axum::serve(listener, routes().await.unwrap())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

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
