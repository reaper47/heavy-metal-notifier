use std::collections::HashMap;

use time::Month;

pub type CalendarData = HashMap<Month, Releases>;

type Day = u8;

pub type Releases = HashMap<Day, Vec<Release>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub artist: String,
    pub album: String,
    pub metallum_info: Option<MetallumInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetallumInfo {
    pub artist_link: String,
    pub album_link: String,
    pub release_type: String,
    pub genre: String,
}

impl Release {
    pub fn new(artist: impl Into<String>, album: impl Into<String>) -> Self {
        let mut album: String = album.into();
        album = album.split_whitespace().collect::<Vec<&str>>().join(" ");
        if album.contains("[") {
            album = album.split_once('[').unwrap().0.to_string();
        }

        Self {
            artist: artist.into(),
            album,
            metallum_info: None,
        }
    }

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

#[derive(Debug, PartialEq)]
pub struct Calendar {
    pub year: i32,
    pub data: CalendarData,
}

impl Calendar {
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

    pub fn add_release(&mut self, month: Month, day: Day, release: Release) {
        let releases = self.data.entry(month).or_default().entry(day).or_default();

        if !releases.iter().any(|r| *r == release) {
            releases.push(release);
        }
    }

    pub fn get_releases(&self, month: Month, day: Day) -> Option<&Vec<Release>> {
        self.data.get(&month).and_then(|map| map.get(&day))
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut calendar = Calendar::new(self.year);
        self.data.iter().for_each(|(&month, month_releases)| {
            month_releases.iter().for_each(|(&day, day_releases)| {
                day_releases
                    .iter()
                    .for_each(|release| calendar.add_release(month, day, release.clone()))
            })
        });
        other.data.iter().for_each(|(&month, month_releases)| {
            month_releases.iter().for_each(|(&day, day_releases)| {
                day_releases
                    .iter()
                    .for_each(|release| calendar.add_release(month, day, release.clone()))
            })
        });
        calendar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::scraper::test_utils::compare_calendars;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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
        let calendar1 = Calendar {
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
        };
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

        let want = Calendar {
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
        };
        compare_calendars(got, want);
        Ok(())
    }
}
