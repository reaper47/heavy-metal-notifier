mod error;

use std::{fs, io, sync::Arc};

use dotenvy::dotenv;
use tokio::{net::TcpListener, signal};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

use heavy_metal_notifier::model::{CalendarBmc, EntitiesBmc, FeedBmc};
use heavy_metal_notifier::web::AppState;
use heavy_metal_notifier::{Result, config::config, jobs, web::routes};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().with_target(false).init();

    let data_folder = "data";
    match fs::create_dir(data_folder) {
        Ok(_) => info!("Data folder created successfully!"),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            warn!("Data folder already exists.");
        }
        Err(e) => {
            eprintln!("Error creating data folder: {}", e);
            std::process::exit(1);
        }
    }

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

    let base_addr = if cfg!(target_os = "windows") {
        &config().local_server_addr()
    } else {
        &config().local_server_addr().replace("localhost", "0.0.0.0")
    };
    let listener = TcpListener::bind(base_addr).await?;
    info!("Serving at http://{base_addr}");

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
