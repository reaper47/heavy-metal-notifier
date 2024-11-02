mod core;

pub mod calendar;
pub mod main;

#[derive(PartialEq)]
pub enum Page {
    About,
    Calendar,
    Contact,
    Home,
    Other,
}
