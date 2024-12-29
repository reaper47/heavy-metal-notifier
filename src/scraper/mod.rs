pub mod client;
pub mod metallum;
pub mod wiki;

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::calendar::Calendar;

    pub fn compare_calendars(got: Calendar, want: Calendar) {
        for (month, releases) in want.data.iter() {
            match got.data.get(month) {
                Some(got_releases) => {
                    for (day, want_day) in releases.iter() {
                        let got_day = match got_releases.get(day) {
                            Some(day) => day,
                            None => {
                                eprintln!("Missing day {:?} {day}", month);
                                std::process::exit(1);
                            }
                        };
                        pretty_assertions::assert_eq!(got_day, want_day, "{:?} {}", month, day);
                    }
                }
                None => {
                    eprintln!("Should have had month `{:?}`", month);
                    std::process::exit(1);
                }
            }
        }
    }
}
