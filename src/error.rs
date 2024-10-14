use derive_more::derive::From;

/// Heavy Metal Notifier main Result type alias (with heavy_metal_notifier::Error).
#[allow(unused)]
pub type Result<T> = core::result::Result<T, Error>;

/// Heavy Metal Notifier main error.
#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    CalendarUpdateFail,
    DbFeed,
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    MissingEnv(&'static str),
    NoItem,
    ParseFail,
    RequestFail,
    ScraperFail,

    // Externals
    #[from]
    Diesel(diesel::result::Error),
    #[from]
    Env(std::env::VarError),
    #[from]
    Job(tokio_cron_scheduler::JobSchedulerError),
    #[from]
    Io(std::io::Error),
    #[from]
    Reqwest(reqwest::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
