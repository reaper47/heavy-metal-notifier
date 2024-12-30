mod core;

pub mod calendar;
pub mod main;

/// Represents a page within an application.
#[derive(PartialEq)]
pub enum Page {
    About,
    Calendar,
    Contact,
    Home,
    Other,
}
