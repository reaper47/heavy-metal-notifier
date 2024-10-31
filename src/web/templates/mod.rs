mod core;

pub mod calendar;
pub mod general;
pub mod main;

#[derive(PartialEq)]
pub enum Page {
    About,
    Calendar,
    Contact,
    Home,
    Other,
}
