use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    // Env
    MissingEnv(&'static str),

    // Externals
    #[from]
    Env(std::env::VarError),
    #[from]
    Io(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
