use std::collections::HashMap;
use time::Month;

/// Represents a collection of Releases organized by month.
pub type CalendarData = HashMap<Month, Releases>;

/// Represents a collection of heavy metal releases organized by day.
pub type Releases = HashMap<Day, Vec<Release>>;

type Day = u8;

/// Represents a heavy metal release.
#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub artist: String,
    pub album: String,
    pub metallum_info: Option<MetallumInfo>,
}

/// Represents information about a release obtained from Metal Archives (Metallum).
#[derive(Clone, Debug, PartialEq)]
pub struct MetallumInfo {
    pub artist_link: String,
    pub album_link: String,
    pub release_type: String,
    pub genre: String,
}

impl Release {
    /// Creates a new `Release` instance with the given artist and album names.
    pub fn new(artist: impl Into<String>, album: impl Into<String>) -> Self {
        let mut album: String = album.into();
        album = album.split_whitespace().collect::<Vec<_>>().join(" ");

        if let Some((before_bracket, _)) = album.split_once('[') {
            album = before_bracket.trim().to_string();
        }

        Self {
            artist: artist.into(),
            album,
            metallum_info: None,
        }
    }

    /// Adds Metallum information to the `Release`.
    pub fn with_metallum(
        mut self,
        artist_link: impl Into<String>,
        album_link: impl Into<String>,
        release_type: impl Into<String>,
        genre: impl Into<String>,
    ) -> Self {
        self.metallum_info = Some(MetallumInfo {
            artist_link: artist_link.into(),
            album_link: album_link.into(),
            release_type: release_type.into(),
            genre: genre.into(),
        });
        self
    }
}

/// Represents a calendar of heavy metal releases for a specific year.
#[derive(Debug, PartialEq)]
pub struct Calendar {
    pub year: i32,
    pub data: CalendarData,
}

impl Calendar {
    /// Creates a new `Calendar` instance for the given year.
    ///
    /// It initializes the calendar with an empty `HashMap` for each month of the year.
    pub fn new(year: i32) -> Self {
        Self {
            year,
            data: HashMap::from([
                (Month::January, HashMap::new()),
                (Month::February, HashMap::new()),
                (Month::March, HashMap::new()),
                (Month::April, HashMap::new()),
                (Month::May, HashMap::new()),
                (Month::June, HashMap::new()),
                (Month::July, HashMap::new()),
                (Month::August, HashMap::new()),
                (Month::September, HashMap::new()),
                (Month::October, HashMap::new()),
                (Month::November, HashMap::new()),
                (Month::December, HashMap::new()),
            ]),
        }
    }

    /// Adds a new heavy metal release to the calendar.
    pub fn add_release(&mut self, month: Month, day: Day, release: Release) {
        let releases = self.data.entry(month).or_default().entry(day).or_default();

        if !releases.contains(&release) {
            releases.push(release);
        }
    }

    /// Retrieves a reference to the list of heavy metal releases for a specific month and day.
    ///
    /// It is possible there are none for the given month and day.
    pub fn get_releases(&self, month: Month, day: Day) -> Option<&Vec<Release>> {
        self.data.get(&month).and_then(|map| map.get(&day))
    }

    /// Merges the current calendar with another calendar by combining their releases.
    pub fn merge(&self, other: &Self) -> Self {
        let mut calendar = Calendar::new(self.year);

        for (&month, month_releases) in &self.data {
            for (&day, day_releases) in month_releases {
                for release in day_releases {
                    calendar.add_release(month, day, release.clone());
                }
            }
        }

        for (&month, month_releases) in &other.data {
            for (&day, day_releases) in month_releases {
                for release in day_releases {
                    calendar.add_release(month, day, release.clone());
                }
            }
        }

        calendar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::scraper::test_utils::compare_calendars;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_release_new_ok() -> Result<()> {
        let artist = "Bad Omens";
        let album = "Concrete Jungle";

        let got = Release::new(artist, album);

        pretty_assertions::assert_eq!(
            got,
            Release {
                artist: artist.to_string(),
                album: album.to_string(),
                metallum_info: None,
            }
        );
        Ok(())
    }

    #[test]
    fn test_release_new_artist_with_bracket_ok() -> Result<()> {
        let artist = "Bad Omens";
        let album = "Concrete Jungle [remix album]";

        let got = Release::new(artist, album);

        pretty_assertions::assert_eq!(
            got,
            Release {
                artist: artist.to_string(),
                album: "Concrete Jungle".to_string(),
                metallum_info: None,
            }
        );
        Ok(())
    }

    #[test]
    fn test_release_new_with_metallum_ok() -> Result<()> {
        let artist = "Norther";
        let album = "Mirror of Madness";
        let metallum = MetallumInfo {
            artist_link: "https://www.metal-archives.com/bands/Norther/1815".to_string(),
            album_link: "https://www.metal-archives.com/albums/Norther/Mirror_of_Madness/18535"
                .to_string(),
            release_type: "Full-length".to_string(),
            genre: "Melodic Death/Power Metal".to_string(),
        };

        let got = Release::new(artist, album).with_metallum(
            &metallum.artist_link,
            &metallum.album_link,
            &metallum.release_type,
            &metallum.genre,
        );

        pretty_assertions::assert_eq!(
            got,
            Release {
                artist: artist.to_string(),
                album: album.to_string(),
                metallum_info: Some(metallum),
            }
        );
        Ok(())
    }

    #[test]
    fn test_default_calendar_empty_ok() -> Result<()> {
        let got = Calendar::new(2024);

        pretty_assertions::assert_eq!(
            got.data,
            CalendarData::from([
                (Month::January, HashMap::new()),
                (Month::February, HashMap::new()),
                (Month::March, HashMap::new()),
                (Month::April, HashMap::new()),
                (Month::May, HashMap::new()),
                (Month::June, HashMap::new()),
                (Month::July, HashMap::new()),
                (Month::August, HashMap::new()),
                (Month::September, HashMap::new()),
                (Month::October, HashMap::new()),
                (Month::November, HashMap::new()),
                (Month::December, HashMap::new()),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_calendar_add_release_ok() -> Result<()> {
        let mut got = Calendar::new(2024);
        let release = Release::new("Wintersun", "Time II");

        got.add_release(Month::August, 30, release.clone());

        let mut want = Calendar::new(2024);
        want.data
            .insert(Month::August, HashMap::from([(30, vec![release])]));
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn test_calendar_get_releases_ok() -> Result<()> {
        let release = Release::new("Wintersun", "Time II");
        let calendar = Calendar {
            year: 2024,
            data: CalendarData::from([(
                Month::August,
                Releases::from([(30, vec![release.clone()])]),
            )]),
        };

        let got = calendar.get_releases(Month::August, 30);

        pretty_assertions::assert_eq!(got, Some(&vec![release]));
        Ok(())
    }

    #[test]
    fn test_calendar_merge_ok() -> Result<()> {
        let calendar1 = a_calendar();
        let calendar2 = Calendar {
            year: 2025,
            data: CalendarData::from([
                (
                    Month::January,
                    Releases::from([
                        (
                            1,
                            vec![
                                Release::new("Death Cult 69", "The Way of All Flesh"),
                                Release::new("Hazzerd", "The 3rd Dimension"),
                            ],
                        ),
                        (3, vec![Release::new("Faidra", "Dies Irae")]),
                        (
                            24,
                            vec![
                                Release::new("Harakiri for the Sky", "Scorched Earth"),
                                Release::new("Wardruna", "Birna"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::February,
                    Releases::from([
                        (14, vec![Release::new("Lacuna Coil", "Sleepless Empire")]),
                        (
                            28,
                            vec![
                                Release::new("Dimman", "Consciousness"),
                                Release::new("Timecode", "La Ruptura Del Equilibrio"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::March,
                    Releases::from([(28, vec![Release::new("Arch Enemy", "Blood Dynasty")])]),
                ),
            ]),
        };

        let got = calendar1.merge(&calendar2);

        compare_calendars(got, calendar1);
        Ok(())
    }

    fn a_calendar() -> Calendar {
        Calendar {
            year: 2025,
            data: CalendarData::from([
                (
                    Month::January,
                    Releases::from([
                        (
                            1,
                            vec![
                                Release::new("Death Cult 69", "The Way of All Flesh"),
                                Release::new("Estuarine", "Corporeal Furnace"),
                                Release::new("Hazzerd", "The 3rd Dimension"),
                            ],
                        ),
                        (
                            3,
                            vec![
                                Release::new("Aeonian Sorrow", "From the Shadows"),
                                Release::new("Faidra", "Dies Irae"),
                            ],
                        ),
                        (
                            10,
                            vec![Release::new("The Halo Effect", "March of the Unheard")],
                        ),
                        (
                            17,
                            vec![
                                Release::new("Grave Digger", "Bone Collector"),
                                Release::new("Tokyo Blade", "Time Is the Fire"),
                                Release::new("Pestilent Scars", "Meadows of Misfortune"),
                            ],
                        ),
                        (
                            24,
                            vec![
                                Release::new("Harakiri for the Sky", "Scorched Earth"),
                                Release::new(
                                    "Avatarium",
                                    "Between You, God, the Devil and the Dead",
                                ),
                                Release::new("Wardruna", "Birna"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::February,
                    Releases::from([
                        (
                            14,
                            vec![
                                Release::new("Atlas Ashes", "New World"),
                                Release::new("Lacuna Coil", "Sleepless Empire"),
                            ],
                        ),
                        (
                            21,
                            vec![Release::new(
                                "Defiled Serenity",
                                "Within the Slumber of the Mind",
                            )],
                        ),
                        (
                            28,
                            vec![
                                Release::new("Dimman", "Consciousness"),
                                Release::new("Timecode", "La Ruptura Del Equilibrio"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::March,
                    Releases::from([(28, vec![Release::new("Arch Enemy", "Blood Dynasty")])]),
                ),
            ]),
        }
    }
}
