use std::str::FromStr;

use scraper::{Html, Selector};
use serde::Deserialize;
use tracing::info;

use super::client::Client;
use crate::{
    calendar::{Calendar, Release},
    error::{Error, Result},
};

#[derive(Deserialize)]
pub struct MetallumReleases {
    #[serde(rename = "iTotalRecords")]
    pub total_records: i32,
    #[serde(rename = "iTotalDisplayRecords")]
    pub total_display_records: i32,
    #[serde(rename = "aaData")]
    pub data: Vec<Vec<String>>,
}

pub struct MetallumReleaseParts {
    artist: String,
    artist_link: String,
    album: String,
    album_link: String,
    release_type: String,
    genre: String,
    release_date: time::Date,
}

impl MetallumReleaseParts {
    fn from_release(release: Vec<String>) -> Result<Self> {
        let selector = Selector::parse("a").map_err(|_| Error::ScraperFail)?;

        let artists = Html::parse_fragment(release.first().ok_or(Error::NoItem)?)
            .select(&selector)
            .map(|el| {
                let artist = el.text().collect::<Vec<_>>().join("");
                let artist_link = el.value().attr("href").unwrap_or("").to_string();
                (artist, artist_link)
            })
            .collect::<Vec<_>>();
        let artist = artists
            .clone()
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<_>>()
            .join(" / ");
        let artist_link = artists.first().cloned().unwrap().1;

        let (album, album_link) = Html::parse_fragment(release.get(1).ok_or(Error::NoItem)?)
            .select(&selector)
            .map(|el| {
                let album = el.text().collect::<Vec<_>>().join("");
                let album_link = el.value().attr("href").unwrap_or("").to_string();
                (album, album_link)
            })
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .unwrap();

        let release_date = release
            .get(4)
            .ok_or(Error::NoItem)?
            .to_string()
            .replace("nd", "")
            .replace("st", "")
            .replace("rd", "")
            .replace("th", "")
            .replace(",", "");
        let mut release_date = release_date.split_whitespace();
        let month = release_date.next().ok_or(Error::ParseFail)?;
        let month = time::Month::from_str(month).map_err(|_| Error::ParseFail)?;
        let day = release_date
            .next()
            .ok_or(Error::ParseFail)?
            .parse()
            .map_err(|_| Error::ParseFail)?;
        let year = release_date
            .next()
            .ok_or(Error::ParseFail)?
            .parse()
            .map_err(|_| Error::ParseFail)?;
        let release_date =
            time::Date::from_calendar_date(year, month, day).map_err(|_| Error::ParseFail)?;

        Ok(Self {
            artist,
            artist_link,
            album,
            album_link,
            release_type: release.get(2).unwrap_or(&String::new()).to_string(),
            genre: release.get(3).unwrap_or(&String::new()).to_string(),
            release_date,
        })
    }
}

pub async fn scrape(client: &impl Client, year: i32) -> Result<Calendar> {
    info!("Scraping The Metal Archives");
    let mut calendar = Calendar::new(year);
    let mut page = 0;

    loop {
        info!("Fetching entries {} to {}", page * 100, page * 100 + 100);

        match client.fetch_metallum(page).await {
            Some(releases) => {
                releases
                    .data
                    .iter()
                    .filter_map(|release| MetallumReleaseParts::from_release(release.to_vec()).ok())
                    .filter(|release| release.release_date.year() == year)
                    .for_each(|parts| {
                        calendar.add_release(
                            parts.release_date.month(),
                            parts.release_date.day(),
                            Release::new(parts.artist, parts.album).with_metallum(
                                parts.artist_link,
                                parts.album_link,
                                parts.release_type,
                                parts.genre,
                            ),
                        )
                    });
            }
            None => break,
        }

        page += 1;
    }

    info!("Calendar created");
    Ok(calendar)
}

#[cfg(test)]
mod tests {
    use time::Month;

    use crate::{
        calendar::{CalendarData, Releases},
        scraper::{client::tests::MockClient, test_utils::compare_calendars},
    };

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_2024_calendar_ok() -> Result<()> {
        let client = MockClient::new();

        let got = scrape(&client, 2024).await?;

        let want = Calendar {
            year: 2024,
            data: CalendarData::from([
                (Month::October, Releases::from([
					(9, vec![
						Release::new("Threshold", "Concert in London | London Astoria 2 | 1999").with_metallum("https://www.metal-archives.com/bands/Threshold/1114", "https://www.metal-archives.com/albums/Threshold/Concert_in_London_%7C_London_Astoria_2_%7C_1999/1271070", "Live album", "Progressive Metal")						,
						Release::new("Fyrgast", "Frozen in time").with_metallum("https://www.metal-archives.com/bands/Fyrgast/3540501196", "https://www.metal-archives.com/albums/Fyrgast/Frozen_in_time/1280374", "Single", "Black Metal")						,
						Release::new("Templar", "I Natt").with_metallum("https://www.metal-archives.com/bands/Templar/3540532035", "https://www.metal-archives.com/albums/Templar/I_Natt/1280379", "Single", "Heavy Metal")						,
						Release::new("Morbid Invocation", "Opus I").with_metallum("https://www.metal-archives.com/bands/Morbid_Invocation/3540552419", "https://www.metal-archives.com/albums/Morbid_Invocation/Opus_I/1280396", "Full-length", "Black Metal"),
						Release::new("Phyllomedusa", "Hope Floats").with_metallum("https://www.metal-archives.com/bands/Phyllomedusa/3540529653", "https://www.metal-archives.com/albums/Phyllomedusa/Hope_Floats/1280408", "EP", "Gorenoise, Various")						,
						Release::new("Hazzerd", "Deathbringer").with_metallum("https://www.metal-archives.com/bands/Hazzerd/3540393393", "https://www.metal-archives.com/albums/Hazzerd/Deathbringer/1280442", "Single", "Thrash Metal")						,
						Release::new("Död Sol", "På drift i v​ä​st").with_metallum("https://www.metal-archives.com/bands/D%C3%B6d_Sol/3540503122", "https://www.metal-archives.com/albums/D%C3%B6d_Sol/P%C3%A5_drift_i_v%E2%80%8B%C3%A4%E2%80%8Bst/1280508", "Single", "Doom/Stoner Metal/Rock"),
					]),
                    (10, vec![
						Release::new("Rise of Kronos", "Imperium").with_metallum("https://www.metal-archives.com/bands/Rise_of_Kronos/3540504118", "https://www.metal-archives.com/albums/Rise_of_Kronos/Imperium/1266381", "Full-length", "Death/Thrash Metal"),
						Release::new("Aydra", "Leave to Nowhere").with_metallum("https://www.metal-archives.com/bands/Aydra/4983", "https://www.metal-archives.com/albums/Aydra/Leave_to_Nowhere/1266594", "Full-length", "Technical Death Metal"),
						Release::new("Pandemmy", "Faithless").with_metallum("https://www.metal-archives.com/bands/Pandemmy/3540301984", "https://www.metal-archives.com/albums/Pandemmy/Faithless/1268207", "Full-length", "Thrash/Death Metal"),
						Release::new("Pyracanda", "Losing Faith").with_metallum("https://www.metal-archives.com/bands/Pyracanda/1606", "https://www.metal-archives.com/albums/Pyracanda/Losing_Faith/1268217", "Full-length", "Thrash Metal"),
						Release::new("Demon Sacrifice", "Under the Blacklight of Divine Live").with_metallum("https://www.metal-archives.com/bands/Demon_Sacrifice/3540546557", "https://www.metal-archives.com/albums/Demon_Sacrifice/Under_the_Blacklight_of_Divine_Live/1269276", "Live album", "Black Metal/Punk"),
						Release::new("Patxa", "Just Heavy Metal").with_metallum("https://www.metal-archives.com/bands/Patxa/3540546181", "https://www.metal-archives.com/albums/Patxa/Just_Heavy_Metal/1276942", "Full-length", "Heavy Metal"),
						Release::new("Regicide", "Eternal Siege").with_metallum("https://www.metal-archives.com/bands/Regicide/3540481753", "https://www.metal-archives.com/albums/Regicide/Eternal_Siege/1278060", "Full-length", "Stoner/Sludge/Doom Metal"),
						Release::new("Dream Ocean", "A Chilling Show").with_metallum("https://www.metal-archives.com/bands/Dream_Ocean/3540437523", "https://www.metal-archives.com/albums/Dream_Ocean/A_Chilling_Show/1278425", "Single", "Symphonic Metal"),
						Release::new("Griefsoul", "Extreme Northern Griefmetal").with_metallum("https://www.metal-archives.com/bands/Griefsoul/3540495203", "https://www.metal-archives.com/albums/Griefsoul/Extreme_Northern_Griefmetal/1278522", "Full-length", "Melodic Death Metal"),
						Release::new("Epäkristus", "Satan, Sex and War - The Demo Compilation").with_metallum("https://www.metal-archives.com/bands/Ep%C3%A4kristus/26760", "https://www.metal-archives.com/albums/Ep%C3%A4kristus/Satan%2C_Sex_and_War_-_The_Demo_Compilation/1278987", "Compilation", "Raw Black Metal"),
						Release::new("Dustborn", "Unconcealed Atrocities").with_metallum("https://www.metal-archives.com/bands/Dustborn/3540450363", "https://www.metal-archives.com/albums/Dustborn/Unconcealed_Atrocities/1280472", "Full-length", "Symphonic Death Metal"),
					]),
					(11, vec![
						Release::new("Knightsune", "Fearless").with_metallum("https://www.metal-archives.com/bands/Knightsune/3540481992", "https://www.metal-archives.com/albums/Knightsune/Fearless/1237973", "Full-length", "Heavy/Power/Speed Metal")						,
						Release::new("Andy Gillion", "Exilium").with_metallum("https://www.metal-archives.com/bands/Andy_Gillion/3540460064", "https://www.metal-archives.com/albums/Andy_Gillion/Exilium/1254041", "Full-length", "Progressive Metal, Soundtrack")						,
						Release::new("Vomit Forth", "Terrified of God").with_metallum("https://www.metal-archives.com/bands/Vomit_Forth/3540443431", "https://www.metal-archives.com/albums/Vomit_Forth/Terrified_of_God/1254916", "Full-length", "Death Metal")						,
						Release::new("Kozoria", "The Source").with_metallum("https://www.metal-archives.com/bands/Kozoria/3540413946", "https://www.metal-archives.com/albums/Kozoria/The_Source/1255366", "Full-length", "Heavy Metal"),
						Release::new("Malasorte", "Apex Sovereignty").with_metallum("https://www.metal-archives.com/bands/Malasorte/3540547855", "https://www.metal-archives.com/albums/Malasorte/Apex_Sovereignty/1255960", "Full-length", "Black Metal")						,
						Release::new("Rifftera", "Coda").with_metallum("https://www.metal-archives.com/bands/Rifftera/3540377640", "https://www.metal-archives.com/albums/Rifftera/Coda/1256604", "EP", "Alternative/Melodic Death/Thrash Metal")						,
						Release::new("Hell Is Other People", "Moirae").with_metallum("https://www.metal-archives.com/bands/Hell_Is_Other_People/3540409256", "https://www.metal-archives.com/albums/Hell_Is_Other_People/Moirae/1257499", "Full-length", "Post-Black Metal")						,
						Release::new("Ad Infinitum", "Abyss").with_metallum("https://www.metal-archives.com/bands/Ad_Infinitum/3540465744", "https://www.metal-archives.com/albums/Ad_Infinitum/Abyss/1258247", "Full-length", "Symphonic Metal/Rock")						,
						Release::new("Dragony", "Hic Svnt Dracones").with_metallum("https://www.metal-archives.com/bands/Dragony/3540336425", "https://www.metal-archives.com/albums/Dragony/Hic_Svnt_Dracones/1258636", "Full-length", "Melodic/Symphonic Power Metal")						,
						Release::new("Oranssi Pazuzu", "Muuntautuja").with_metallum("https://www.metal-archives.com/bands/Oranssi_Pazuzu/3540279710", "https://www.metal-archives.com/albums/Oranssi_Pazuzu/Muuntautuja/1260006", "Full-length", "Psychedelic Black Metal")						,
						Release::new("Nightmare", "Waiting for the Power - The Early Years").with_metallum("https://www.metal-archives.com/bands/Nightmare/2727", "https://www.metal-archives.com/albums/Nightmare/Waiting_for_the_Power_-_The_Early_Years/1261180", "Compilation", "Heavy/Power Metal")						,
						Release::new("English Dogs", "Mad Punx & English Dogs 1983-1985").with_metallum("https://www.metal-archives.com/bands/English_Dogs/17467", "https://www.metal-archives.com/albums/English_Dogs/Mad_Punx_%26_English_Dogs_1983-1985/1261181", "Compilation", "Hardcore Punk, Speed/Power Metal, Crossover")						,
						Release::new("Decayed Existence", "The Beginning of Sorrows").with_metallum("https://www.metal-archives.com/bands/Decayed_Existence/3540303282", "https://www.metal-archives.com/albums/Decayed_Existence/The_Beginning_of_Sorrows/1262859", "EP", "Death Metal"),
						Release::new("Ana Argan List", "Irrbloss").with_metallum("https://www.metal-archives.com/bands/Ana_Argan_List/3540471996", "https://www.metal-archives.com/albums/Ana_Argan_List/Irrbloss/1263104", "Full-length", "Post-Black Metal")						,
						Release::new("The Mist from the Mountains", "Portal - The Gathering of Storms").with_metallum("https://www.metal-archives.com/bands/The_Mist_from_the_Mountains/3540499899", "https://www.metal-archives.com/albums/The_Mist_from_the_Mountains/Portal_-_The_Gathering_of_Storms/1263255", "Full-length", "Melodic Black Metal")						,
						Release::new("Barathrum", "Überkill").with_metallum("https://www.metal-archives.com/bands/Barathrum/2368", "https://www.metal-archives.com/albums/Barathrum/%C3%9Cberkill/1263515", "Full-length", "Black/Doom Metal")						,
						Release::new("Amputate", "Abysmal Ascent").with_metallum("https://www.metal-archives.com/bands/Amputate/3540378293", "https://www.metal-archives.com/albums/Amputate/Abysmal_Ascent/1263525", "Full-length", "Death Metal")						,
						Release::new("The Crown", "Crown of Thorns").with_metallum("https://www.metal-archives.com/bands/The_Crown/132", "https://www.metal-archives.com/albums/The_Crown/Crown_of_Thorns/1267829", "Full-length", "Melodic Death/Thrash Metal")						,
						Release::new("Speedrush", "Division Mortality").with_metallum("https://www.metal-archives.com/bands/Speedrush/3540421417", "https://www.metal-archives.com/albums/Speedrush/Division_Mortality/1268256", "Full-length", "Thrash/Speed Metal")						,
						Release::new("Master Boot Record", "Hardwarez").with_metallum("https://www.metal-archives.com/bands/Master_Boot_Record/3540420355", "https://www.metal-archives.com/albums/Master_Boot_Record/Hardwarez/1271001", "Full-length", "Industrial/Electronic Metal, Synthwave/Chiptune")						,
						Release::new("A Constant Knowledge of Death", "Appendix I: Revisions & Annotations").with_metallum("https://www.metal-archives.com/bands/A_Constant_Knowledge_of_Death/3540510172", "https://www.metal-archives.com/albums/A_Constant_Knowledge_of_Death/Appendix_I%3A_Revisions_%26_Annotations/1271002", "Compilation", "Progressive Sludge/Post-Metal/Hardcore")						,
						Release::new("My Dearest Wound", "The Weight of Life Was Greater").with_metallum("https://www.metal-archives.com/bands/My_Dearest_Wound/3540530874", "https://www.metal-archives.com/albums/My_Dearest_Wound/The_Weight_of_Life_Was_Greater/1271217", "EP", "Depressive/Post-Black Metal")						,
						Release::new("Maitreya", "Auxesis").with_metallum("https://www.metal-archives.com/bands/Maitreya/3540530303", "https://www.metal-archives.com/albums/Maitreya/Auxesis/1271547", "EP", "Progressive Metalcore/Djent")						,
						Release::new("Fupa Goddess", "Fuckyourface").with_metallum("https://www.metal-archives.com/bands/Fupa_Goddess/3540536542", "https://www.metal-archives.com/albums/Fupa_Goddess/Fuckyourface/1271878", "Full-length", "Goregrind/Death Metal, Noisegrind")						,
						Release::new("Krvl", "Donkere paden").with_metallum("https://www.metal-archives.com/bands/Krvl/3540481490", "https://www.metal-archives.com/albums/Krvl/Donkere_paden/1272003", "Full-length", "Post-Black Metal"),
						Release::new("Thanateros", "Tranceforming").with_metallum("https://www.metal-archives.com/bands/Thanateros/15278", "https://www.metal-archives.com/albums/Thanateros/Tranceforming/1272051", "Full-length", "Gothic Metal, Folk Metal"),
						Release::new("The Sword", "Live at LEVITATION").with_metallum("https://www.metal-archives.com/bands/The_Sword/57071", "https://www.metal-archives.com/albums/The_Sword/Live_at_LEVITATION/1272521", "Live album", "Stoner/Doom Metal, Hard Rock"),
						Release::new("Alarum", "Imperative").with_metallum("https://www.metal-archives.com/bands/Alarum/2352", "https://www.metal-archives.com/albums/Alarum/Imperative/1272935", "Single", "Progressive/Thrash Metal/Fusion"),
						Release::new("Alias Noone", "Weight of the World").with_metallum("https://www.metal-archives.com/bands/Alias_Noone/3540482486", "https://www.metal-archives.com/albums/Alias_Noone/Weight_of_the_World/1272995", "Full-length", "Progressive/Melodic Death Metal"),
						Release::new("Psychiatric Regurgitation", "Death Scriptures").with_metallum("https://www.metal-archives.com/bands/Psychiatric_Regurgitation/26157", "https://www.metal-archives.com/albums/Psychiatric_Regurgitation/Death_Scriptures/1274420", "EP", "Death Metal, Goregrind"),
						Release::new("Bazooka Troopaz", "The Booze Hounds of Hades").with_metallum("https://www.metal-archives.com/bands/Bazooka_Troopaz/3540532213", "https://www.metal-archives.com/albums/Bazooka_Troopaz/The_Booze_Hounds_of_Hades/1275699", "Single", "Crossover/Thrash Metal"),
						Release::new("Motörhead", "The Bomber Demos").with_metallum("https://www.metal-archives.com/bands/Mot%C3%B6rhead/203", "https://www.metal-archives.com/albums/Mot%C3%B6rhead/The_Bomber_Demos/1275776", "Demo", "Speed Metal, Heavy Metal/Hard Rock"),
						Release::new("Membrance", "Undead Remains").with_metallum("https://www.metal-archives.com/bands/Membrance/3540398647", "https://www.metal-archives.com/albums/Membrance/Undead_Remains/1275927", "EP", "Death Metal"),
						Release::new("Star Rider", "Outta Time").with_metallum("https://www.metal-archives.com/bands/Star_Rider/3540515568", "https://www.metal-archives.com/albums/Star_Rider/Outta_Time/1276444", "Full-length", "Heavy Metal"),
						Release::new("Ornamentos del Miedo", "Sueños").with_metallum("https://www.metal-archives.com/bands/Ornamentos_del_Miedo/3540453625", "https://www.metal-archives.com/albums/Ornamentos_del_Miedo/Sue%C3%B1os/1276782", "EP", "Atmospheric Funeral Doom Metal"),
						Release::new("Wrathprayer", "Enkoimeterion").with_metallum("https://www.metal-archives.com/bands/Wrathprayer/3540281226", "https://www.metal-archives.com/albums/Wrathprayer/Enkoimeterion/1276847", "Full-length", "Black/Death Metal"),
						Release::new("Roots of Disease", "Saligia Speculum").with_metallum("https://www.metal-archives.com/bands/Roots_of_Disease/3540521832", "https://www.metal-archives.com/albums/Roots_of_Disease/Saligia_Speculum/1278097", "Full-length", "Death Metal"),
						Release::new("Arcania", "Lost Generation").with_metallum("https://www.metal-archives.com/bands/Arcania/24396", "https://www.metal-archives.com/albums/Arcania/Lost_Generation/1278326", "Full-length", "Thrash Metal"),
						Release::new("Scumripper", "For a Few Fixes More").with_metallum("https://www.metal-archives.com/bands/Scumripper/3540407068", "https://www.metal-archives.com/albums/Scumripper/For_a_Few_Fixes_More/1278409", "Full-length", "Black/Thrash/Death Metal"),
						Release::new("Cytotoxin", "Hope Terminator").with_metallum("https://www.metal-archives.com/bands/Cytotoxin/3540325917", "https://www.metal-archives.com/albums/Cytotoxin/Hope_Terminator/1278462", "Single", "Technical/Brutal Death Metal"),
						Release::new("Oda", "Bloodstained").with_metallum("https://www.metal-archives.com/bands/Oda/3540550714", "https://www.metal-archives.com/albums/Oda/Bloodstained/1279409", "Full-length", "Psychedelic Doom Metal"),
						Release::new("Konatus", "Psikoz").with_metallum("https://www.metal-archives.com/bands/Konatus/3540545254", "https://www.metal-archives.com/albums/Konatus/Psikoz/1279432", "Full-length", "Death Metal"),
						Release::new("Epiklesis", "La Santa Iglesia Cat​ó​lica").with_metallum("https://www.metal-archives.com/bands/Epiklesis/3540551498", "https://www.metal-archives.com/albums/Epiklesis/La_Santa_Iglesia_Cat%E2%80%8B%C3%B3%E2%80%8Blica/1279523", "Full-length", "Symphonic Black Metal"),
						Release::new("Klynt", "Thunderous").with_metallum("https://www.metal-archives.com/bands/Klynt/3540337071", "https://www.metal-archives.com/albums/Klynt/Thunderous/1280205", "Full-length", "Power/Thrash Metal"),
						Release::new("Druid Stone", "\"Missing Girl\" b/w \"Satellite\"").with_metallum("https://www.metal-archives.com/bands/Druid_Stone/3540495933", "https://www.metal-archives.com/albums/Druid_Stone/%22Missing_Girl%22_b-w_%22Satellite%22/1280343", "Single", "Blackened Doom Metal"),
						Release::new("Timo Tolkki", "Stratovarius: 4th Dimension Demos").with_metallum("https://www.metal-archives.com/bands/Timo_Tolkki/2564", "https://www.metal-archives.com/albums/Timo_Tolkki/Stratovarius%3A_4th_Dimension_Demos/1280356", "Compilation", "Neoclassical Heavy Metal/Shred (early); Melodic Rock/Ambient (later)"),
					]),
                    (12, vec![
						Release::new("Rüsty Diamönds", "Stormbringer").with_metallum("https://www.metal-archives.com/bands/R%C3%BCsty_Diam%C3%B6nds/3540497176", "https://www.metal-archives.com/albums/R%C3%BCsty_Diam%C3%B6nds/Stormbringer/1264022", "EP", "Heavy Metal"),
						Release::new("For the Storms", "Losing What's Left of Us").with_metallum("https://www.metal-archives.com/bands/For_the_Storms/3540497580", "https://www.metal-archives.com/albums/For_the_Storms/Losing_What%27s_Left_of_Us/1271844", "Full-length", "Melodic Death/Doom Metal")						,
						Release::new("Godsin", "Blind Faith").with_metallum("https://www.metal-archives.com/bands/Godsin/3540550853", "https://www.metal-archives.com/albums/Godsin/Blind_Faith/1271900", "EP", "Thrash Metal")						,
						Release::new("Delta", "Gemini").with_metallum("https://www.metal-archives.com/bands/Delta/30091", "https://www.metal-archives.com/albums/Delta/Gemini/1272214", "Full-length", "Neoclassical/Progressive Metal")						,
						Release::new("Genital Grinder", "Anthology: Tricennium Reckoning").with_metallum("https://www.metal-archives.com/bands/Genital_Grinder/3540275941", "https://www.metal-archives.com/albums/Genital_Grinder/Anthology%3A_Tricennium_Reckoning/1278341", "Compilation", "Death Metal (early); Thrash Metal (later)")						,
						Release::new("Vitriolic", "Black Steel Vengeance").with_metallum("https://www.metal-archives.com/bands/Vitriolic/3540458345", "https://www.metal-archives.com/albums/Vitriolic/Black_Steel_Vengeance/1279649", "Full-length", "Black/Speed Metal")						,
						Release::new("Skull Servant", "Sepulcher of Barbarians").with_metallum("https://www.metal-archives.com/bands/Skull_Servant/3540513372", "https://www.metal-archives.com/albums/Skull_Servant/Sepulcher_of_Barbarians/1280007", "Single", "Blackened Thrash/Doom Metal"),
					]),
					(13, vec![
						Release::new("Scars of Solitude", "Under Disheartening Skies").with_metallum(
							"https://www.metal-archives.com/bands/Scars_of_Solitude/3540407725",
							"https://www.metal-archives.com/albums/Scars_of_Solitude/Under_Disheartening_Skies/1240074",
							"Full-length",
							"Melodic Heavy Metal"
						),
						Release::new("Bál", "Nagyobb n\u{200b}á\u{200b}lam").with_metallum(
							"https://www.metal-archives.com/bands/B%C3%A1l/3540446784",
							"https://www.metal-archives.com/albums/B%C3%A1l/Nagyobb_n%E2%80%8B%C3%A1%E2%80%8Blam/1279328",
							"Full-length",
							"Atmospheric/Post-Black Metal"
						),
						Release::new("Morbus Kinski", "Blunt Force Boogey").with_metallum(
							"https://www.metal-archives.com/bands/Morbus_Kinski/3540450950",
							"https://www.metal-archives.com/albums/Morbus_Kinski/Blunt_Force_Boogey/1279853",
							"EP",
							"Sludge Metal"
						  ),
					]),
					(15, vec![
						Release::new("Blood Red Fog / Verge", "Prism of Darkness / Second Mortification").with_metallum("https://www.metal-archives.com/bands/Blood_Red_Fog/42404", "https://www.metal-archives.com/albums/Blood_Red_Fog_-_Verge/Prism_of_Darkness_-_Second_Mortification/1263837", "Split", "Black Metal | Black Metal"),
						Release::new("War Dogs", "Only the Stars Are Left").with_metallum("https://www.metal-archives.com/bands/War_Dogs/3540441681", "https://www.metal-archives.com/albums/War_Dogs/Only_the_Stars_Are_Left/1265853", "Full-length", "Heavy/Speed Metal"),
						Release::new("Bloodrust / Regicide", "Through Death We Reign").with_metallum("https://www.metal-archives.com/bands/Bloodrust/3540478263", "https://www.metal-archives.com/albums/Bloodrust_-_Regicide/Through_Death_We_Reign/1268630", "Split", "Death Metal | Thrash Metal/Hardcore"),
						Release::new("Mortem", "Ilusión de sangre Pre-demo 1988").with_metallum("https://www.metal-archives.com/bands/Mortem/4234", "https://www.metal-archives.com/albums/Mortem/Ilusi%C3%B3n_de_sangre_Pre-demo_1988/1269512", "Demo", "Death Metal"),
					]),
					(17, vec![
						Release::new("Korkvak", "The Hermetic Ritual").with_metallum("https://www.metal-archives.com/bands/Korkvak/3540476374", "https://www.metal-archives.com/albums/Korkvak/The_Hermetic_Ritual/1257551", "EP", "Black Metal"),
						Release::new("Ghostheart Nebula", "Blackshift").with_metallum("https://www.metal-archives.com/bands/Ghostheart_Nebula/3540448531", "https://www.metal-archives.com/albums/Ghostheart_Nebula/Blackshift/1258822", "Full-length", "Melodic Doom/Death Metal"),
						Release::new("Winter Lantern", "Hymne to a Dismal Starre").with_metallum("https://www.metal-archives.com/bands/Winter_Lantern/3540475930", "https://www.metal-archives.com/albums/Winter_Lantern/Hymne_to_a_Dismal_Starre/1268695", "Full-length", "Black Metal"),
						Release::new("Harvestman", "Triptych: Part Three").with_metallum("https://www.metal-archives.com/bands/Harvestman/79302", "https://www.metal-archives.com/albums/Harvestman/Triptych%3A_Part_Three/1270484", "Full-length", "Folk/Drone/Ambient"),
						Release::new("Dianne", "Flameborn").with_metallum("https://www.metal-archives.com/bands/Dianne/3540518565", "https://www.metal-archives.com/albums/Dianne/Flameborn/1270872", "Single", "Symphonic Metal"),
						Release::new("Oannes", "Spiders Crawl in the Abode of Enki (An Key to Absu; The Threshold of Mystery)").with_metallum("https://www.metal-archives.com/bands/Oannes/3540532670", "https://www.metal-archives.com/albums/Oannes/Spiders_Crawl_in_the_Abode_of_Enki_%28An_Key_to_Absu%3B_The_Threshold_of_Mystery%29/1279144", "EP", "Black Metal"),
					]),
					(18, vec![
						Release::new("Obnoxious Youth", "Burning Savage").with_metallum(
							"https://www.metal-archives.com/bands/Obnoxious_Youth/3540329578",
							"https://www.metal-archives.com/albums/Obnoxious_Youth/Burning_Savage/1247851",
							"Full-length",
							"Heavy/Speed/Black Metal"
						),
						Release::new("Ensiferum", "Winter Storm").with_metallum(
							"https://www.metal-archives.com/bands/Ensiferum/332",
							"https://www.metal-archives.com/albums/Ensiferum/Winter_Storm/1248801",
							"Full-length",
							"Epic Folk Metal"
						),
						Release::new("The Resilient Dream Project", "Te recordaré").with_metallum(
							"https://www.metal-archives.com/bands/The_Resilient_Dream_Project/3540531567",
							"https://www.metal-archives.com/albums/The_Resilient_Dream_Project/Te_recordar%C3%A9/1252471",
							"Full-length",
							"Power Metal"
						),
						Release::new("Funeral", "Gospel of Bones").with_metallum(
							"https://www.metal-archives.com/bands/Funeral/3438",
							"https://www.metal-archives.com/albums/Funeral/Gospel_of_Bones/1254574",
							"Full-length",
							"Funeral Doom/Death Metal (early); Gothic/Doom Metal (later)"
						),
						Release::new("Capilla Ardiente", "Where Gods Live and Men Die").with_metallum(
							"https://www.metal-archives.com/bands/Capilla_Ardiente/3540287234",
							"https://www.metal-archives.com/albums/Capilla_Ardiente/Where_Gods_Live_and_Men_Die/1255958",
							"Full-length",
							"Epic Doom Metal"
						),
						Release::new("Cortez", "Thieves and Charlatans").with_metallum(
							"https://www.metal-archives.com/bands/Cortez/118000",
							"https://www.metal-archives.com/albums/Cortez/Thieves_and_Charlatans/1257775",
							"Full-length",
							"Stoner/Doom Metal"
						),
						Release::new("Jewel Throne", "Blood Vultures").with_metallum(
							"https://www.metal-archives.com/bands/Jewel_Throne/3540456874",
							"https://www.metal-archives.com/albums/Jewel_Throne/Blood_Vultures/1257909",
							"Full-length",
							"Thrash Metal"
						),
						Release::new("Grand Magus", "Sunraven").with_metallum(
							"https://www.metal-archives.com/bands/Grand_Magus/7162",
							"https://www.metal-archives.com/albums/Grand_Magus/Sunraven/1258193",
							"Full-length",
							"Heavy/Doom Metal"
						),
						Release::new("Jerry Cantrell", "I Want Blood") .with_metallum(
							"https://www.metal-archives.com/bands/Jerry_Cantrell/4079",
							"https://www.metal-archives.com/albums/Jerry_Cantrell/I_Want_Blood/1258560",
							"Full-length",
							"Doom/Alternative Metal/Rock, Grunge"
						),
						Release::new("Camos", "Hide from the Light").with_metallum(
							"https://www.metal-archives.com/bands/Camos/5486",
							"https://www.metal-archives.com/albums/Camos/Hide_from_the_Light/1258823",
							"Full-length",
							"Black Metal"
						),
						Release::new("Astral Doors", "The End of It All").with_metallum(
							"https://www.metal-archives.com/bands/Astral_Doors/14537",
							"https://www.metal-archives.com/albums/Astral_Doors/The_End_of_It_All/1259656",
							"Full-length",
							"Heavy/Power Metal"
						),
						Release::new("Feral", "To Usurp the Thrones").with_metallum(
							"https://www.metal-archives.com/bands/Feral/101215",
							"https://www.metal-archives.com/albums/Feral/To_Usurp_the_Thrones/1260256",
							"Full-length",
							"Death Metal"
						),
						Release::new("Crest of Darkness", "My Ghost").with_metallum(
							"https://www.metal-archives.com/bands/Crest_of_Darkness/4594",
							"https://www.metal-archives.com/albums/Crest_of_Darkness/My_Ghost/1260337",
							"Full-length",
							"Black/Death Metal"
						),
						Release::new("Chrysalïd", "Breaking the Chains").with_metallum(
							"https://www.metal-archives.com/bands/Chrysal%C3%AFd/3540504184",
							"https://www.metal-archives.com/albums/Chrysal%C3%AFd/Breaking_the_Chains/1260479",
							"Full-length",
							"Power Metal"
						),
						Release::new("Veonity", "The Final Element").with_metallum(
							"https://www.metal-archives.com/bands/Veonity/3540391551",
							"https://www.metal-archives.com/albums/Veonity/The_Final_Element/1261385",
							"Full-length",
							"Power Metal"
						),
						Release::new("Fate", "Reconnect 'n Ignite").with_metallum(
							"https://www.metal-archives.com/bands/Fate/3540456328",
							"https://www.metal-archives.com/albums/Fate/Reconnect_%27n_Ignite/1261530",
							"Full-length",
							"AOR (early); Melodic Heavy Metal/Hard Rock (later)"
						),
						Release::new("Frozen Crown", "War Hearts").with_metallum(
							"https://www.metal-archives.com/bands/Frozen_Crown/3540436490",
							"https://www.metal-archives.com/albums/Frozen_Crown/War_Hearts/1261897",
							"Full-length",
							"Power Metal"
				 		),
						Release::new("Nolove", "Corpse Bride").with_metallum(
							"https://www.metal-archives.com/bands/Nolove/3540531420",
							"https://www.metal-archives.com/albums/Nolove/Corpse_Bride/1262057",
							"Single",
							"Experimental/Depressive Black Metal, Post-Rock"
						),
						Release::new("Swallow the Sun", "Shining").with_metallum(
							"https://www.metal-archives.com/bands/Swallow_the_Sun/12613",
							"https://www.metal-archives.com/albums/Swallow_the_Sun/Shining/1262384",
							"Full-length",
							"Melodic Doom/Death Metal"
						),
						Release::new("Bonjour Tristesse", "The World Without Us").with_metallum(
							"https://www.metal-archives.com/bands/Bonjour_Tristesse/3540326670",
							"https://www.metal-archives.com/albums/Bonjour_Tristesse/The_World_Without_Us/1264294",
							"Full-length",
							"Post-Black Metal"
						),
						Release::new("Carnosus", "Wormtales").with_metallum(
							"https://www.metal-archives.com/bands/Carnosus/3540406456",
							"https://www.metal-archives.com/albums/Carnosus/Wormtales/1264311",
							"Full-length",
							"Death/Thrash Metal (early); Technical Melodic Death Metal (later)"
						),
						Release::new("Immortal Bird", "Sin Querencia").with_metallum(
							"https://www.metal-archives.com/bands/Immortal_Bird/3540373235",
							"https://www.metal-archives.com/albums/Immortal_Bird/Sin_Querencia/1264453",
							"Full-length",
							"Black/Sludge Metal, Crust"
						),
						Release::new("Mother of Graves", "The Periapt of Absence").with_metallum(
							"https://www.metal-archives.com/bands/Mother_of_Graves/3540475764",
							"https://www.metal-archives.com/albums/Mother_of_Graves/The_Periapt_of_Absence/1265234",
							"Full-length",
							"Melodic Death/Doom Metal"
						),
						Release::new("Ashen Tomb", "Ecstatic Death Reign").with_metallum(
							"https://www.metal-archives.com/bands/Ashen_Tomb/3540510857",
							"https://www.metal-archives.com/albums/Ashen_Tomb/Ecstatic_Death_Reign/1265616",
							"Full-length",
							"Death Metal"
						),
						Release::new("Harsh Realm", "Death Carries On").with_metallum(
							"https://www.metal-archives.com/bands/Harsh_Realm/3540454672",
							"https://www.metal-archives.com/albums/Harsh_Realm/Death_Carries_On/1265922",
							"Full-length",
							"Death/Doom Metal"
						),
						Release::new("Five the Hierophant", "Apeiron") .with_metallum(
							"https://www.metal-archives.com/bands/Five_the_Hierophant/3540432469",
							"https://www.metal-archives.com/albums/Five_the_Hierophant/Apeiron/1266330",
							"Full-length",
							"Avant-garde Post-Black/Doom Metal, Dark Ambient/Jazz"
						),
						Release::new("The Hypothesis", "Evolve").with_metallum(
							"https://www.metal-archives.com/bands/The_Hypothesis/3540309385",
							"https://www.metal-archives.com/albums/The_Hypothesis/Evolve/1266343",
							"Full-length",
							"Melodic Death Metal"
						),
						Release::new("Wreck-Defy", "Dissecting the Leech").with_metallum(
							"https://www.metal-archives.com/bands/Wreck-Defy/3540433322",
							"https://www.metal-archives.com/albums/Wreck-Defy/Dissecting_the_Leech/1267187",
							"Full-length",
							"Thrash Metal"
						),
						Release::new("Oryx", "Primordial Sky").with_metallum(
							"https://www.metal-archives.com/bands/Oryx/3540366004",
							"https://www.metal-archives.com/albums/Oryx/Primordial_Sky/1267621",
							"Full-length",
							"Sludge/Doom/Death Metal"
						),
						Release::new("Vanik", "IV").with_metallum(
							"https://www.metal-archives.com/bands/Vanik/3540420635",
							"https://www.metal-archives.com/albums/Vanik/IV/1267627",
							"Full-length",
							"Heavy/Speed Metal"
						),
						Release::new("Gorebringer", "Condemned to Suffer").with_metallum(
							"https://www.metal-archives.com/bands/Gorebringer/3540453473",
							"https://www.metal-archives.com/albums/Gorebringer/Condemned_to_Suffer/1267760",
							"Full-length",
							"Melodic Death Metal"
						),
						Release::new("DGM", "Endless") .with_metallum(
							"https://www.metal-archives.com/bands/DGM/1419",
							"https://www.metal-archives.com/albums/DGM/Endless/1267903",
							"Full-length",
							"Progressive Power Metal"
						),
						Release::new("Destruktor", "Indomitable").with_metallum(
							"https://www.metal-archives.com/bands/Destruktor/2999",
							"https://www.metal-archives.com/albums/Destruktor/Indomitable/1268474",
							"Full-length",
							"Black/Death Metal"
						),
						Release::new("Clot", "Dehiscence") .with_metallum(
							"https://www.metal-archives.com/bands/Clot/3540509988",
							"https://www.metal-archives.com/albums/Clot/Dehiscence/1268673",
							"EP",
							"Grindcore/Death Metal/Powerviolence"
						),
						Release::new("Persecutory", "The Glorious Persecution").with_metallum(
							"https://www.metal-archives.com/bands/Persecutory/3540404249",
							"https://www.metal-archives.com/albums/Persecutory/The_Glorious_Persecution/1269893",
							"EP",
							"Black/Death Metal"
						),
						Release::new("Deathrite", "Flames Licking Fever").with_metallum(
							"https://www.metal-archives.com/bands/Deathrite/3540331245",
							"https://www.metal-archives.com/albums/Deathrite/Flames_Licking_Fever/1270493",
							"Full-length",
							"Death Metal/Punk"
						),
						Release::new("Contrition", "Pariahs").with_metallum(
							"https://www.metal-archives.com/bands/Contrition/3540496861",
							"https://www.metal-archives.com/albums/Contrition/Pariahs/1270601",
							"EP",
							"Death Metal/Grindcore"
						),
						Release::new("Seid", "Hymns to the Norse").with_metallum(
							"https://www.metal-archives.com/bands/Seid/3540410306",
							"https://www.metal-archives.com/albums/Seid/Hymns_to_the_Norse/1270865",
							"Full-length",
							"Black Metal"
						),
						Release::new("Deserts of Mars", "Dead Planet Exodus").with_metallum(
							"https://www.metal-archives.com/bands/Deserts_of_Mars/3540461577",
							"https://www.metal-archives.com/albums/Deserts_of_Mars/Dead_Planet_Exodus/1270978",
							"Full-length",
							"Stoner Rock/Metal"
						),
						Release::new("Fórn", "Repercussions of the Self") .with_metallum(
							"https://www.metal-archives.com/bands/F%C3%B3rn/3540367402",
							"https://www.metal-archives.com/albums/F%C3%B3rn/Repercussions_of_the_Self/1271589",
							"Full-length",
							"Doom/Sludge Metal"
						),
						Release::new("Aethyrick / Marras", "A Union of Spectres").with_metallum(
							"https://www.metal-archives.com/bands/Aethyrick/3540427869",
							"https://www.metal-archives.com/albums/Aethyrick_-_Marras/A_Union_of_Spectres/1271917",
							"Split",
							"Black Metal | Black Metal"
						),
						Release::new("Porenut", "Zaklęcie").with_metallum(
							"https://www.metal-archives.com/bands/Porenut/3540522079",
							"https://www.metal-archives.com/albums/Porenut/Zakl%C4%99cie/1272346",
							"Single",
							"Pagan/Folk Metal"
						),
						Release::new("Weeping", "Spiritual Barbarism").with_metallum(
							"https://www.metal-archives.com/bands/Weeping/3540476831",
							"https://www.metal-archives.com/albums/Weeping/Spiritual_Barbarism/1272938",
							"Full-length",
							"Death Metal/Hardcore/Powerviolence"
						),
						Release::new("Dawnwalker", "The Unknowing").with_metallum(
							"https://www.metal-archives.com/bands/Dawnwalker/3540436302",
							"https://www.metal-archives.com/albums/Dawnwalker/The_Unknowing/1273163",
							"Full-length",
							"Post-Metal/Rock"
						),
						Release::new("Onslaught Kommand", "Malignancy").with_metallum(
							"https://www.metal-archives.com/bands/Onslaught_Kommand/3540510750",
							"https://www.metal-archives.com/albums/Onslaught_Kommand/Malignancy/1275146",
							"Full-length",
							"Black/Death Metal/Grindcore"
						),
						Release::new("Maatkare", "Rise to Power").with_metallum(
							"https://www.metal-archives.com/bands/Maatkare/3540549903",
							"https://www.metal-archives.com/albums/Maatkare/Rise_to_Power/1275352",
							"Full-length",
							"Death Metal"
						),
						Release::new("Synthwailer", "Cry Waterfalls").with_metallum(
							"https://www.metal-archives.com/bands/Synthwailer/3540486233",
							"https://www.metal-archives.com/albums/Synthwailer/Cry_Waterfalls/1275479",
							"Single",
							"Symphonic Power/Heavy Metal"
						),
						Release::new("Torn from the Womb", "Final Improvement Operation Symposium: Terminal Epicrise, vol. I - IV").with_metallum(
							"https://www.metal-archives.com/bands/Torn_from_the_Womb/3540438240",
							"https://www.metal-archives.com/albums/Torn_from_the_Womb/Final_Improvement_Operation_Symposium%3A_Terminal_Epicrise%2C_vol._I_-_IV/1275870",
							"Compilation",
							"Technical/Brutal Death Metal"
						),
						Release::new("Disentomb", "Nothing Above") .with_metallum(
							"https://www.metal-archives.com/bands/Disentomb/3540290104",
							"https://www.metal-archives.com/albums/Disentomb/Nothing_Above/1276528",
							"EP",
							"Brutal Death Metal"
						),
						Release::new("Kreyl", "Obscure Rise of Ancient Eulogy").with_metallum("https://www.metal-archives.com/bands/Kreyl/3540548438", "https://www.metal-archives.com/albums/Kreyl/Obscure_Rise_of_Ancient_Eulogy/1276611", "Full-length", "Black Metal"),
						Release::new("Damnations Domain", "The God of Blood").with_metallum("https://www.metal-archives.com/bands/Damnations_Domain/3540474797", "https://www.metal-archives.com/albums/Damnations_Domain/The_God_of_Blood/1277928", "Compilation", "Death Metal/Grindcore"),
						Release::new("Admire the Grim", "Crescent Moon").with_metallum("https://www.metal-archives.com/bands/Admire_the_Grim/3540514900", "https://www.metal-archives.com/albums/Admire_the_Grim/Crescent_Moon/1278184", "Single", "Melodic Death Metal")						,
						Release::new("Royal Glam", "Shields Ain't Gunna Save Ya").with_metallum("https://www.metal-archives.com/bands/Royal_Glam/3540469056", "https://www.metal-archives.com/albums/Royal_Glam/Shields_Ain%27t_Gunna_Save_Ya/1278271", "EP", "Heavy/Glam Metal")						,
						Release::new("Deivos", "Apophenia").with_metallum("https://www.metal-archives.com/bands/Deivos/13187", "https://www.metal-archives.com/albums/Deivos/Apophenia/1278311", "Full-length", "Technical Death Metal"),
						Release::new("Ixion", "Regeneration").with_metallum("https://www.metal-archives.com/bands/Ixion/108420", "https://www.metal-archives.com/albums/Ixion/Regeneration/1278552", "EP", "Atmospheric Doom Metal"),
						Release::new("Wormrot", "Left to Rot").with_metallum("https://www.metal-archives.com/bands/Wormrot/3540285169", "https://www.metal-archives.com/albums/Wormrot/Left_to_Rot/1278710", "Compilation", "Grindcore")						,
						Release::new("Valac", "Under the Ophidians Curse").with_metallum("https://www.metal-archives.com/bands/Valac/3540452509", "https://www.metal-archives.com/albums/Valac/Under_the_Ophidians_Curse/1278833", "Full-length", "Raw Black Metal")						,
						Release::new("Cursed Cemetery", "Magma Transmigration").with_metallum("https://www.metal-archives.com/bands/Cursed_Cemetery/58613", "https://www.metal-archives.com/albums/Cursed_Cemetery/Magma_Transmigration/1279095", "Full-length", "Death/Black Metal (early); Black Metal, Ambient/Downtempo (later)"),
						Release::new("Djevel", "Under nattens fane i Fandens prakt").with_metallum("https://www.metal-archives.com/bands/Djevel/3540328290", "https://www.metal-archives.com/albums/Djevel/Under_nattens_fane_i_Fandens_prakt/1279573", "Single", "Black Metal"),
						Release::new("Outworld", "Way of the Samurai").with_metallum("https://www.metal-archives.com/bands/Outworld/3540528437", "https://www.metal-archives.com/albums/Outworld/Way_of_the_Samurai/1280018", "Full-length", "Power Metal"),
						Release::new("Distrüster", "Obscurum Per Obscurius").with_metallum("https://www.metal-archives.com/bands/Distr%C3%BCster/3540509268", "https://www.metal-archives.com/albums/Distr%C3%BCster/Obscurum_Per_Obscurius/1280180", "EP", "Crust/Speed/Death Metal")						,
					]),
					(19, vec![
						Release::new("Inche Kai Che", "Transmutar").with_metallum("https://www.metal-archives.com/bands/Inche_Kai_Che/3540549007", "https://www.metal-archives.com/albums/Inche_Kai_Che/Transmutar/1262156", "Single", "Stoner Metal/Rock"),
					]),
					(20, vec![
						Release::new("Silhouette", "Les dires de l'âme").with_metallum(
							"https://www.metal-archives.com/bands/Silhouette/3540500943",
							"https://www.metal-archives.com/albums/Silhouette/Les_dires_de_l%27%C3%A2me/1268053",
							"Full-length",
							"Atmospheric Black Metal"
						),
						Release::new("Infernal Cult", "Necessity of Unreal").with_metallum(
							"https://www.metal-archives.com/bands/Infernal_Cult/3540419847",
							"https://www.metal-archives.com/albums/Infernal_Cult/Necessity_of_Unreal/1268313",
							"Full-length",
							"Black Metal"
						),
						Release::new("Bewitcher", "The Warrior Trail").with_metallum(
							"https://www.metal-archives.com/bands/Bewitcher/3540374488",
							"https://www.metal-archives.com/albums/Bewitcher/The_Warrior_Trail/1277285",
							"Single",
							"Black/Speed Metal"
						),
						Release::new("Silent Requiem", "Alice: Un Cuento de Luz y Sombras").with_metallum(
							"https://www.metal-archives.com/bands/Silent_Requiem/3540478127",
							"https://www.metal-archives.com/albums/Silent_Requiem/Alice%3A_Un_Cuento_de_Luz_y_Sombras/1277956",
							"EP",
							"Symphonic Power Metal"
						),
						Release::new("Kiko Loureiro", "Theory of Mind").with_metallum(
							"https://www.metal-archives.com/bands/Kiko_Loureiro/32477",
							"https://www.metal-archives.com/albums/Kiko_Loureiro/Theory_of_Mind/1278778",
							"Full-length",
							"Melodic Progressive Metal/Shred"
						  ),
					]),
					(21, vec![
						Release::new("Reckless Manslaughter", "Sinking into Filth").with_metallum("https://www.metal-archives.com/bands/Reckless_Manslaughter/3540310538", "https://www.metal-archives.com/albums/Reckless_Manslaughter/Sinking_into_Filth/1262756", "Full-length", "Death Metal"),
						Release::new("Midnightmares", "Shadow People").with_metallum("https://www.metal-archives.com/bands/Midnightmares/3540537876", "https://www.metal-archives.com/albums/Midnightmares/Shadow_People/1268797", "Full-length", "Symphonic Gothic Metal"),
						Release::new("The Fall of Creation", "Enlightenment").with_metallum("https://www.metal-archives.com/bands/The_Fall_of_Creation/3540483665", "https://www.metal-archives.com/albums/The_Fall_of_Creation/Enlightenment/1275846", "Full-length", "Melodic Death/Groove Metal"),
						Release::new("Sissourlet", "Rituals in the Catacombs").with_metallum("https://www.metal-archives.com/bands/Sissourlet/3540545064", "https://www.metal-archives.com/albums/Sissourlet/Rituals_in_the_Catacombs/1279821", "Full-length", "Hardcore/Death Metal"),
					]),
					(22, vec![
						Release::new("Anialator", "Death Is Calling").with_metallum(
							"https://www.metal-archives.com/bands/Anialator/1485",
							"https://www.metal-archives.com/albums/Anialator/Death_Is_Calling/1266210",
							"Full-length",
							"Thrash Metal"
						),
						Release::new("Tarfania", "Where No Wolf Howls...").with_metallum(
							"https://www.metal-archives.com/bands/Tarfania/3540504695",
							"https://www.metal-archives.com/albums/Tarfania/Where_No_Wolf_Howls.../1269440",
							"Full-length",
							"Black Metal"
						),
						Release::new("Soupir", "Ecoute s'il pleut").with_metallum(
							"https://www.metal-archives.com/bands/Soupir/3540500785",
							"https://www.metal-archives.com/albums/Soupir/Ecoute_s%27il_pleut/1270947",
							"EP",
							"Atmospheric/Depressive Black Metal"
						  ),
					]),
					(23, vec![
						Release::new("Clandestined", "Dead.....Forever").with_metallum("https://www.metal-archives.com/bands/Clandestined/3540311370", "https://www.metal-archives.com/albums/Clandestined/Dead.....Forever/1259724", "Full-length", "Thrash/Death Metal"),
						Release::new("Bellfast", "The Warrior Celt").with_metallum("https://www.metal-archives.com/bands/Bellfast/90747", "https://www.metal-archives.com/albums/Bellfast/The_Warrior_Celt/1262455", "Full-length", "Folk Metal"),
					]),
					(24, vec![
						Release::new("Lankester Merrin", "Dark Mother's Child").with_metallum("https://www.metal-archives.com/bands/Lankester_Merrin/3540487258", "https://www.metal-archives.com/albums/Lankester_Merrin/Dark_Mother%27s_Child/1268707", "Full-length", "Melodic Heavy Metal"),
						Release::new("Bind Torture Kill", "Sauvagerie").with_metallum("https://www.metal-archives.com/bands/Bind_Torture_Kill/3540418093", "https://www.metal-archives.com/albums/Bind_Torture_Kill/Sauvagerie/1272607", "Full-length", "Death Metal/Grindcore"),
					]),
					(25, vec![
						Release::new("Harpyie", "Voodoo").with_metallum("https://www.metal-archives.com/bands/Harpyie/3540345686", "https://www.metal-archives.com/albums/Harpyie/Voodoo/1247634", "Full-length", "Folk/Melodic Groove Metal/Metalcore"),
						Release::new("Burial Remains", "Adversarial").with_metallum("https://www.metal-archives.com/bands/Burial_Remains/3540454759", "https://www.metal-archives.com/albums/Burial_Remains/Adversarial/1249107", "Full-length", "Death Metal"),
						Release::new("The Spirit", "Songs Against Humanity").with_metallum("https://www.metal-archives.com/bands/The_Spirit/3540432482", "https://www.metal-archives.com/albums/The_Spirit/Songs_Against_Humanity/1251505", "Full-length", "Black/Death Metal"),
						Release::new("Entheos", "An End to Everything").with_metallum("https://www.metal-archives.com/bands/Entheos/3540521376", "https://www.metal-archives.com/albums/Entheos/An_End_to_Everything/1254557", "EP", "Technical/Progressive Deathcore"),
						Release::new("Triumpher", "Spirit Invictus").with_metallum("https://www.metal-archives.com/bands/Triumpher/3540521732", "https://www.metal-archives.com/albums/Triumpher/Spirit_Invictus/1255352", "Full-length", "Heavy/Power Metal"),
						Release::new("Loudblast", "Altering Fates and Destinies").with_metallum("https://www.metal-archives.com/bands/Loudblast/944", "https://www.metal-archives.com/albums/Loudblast/Altering_Fates_and_Destinies/1255971", "Full-length", "Death/Thrash Metal (early); Death Metal (later)"),
						Release::new("Gaerea", "Coma").with_metallum("https://www.metal-archives.com/bands/Gaerea/3540418247", "https://www.metal-archives.com/albums/Gaerea/Coma/1256039", "Full-length", "Black Metal"),
						Release::new("Elephant Tree / Lowrider", "The Long Forever").with_metallum("https://www.metal-archives.com/bands/Elephant_Tree/3540386663", "https://www.metal-archives.com/albums/Elephant_Tree_-_Lowrider/The_Long_Forever/1257761", "Split", "Doom/Stoner Metal | Stoner Metal/Rock"),
						Release::new("Haliphron", "Anatomy of Darkness").with_metallum("https://www.metal-archives.com/bands/Haliphron/3540522764", "https://www.metal-archives.com/albums/Haliphron/Anatomy_of_Darkness/1258547", "Full-length", "Symphonic Black/Death Metal"),
						Release::new("Blackevil", "Praise the Communion Fire for the Unhallowed Sacrament").with_metallum("https://www.metal-archives.com/bands/Blackevil/3540390999", "https://www.metal-archives.com/albums/Blackevil/Praise_the_Communion_Fire_for_the_Unhallowed_Sacrament/1261696", "Full-length", "Black/Thrash Metal"),
						Release::new("Challenger", "Force of Nature").with_metallum("https://www.metal-archives.com/bands/Challenger/3540452597", "https://www.metal-archives.com/albums/Challenger/Force_of_Nature/1261964", "Full-length", "Heavy/Speed Metal"),
						Release::new("Emasculator", "The Disfigured and the Divine").with_metallum("https://www.metal-archives.com/bands/Emasculator/3540509792", "https://www.metal-archives.com/albums/Emasculator/The_Disfigured_and_the_Divine/1262625", "EP", "Brutal Death Metal"),
						Release::new("Ghosts of Glaciers", "Eternal").with_metallum("https://www.metal-archives.com/bands/Ghosts_of_Glaciers/3540399472", "https://www.metal-archives.com/albums/Ghosts_of_Glaciers/Eternal/1263334", "Full-length", "Blackened Post-Metal"),
						Release::new("Nuclear", "Violent DNA").with_metallum("https://www.metal-archives.com/bands/Nuclear/47481", "https://www.metal-archives.com/albums/Nuclear/Violent_DNA/1263388", "EP", "Thrash Metal"),
						Release::new("Paysage d'Hiver", "Die Berge").with_metallum("https://www.metal-archives.com/bands/Paysage_d%27Hiver/13417", "https://www.metal-archives.com/albums/Paysage_d%27Hiver/Die_Berge/1263444", "Full-length", "Black Metal, Ambient"),
						Release::new("Leviticus", "MMXXIV").with_metallum("https://www.metal-archives.com/bands/Leviticus/9513", "https://www.metal-archives.com/albums/Leviticus/MMXXIV/1263536", "EP", "Heavy Metal/Hard Rock"),
						Release::new("Schammasch", "The Maldoror Chants: Old Ocean").with_metallum("https://www.metal-archives.com/bands/Schammasch/3540316251", "https://www.metal-archives.com/albums/Schammasch/The_Maldoror_Chants%3A_Old_Ocean/1263598", "Full-length", "Black/Death Metal (early); Avant-garde/Black Metal (later)"),
						Release::new("Autumn's Grief", "Dead Among the Living").with_metallum("https://www.metal-archives.com/bands/Autumn%27s_Grief/3540496003", "https://www.metal-archives.com/albums/Autumn%27s_Grief/Dead_Among_the_Living/1263677", "Full-length", "Symphonic Metal"),
						Release::new("Ancient Curse", "Dimension 5").with_metallum("https://www.metal-archives.com/bands/Ancient_Curse/1699", "https://www.metal-archives.com/albums/Ancient_Curse/Dimension_5/1263717", "Full-length", "Progressive Heavy/Power Metal"),
						Release::new("Mindless Sinner", "Metal Merchants").with_metallum("https://www.metal-archives.com/bands/Mindless_Sinner/23990", "https://www.metal-archives.com/albums/Mindless_Sinner/Metal_Merchants/1264122", "Full-length", "Heavy Metal"),
						Release::new("Disarray", "Religious Disease").with_metallum("https://www.metal-archives.com/bands/Disarray/3540529039", "https://www.metal-archives.com/albums/Disarray/Religious_Disease/1265036", "Full-length", "Thrash Metal"),
						Release::new("Psychonaut 4", "...of Mourning").with_metallum("https://www.metal-archives.com/bands/Psychonaut_4/3540329261", "https://www.metal-archives.com/albums/Psychonaut_4/...of_Mourning/1265317", "Full-length", "Depressive Black Metal/Rock"),
						Release::new("Gigan", "Anomalous Abstractigate Infinitessimus").with_metallum("https://www.metal-archives.com/bands/Gigan/108235", "https://www.metal-archives.com/albums/Gigan/Anomalous_Abstractigate_Infinitessimus/1265799", "Full-length", "Progressive/Technical Death Metal"),
						Release::new("Sentient Horror", "In Service of the Dead").with_metallum("https://www.metal-archives.com/bands/Sentient_Horror/3540420358", "https://www.metal-archives.com/albums/Sentient_Horror/In_Service_of_the_Dead/1265850", "Full-length", "Death Metal"),
						Release::new("Hatchet", "Leave No Soul").with_metallum("https://www.metal-archives.com/bands/Hatchet/75808", "https://www.metal-archives.com/albums/Hatchet/Leave_No_Soul/1265863", "EP", "Thrash Metal"),
						Release::new("Athena XIX", "Everflow Part 1: Frames of Humanity").with_metallum("https://www.metal-archives.com/bands/Athena_XIX/707", "https://www.metal-archives.com/albums/Athena_XIX/Everflow_Part_1%3A_Frames_of_Humanity/1265971", "Full-length", "Progressive/Power Metal"),
						Release::new("Iotunn", "Kinship").with_metallum("https://www.metal-archives.com/bands/Iotunn/3540408017", "https://www.metal-archives.com/albums/Iotunn/Kinship/1266004", "Full-length", "Progressive Power Metal (early); Progressive Melodic Death Metal (later)"),
						Release::new("Gigan", "The Gigan Cassette Box Set").with_metallum("https://www.metal-archives.com/bands/Gigan/108235", "https://www.metal-archives.com/albums/Gigan/The_Gigan_Cassette_Box_Set/1266302", "Boxed set", "Progressive/Technical Death Metal"),
						Release::new("Devin Townsend", "PowerNerd").with_metallum("https://www.metal-archives.com/bands/Devin_Townsend/1245", "https://www.metal-archives.com/albums/Devin_Townsend/PowerNerd/1266334", "Full-length", "Progressive Metal/Rock, Ambient"),
						Release::new("Turkey Vulture", "On The List").with_metallum("https://www.metal-archives.com/bands/Turkey_Vulture/3540483095", "https://www.metal-archives.com/albums/Turkey_Vulture/On_The_List/1266491", "EP", "Stoner/Doom Metal"),
						Release::new("Smoke / Doomsday Profit", "Smoke // Doomsday Profit").with_metallum("https://www.metal-archives.com/bands/Smoke/3540502223", "https://www.metal-archives.com/albums/Smoke_-_Doomsday_Profit/Smoke_--_Doomsday_Profit/1266534", "Split", "Stoner/Doom Metal | Stoner/Doom Metal"),
						Release::new("Taking the Head of Goliath", "Futility of the Flesh").with_metallum("https://www.metal-archives.com/bands/Taking_the_Head_of_Goliath/3540424198", "https://www.metal-archives.com/albums/Taking_the_Head_of_Goliath/Futility_of_the_Flesh/1266668", "EP", "Brutal Death Metal"),
						Release::new("Antipope", "Doors of the Dead").with_metallum("https://www.metal-archives.com/bands/Antipope/85290", "https://www.metal-archives.com/albums/Antipope/Doors_of_the_Dead/1266671", "Full-length", "Progressive Black Metal (early); Progressive/Gothic/Industrial Metal (later)"),
						Release::new("Alex Nunziati", "Impending Catastrophe").with_metallum("https://www.metal-archives.com/bands/Alex_Nunziati/3540506323", "https://www.metal-archives.com/albums/Alex_Nunziati/Impending_Catastrophe/1266673", "Full-length", "Heavy Metal, Thrash Metal"),
						Release::new("Vokonis", "Transitions").with_metallum("https://www.metal-archives.com/bands/Vokonis/3540411114", "https://www.metal-archives.com/albums/Vokonis/Transitions/1267264", "Full-length", "Stoner/Doom Metal"),
						Release::new("Mercyless", "Those Who Reign Below").with_metallum("https://www.metal-archives.com/bands/Mercyless/7544", "https://www.metal-archives.com/albums/Mercyless/Those_Who_Reign_Below/1267629", "Full-length", "Death/Thrash Metal"),
						Release::new("Sedimentum", "Derri​è​re les portes d'une arcane transcendante").with_metallum("https://www.metal-archives.com/bands/Sedimentum/3540455227", "https://www.metal-archives.com/albums/Sedimentum/Derri%E2%80%8B%C3%A8%E2%80%8Bre_les_portes_d%27une_arcane_transcendante/1267941", "EP", "Death Metal"),
						Release::new("Adamantra", "Act III: Pareidolia of Depravity").with_metallum("https://www.metal-archives.com/bands/Adamantra/84533", "https://www.metal-archives.com/albums/Adamantra/Act_III%3A_Pareidolia_of_Depravity/1268265", "Full-length", "Progressive/Power Metal"),
						Release::new("Stilverlight", "Dead Souls").with_metallum("https://www.metal-archives.com/bands/Stilverlight/3540389416", "https://www.metal-archives.com/albums/Stilverlight/Dead_Souls/1268317", "Full-length", "Melodic Power Metal"),
						Release::new("Perfidious", "Savouring His Flesh").with_metallum("https://www.metal-archives.com/bands/Perfidious/3540395457", "https://www.metal-archives.com/albums/Perfidious/Savouring_His_Flesh/1268454", "Full-length", "Death Metal"),
						Release::new("Bloodletter / Grozov / Acid Mass / Ninth Realm", "Faster than the Devil III").with_metallum("https://www.metal-archives.com/bands/Bloodletter/3540386435", "https://www.metal-archives.com/albums/Bloodletter_-_Grozov_-_Acid_Mass_-_Ninth_Realm/Faster_than_the_Devil_III/1268676", "Split", "Thrash Metal (early); Melodic Thrash Metal (later) | Black Metal (early); Black/Power/Thrash Metal (later) | Thrash Metal | Crossover/Thrash Metal"),
						Release::new("Ataraxie", "Le déclin").with_metallum("https://www.metal-archives.com/bands/Ataraxie/10030", "https://www.metal-archives.com/albums/Ataraxie/Le_d%C3%A9clin/1269268", "Full-length", "Funeral Doom/Death Metal"),
						Release::new("Thaw", "Fading Backwards").with_metallum("https://www.metal-archives.com/bands/Thaw/3540318708", "https://www.metal-archives.com/albums/Thaw/Fading_Backwards/1269883", "Full-length", "Black Metal/Ambient/Noise"),
						Release::new("Behemoth", "XXX Years ov Blasphemy").with_metallum("https://www.metal-archives.com/bands/Behemoth/263", "https://www.metal-archives.com/albums/Behemoth/XXX_Years_ov_Blasphemy/1269914", "Live album", "Black Metal (early); Black/Death Metal (later)"),
						Release::new("Bütcher", "On Fowl of Tyrant Wing").with_metallum("https://www.metal-archives.com/bands/B%C3%BCtcher/19687", "https://www.metal-archives.com/albums/B%C3%BCtcher/On_Fowl_of_Tyrant_Wing/1270136", "Full-length", "Black/Speed Metal")						,
						Release::new("Blasphemous", "To Lay Siege and Conquer").with_metallum("https://www.metal-archives.com/bands/Blasphemous/42070", "https://www.metal-archives.com/albums/Blasphemous/To_Lay_Siege_and_Conquer/1270221", "Full-length", "Death/Black Metal"),
						Release::new("Living Gate", "Suffer as One").with_metallum("https://www.metal-archives.com/bands/Living_Gate/3540469330", "https://www.metal-archives.com/albums/Living_Gate/Suffer_as_One/1270488", "Full-length", "Death Metal"),
						Release::new("Avtotheism", "Reflections of Execrable Stillness").with_metallum("https://www.metal-archives.com/bands/Avtotheism/3540424369", "https://www.metal-archives.com/albums/Avtotheism/Reflections_of_Execrable_Stillness/1270825", "Full-length", "Brutal Death Metal (early); Technical Death Metal (later)")						,
						Release::new("Vananidr", "In Silence Descent").with_metallum("https://www.metal-archives.com/bands/Vananidr/3540447504", "https://www.metal-archives.com/albums/Vananidr/In_Silence_Descent/1271718", "Full-length", "Melodic Black/Death Metal"),
						Release::new("Nitrogods", "Valley of the Gods").with_metallum("https://www.metal-archives.com/bands/Nitrogods/3540492123", "https://www.metal-archives.com/albums/Nitrogods/Valley_of_the_Gods/1271980", "Full-length", "Heavy Metal/Hard Rock"),
						Release::new("Symphony of Heaven", "Ordo Aurum Archeia").with_metallum("https://www.metal-archives.com/bands/Symphony_of_Heaven/3540434302", "https://www.metal-archives.com/albums/Symphony_of_Heaven/Ordo_Aurum_Archeia/1272089", "Full-length", "Melodic Black/Death Metal"),
						Release::new("Motörhead", "We Take No Prisoners (The Singles 1995-2006)").with_metallum("https://www.metal-archives.com/bands/Mot%C3%B6rhead/203", "https://www.metal-archives.com/albums/Mot%C3%B6rhead/We_Take_No_Prisoners_%28The_Singles_1995-2006%29/1272110", "Compilation", "Speed Metal, Heavy Metal/Hard Rock"),
						Release::new("Pounder", "Thunderforged").with_metallum("https://www.metal-archives.com/bands/Pounder/3540426300", "https://www.metal-archives.com/albums/Pounder/Thunderforged/1272203", "Full-length", "Heavy Metal"),
						Release::new("Grand Harvest", "Till Förruttnelsen").with_metallum("https://www.metal-archives.com/bands/Grand_Harvest/3540477264", "https://www.metal-archives.com/albums/Grand_Harvest/Till_F%C3%B6rruttnelsen/1272508", "EP", "Death/Doom Metal"),
						Release::new("Black Curse", "Burning in Celestial Poison").with_metallum("https://www.metal-archives.com/bands/Black_Curse/3540457850", "https://www.metal-archives.com/albums/Black_Curse/Burning_in_Celestial_Poison/1272699", "Full-length", "Death/Black Metal"),
						Release::new("Deadform", "Entrenched in Hell").with_metallum("https://www.metal-archives.com/bands/Deadform/3540515886", "https://www.metal-archives.com/albums/Deadform/Entrenched_in_Hell/1272722", "Full-length", "Sludge Metal/Crust Punk"),
						Release::new("Upiór", "Predator of Fear").with_metallum("https://www.metal-archives.com/bands/Upi%C3%B3r/3540475866", "https://www.metal-archives.com/albums/Upi%C3%B3r/Predator_of_Fear/1273055", "EP", "Symphonic Death Metal"),
						Release::new("Bog Wizard", "Journey Through the Dying Lands").with_metallum("https://www.metal-archives.com/bands/Bog_Wizard/3540444101", "https://www.metal-archives.com/albums/Bog_Wizard/Journey_Through_the_Dying_Lands/1273973", "EP", "Stoner/Doom Metal")						,
						Release::new("Visions of Disfigurement", "Vile Mutation").with_metallum("https://www.metal-archives.com/bands/Visions_of_Disfigurement/3540389578", "https://www.metal-archives.com/albums/Visions_of_Disfigurement/Vile_Mutation/1274025", "Full-length", "Slam/Brutal Death Metal"),
						Release::new("Leatherhead", "Leatherhead").with_metallum("https://www.metal-archives.com/bands/Leatherhead/3540510256", "https://www.metal-archives.com/albums/Leatherhead/Leatherhead/1274470", "Full-length", "Heavy/Speed Metal"),
						Release::new("Centinex", "As You Die").with_metallum("https://www.metal-archives.com/bands/Centinex/633", "https://www.metal-archives.com/albums/Centinex/As_You_Die/1274900", "Single", "Death Metal"),
						Release::new("Weep", "The Constant Strain of Life").with_metallum("https://www.metal-archives.com/bands/Weep/3540518711", "https://www.metal-archives.com/albums/Weep/The_Constant_Strain_of_Life/1274911", "Full-length", "Black Metal/Shoegaze")						,
						Release::new("Zagan", "Total Suffering").with_metallum("https://www.metal-archives.com/bands/Zagan/3540372571", "https://www.metal-archives.com/albums/Zagan/Total_Suffering/1275447", "Full-length", "Black Metal"),
						Release::new("Recently Vacated Graves: True Zombie Metal", "Musk of Death").with_metallum("https://www.metal-archives.com/bands/Recently_Vacated_Graves%3A_True_Zombie_Metal/29827", "https://www.metal-archives.com/albums/Recently_Vacated_Graves%3A_True_Zombie_Metal/Musk_of_Death/1275785", "EP", "Death/Thrash Metal"),
						Release::new("Sordide", "Ainsi finit le jour").with_metallum("https://www.metal-archives.com/bands/Sordide/3540389183", "https://www.metal-archives.com/albums/Sordide/Ainsi_finit_le_jour/1276239", "Full-length", "Black Metal")	,
						Release::new("Extermination Dismemberment", "Butcher Basement (Revamp)").with_metallum("https://www.metal-archives.com/bands/Extermination_Dismemberment/3540318825", "https://www.metal-archives.com/albums/Extermination_Dismemberment/Butcher_Basement_%28Revamp%29/1276476", "Full-length", "Slam/Brutal Death Metal")						,
						Release::new("Sallow Moth", "Vial").with_metallum("https://www.metal-archives.com/bands/Sallow_Moth/3540438444", "https://www.metal-archives.com/albums/Sallow_Moth/Vial/1276786", "EP", "Death Metal"),
						Release::new("Draconicon", "A Symphony of Pestilence").with_metallum("https://www.metal-archives.com/bands/Draconicon/3540486854", "https://www.metal-archives.com/albums/Draconicon/A_Symphony_of_Pestilence/1277969", "Full-length", "Power Metal"),
						Release::new("Mordran", "One​-​and​-​Ninety Years of Darkness").with_metallum("https://www.metal-archives.com/bands/Mordran/3540496459", "https://www.metal-archives.com/albums/Mordran/One%E2%80%8B-%E2%80%8Band%E2%80%8B-%E2%80%8BNinety_Years_of_Darkness/1278163", "EP", "Depressive/Raw Atmospheric Black Metal/Dark Ambient"),
						Release::new("The Holy Flesh", "Advocate, Martyr and Redeemer").with_metallum("https://www.metal-archives.com/bands/The_Holy_Flesh/3540461827", "https://www.metal-archives.com/albums/The_Holy_Flesh/Advocate%2C_Martyr_and_Redeemer/1278257", "Full-length", "Atmospheric Black Metal")						,
						Release::new("Intöxicated", "Under the Sign of the Red Light").with_metallum("https://www.metal-archives.com/bands/Int%C3%B6xicated/3540299709", "https://www.metal-archives.com/albums/Int%C3%B6xicated/Under_the_Sign_of_the_Red_Light/1278465", "EP", "Speed/Thrash Metal"),
						Release::new("Lóstregos", "Nai").with_metallum("https://www.metal-archives.com/bands/L%C3%B3stregos/3540411010", "https://www.metal-archives.com/albums/L%C3%B3stregos/Nai/1279093", "Full-length", "Melodic/Pagan Black Metal"),
						Release::new("Solarnaut", "There's A Light In The Blur").with_metallum("https://www.metal-archives.com/bands/Solarnaut/3540447326", "https://www.metal-archives.com/albums/Solarnaut/There%27s_A_Light_In_The_Blur/1279871", "Full-length", "Stoner/Doom Metal"),
						Release::new("The Contagion", "Swept into Nothing").with_metallum("https://www.metal-archives.com/bands/The_Contagion/3540542586", "https://www.metal-archives.com/albums/The_Contagion/Swept_into_Nothing/1279936", "EP", "Death Metal"),
						Release::new("Traktat", "Dogmatic Accusations").with_metallum("https://www.metal-archives.com/bands/Traktat/3540531868", "https://www.metal-archives.com/albums/Traktat/Dogmatic_Accusations/1280174", "Full-length", "Black Metal"),
					]),
					(26, vec![
						Release::new("Helldrifter", "Dark Descent").with_metallum(
							"https://www.metal-archives.com/bands/Helldrifter/3540496814",
							"https://www.metal-archives.com/albums/Helldrifter/Dark_Descent/1265648",
							"EP",
							"Melodic Death Metal"
						),
						Release::new("Weight Shift", "Haled from Aether").with_metallum(
							"https://www.metal-archives.com/bands/Weight_Shift/3540511696",
							"https://www.metal-archives.com/albums/Weight_Shift/Haled_from_Aether/1278534",
							"Full-length",
							"Sludge/Doom Metal"
						),
						Release::new("Darkspell", "Victorious Reminiscent of Darkness").with_metallum(
							"https://www.metal-archives.com/bands/Darkspell/3540443025",
							"https://www.metal-archives.com/albums/Darkspell/Victorious_Reminiscent_of_Darkness/1280220",
							"EP",
							"Death/Thrash Metal"
						),
						Release::new("Messe Noire", "Ceremonial Death").with_metallum(
							"https://www.metal-archives.com/bands/Messe_Noire/3540519990",
							"https://www.metal-archives.com/albums/Messe_Noire/Ceremonial_Death/1280369",
							"Full-length",
							"Black Metal"
						  ),
					]),
					(27, vec![
						Release::new("Sukkubys", "Ma'am, Your Son Is Dead").with_metallum("https://www.metal-archives.com/bands/Sukkubys/3540540256", "https://www.metal-archives.com/albums/Sukkubys/Ma%27am%2C_Your_Son_Is_Dead/1273436", "Full-length", "Depressive Black Metal")						,
						Release::new("Snowman", "Dragon's Heart").with_metallum("https://www.metal-archives.com/bands/Snowman/3540436836", "https://www.metal-archives.com/albums/Snowman/Dragon%27s_Heart/1278945", "Single", "Symphonic Power Metal")						,
						Release::new("Pratanallis", "雨色Gentiana").with_metallum("https://www.metal-archives.com/bands/Pratanallis/3540454256", "https://www.metal-archives.com/albums/Pratanallis/%E9%9B%A8%E8%89%B2Gentiana/1279566", "Single", "Melodic/Symphonic Power Metal/Rock")						,
					]),
					(28, vec![
						Release::new("Kaivs", "After the Flesh").with_metallum("https://www.metal-archives.com/bands/Kaivs/3540519744", "https://www.metal-archives.com/albums/Kaivs/After_the_Flesh/1255256", "Full-length", "Death Metal"),
						Release::new("Rotgod", "Polemics and Obscenity - Part 2").with_metallum("https://www.metal-archives.com/bands/Rotgod/3540491454", "https://www.metal-archives.com/albums/Rotgod/Polemics_and_Obscenity_-_Part_2/1273061", "EP", "Thrash/Death Metal/Grindcore"),
						Release::new("Mental Torment", "Dead Shot Revival").with_metallum("https://www.metal-archives.com/bands/Mental_Torment/3540358661", "https://www.metal-archives.com/albums/Mental_Torment/Dead_Shot_Revival/1273415", "Full-length", "Death/Funeral Doom Metal"),
						Release::new("High Inquisitor Woe", "Painted Vision of an Era Forlorn").with_metallum("https://www.metal-archives.com/bands/High_Inquisitor_Woe/3540403857", "https://www.metal-archives.com/albums/High_Inquisitor_Woe/Painted_Vision_of_an_Era_Forlorn/1278786", "Full-length", "Doom Metal"),
						Release::new("Imagine a Boot", "Fearless Werewolf Killers").with_metallum("https://www.metal-archives.com/bands/Imagine_a_Boot/3540551374", "https://www.metal-archives.com/albums/Imagine_a_Boot/Fearless_Werewolf_Killers/1279292", "Demo", "Raw Black/Heavy Metal/Oi!"),
					]),
					(29, vec![
						Release::new("Vulgar Mephitis", "Demo 2024").with_metallum("https://www.metal-archives.com/bands/Vulgar_Mephitis/3540508775", "https://www.metal-archives.com/albums/Vulgar_Mephitis/Demo_2024/1279652", "Demo", "Brutal Death Metal"),
					]),
					(30, vec![
						Release::new("Necromoon", "War and Obedience").with_metallum("https://www.metal-archives.com/bands/Necromoon/3540545265", "https://www.metal-archives.com/albums/Necromoon/War_and_Obedience/1263644", "Compilation", "Black/Doom Metal")						,
						Release::new("Lay of the Autumn", "Of Love and Sorrow").with_metallum("https://www.metal-archives.com/bands/Lay_of_the_Autumn/3540550347", "https://www.metal-archives.com/albums/Lay_of_the_Autumn/Of_Love_and_Sorrow/1268770", "Full-length", "Symphonic Power Metal")						,
						Release::new("Weltschmerz", "III: Non Sequitur").with_metallum("https://www.metal-archives.com/bands/Weltschmerz/3540288722", "https://www.metal-archives.com/albums/Weltschmerz/III%3A_Non_Sequitur/1272712", "Full-length", "Black Metal")						,
						Release::new("DeadRipper", "Nightmare").with_metallum("https://www.metal-archives.com/bands/DeadRipper/3540519642", "https://www.metal-archives.com/albums/DeadRipper/Nightmare/1278505", "Full-length", "Black/Thrash Metal"),
						Release::new("Delusions of Godhood", "Salvation's Withdrawal").with_metallum("https://www.metal-archives.com/bands/Delusions_of_Godhood/3540397950", "https://www.metal-archives.com/albums/Delusions_of_Godhood/Salvation%27s_Withdrawal/1280060", "Full-length", "Melodic Death Metal")						,
					]),
					(31, vec![
						Release::new("Thine Inner Sanctum", "The Coming of the Dawn").with_metallum("https://www.metal-archives.com/bands/Thine_Inner_Sanctum/3540465817", "https://www.metal-archives.com/albums/Thine_Inner_Sanctum/The_Coming_of_the_Dawn/1216631", "Compilation", "Doom/Black Metal"),
						Release::new("Misantropical Painforest / W.A.I.L.", "Dare to Venture Down to Earth, Father! Perish into Nothingness").with_metallum("https://www.metal-archives.com/bands/Misantropical_Painforest/30945", "https://www.metal-archives.com/albums/Misantropical_Painforest_-_W.A.I.L./Dare_to_Venture_Down_to_Earth%2C_Father%21_Perish_into_Nothingness/1251538", "Split", "Epic Black Metal | Doom/Black/Death Metal"),
						Release::new("Aelvica", "Aelvica V: Vengeance").with_metallum("https://www.metal-archives.com/bands/Aelvica/3540517438", "https://www.metal-archives.com/albums/Aelvica/Aelvica_V%3A_Vengeance/1266353", "Full-length", "Black/Death Metal"),
						Release::new("Asgrauw", "Oorsprong").with_metallum("https://www.metal-archives.com/bands/Asgrauw/3540344621", "https://www.metal-archives.com/albums/Asgrauw/Oorsprong/1267751", "Full-length", "Black Metal"),
						Release::new("Sleepless", "Through Endless Black").with_metallum("https://www.metal-archives.com/bands/Sleepless/3540484422", "https://www.metal-archives.com/albums/Sleepless/Through_Endless_Black/1268093", "Full-length", "Technical Thrash Metal")						,
						Release::new("Summoning Death", "Tombs of the Blind Dead").with_metallum("https://www.metal-archives.com/bands/Summoning_Death/3540390270", "https://www.metal-archives.com/albums/Summoning_Death/Tombs_of_the_Blind_Dead/1268471", "Full-length", "Death Metal")						,
						Release::new("Goreatorium", "Vile​-​Lence").with_metallum("https://www.metal-archives.com/bands/Goreatorium/3540414223", "https://www.metal-archives.com/albums/Goreatorium/Vile%E2%80%8B-%E2%80%8BLence/1269092", "Full-length", "Death Metal/Goregrind")						,
						Release::new("Alien Carcass", "Entropic Visions of a Celestial Heaven").with_metallum("https://www.metal-archives.com/bands/Alien_Carcass/3540496967", "https://www.metal-archives.com/albums/Alien_Carcass/Entropic_Visions_of_a_Celestial_Heaven/1269102", "Full-length", "Black/Death Metal")						,
						Release::new("Slechtvalk", "At Death's Gate").with_metallum("https://www.metal-archives.com/bands/Slechtvalk/5957", "https://www.metal-archives.com/albums/Slechtvalk/At_Death%27s_Gate/1270255", "Full-length", "Melodic/Epic Black Metal")						,
						Release::new("Sorry...", "Drowned in Misery").with_metallum("https://www.metal-archives.com/bands/Sorry.../3540452576", "https://www.metal-archives.com/albums/Sorry.../Drowned_in_Misery/1271019", "Full-length", "Depressive Black Metal/Post-Punk")						,
						Release::new("Shaarimoth", "Devildom").with_metallum("https://www.metal-archives.com/bands/Shaarimoth/55213", "https://www.metal-archives.com/albums/Shaarimoth/Devildom/1271142", "Full-length", "Blackened Death Metal")						,
						Release::new("Tryblith", "Draconis Maleficium").with_metallum("https://www.metal-archives.com/bands/Tryblith/3540333217", "https://www.metal-archives.com/albums/Tryblith/Draconis_Maleficium/1271684", "Full-length", "Black Metal")						,
						Release::new("Dead Icarus", "Zealot").with_metallum("https://www.metal-archives.com/bands/Dead_Icarus/3540533195", "https://www.metal-archives.com/albums/Dead_Icarus/Zealot/1272707", "Full-length", "Metalcore/Groove Metal")						,
						Release::new("Dead Nexus", "Call of the Void").with_metallum("https://www.metal-archives.com/bands/Dead_Nexus/3540387088", "https://www.metal-archives.com/albums/Dead_Nexus/Call_of_the_Void/1272885", "EP", "Thrash/Death Metal")						,
						Release::new("Mälevolent", "Dark Tranquil Night").with_metallum("https://www.metal-archives.com/bands/M%C3%A4levolent/3540550580", "https://www.metal-archives.com/albums/M%C3%A4levolent/Dark_Tranquil_Night/1275019", "Full-length", "Black Metal")						,
						Release::new("Kre^u / Ticinum / Strja / Vrim", "Voces Antiqui Sanguinis").with_metallum("https://www.metal-archives.com/bands/Kre%5Eu/3540522700", "https://www.metal-archives.com/albums/Kre%5Eu_-_Ticinum_-_Strja_-_Vrim/Voces_Antiqui_Sanguinis/1276026", "Split", "Black Metal | Epic/Melodic Black Metal | Atmospheric Black Metal | Epic Black Metal")						,
						Release::new("Holy Death", "Sad But True").with_metallum("https://www.metal-archives.com/bands/Holy_Death/3540459748", "https://www.metal-archives.com/albums/Holy_Death/Sad_But_True/1276795", "EP", "Doom/Death Metal")						,
						Release::new("Foul Body Autopsy", "The Discovery of Witches").with_metallum("https://www.metal-archives.com/bands/Foul_Body_Autopsy/3540312483", "https://www.metal-archives.com/albums/Foul_Body_Autopsy/The_Discovery_of_Witches/1278026", "EP", "Death Metal (early); Melodic Death Metal (later)")						,
						Release::new("Sausage Wallet", "Vagpire").with_metallum("https://www.metal-archives.com/bands/Sausage_Wallet/3540538989", "https://www.metal-archives.com/albums/Sausage_Wallet/Vagpire/1278281", "EP", "Brutal Death Metal/Goregrind")						,
						Release::new("Nox Terror", "Frostbound Realm of the Dead").with_metallum("https://www.metal-archives.com/bands/Nox_Terror/3540481303", "https://www.metal-archives.com/albums/Nox_Terror/Frostbound_Realm_of_the_Dead/1278831", "Single", "Black Metal")						,
						Release::new("Picha", "Hecho picha").with_metallum("https://www.metal-archives.com/bands/Picha/3540427636", "https://www.metal-archives.com/albums/Picha/Hecho_picha/1279140", "Full-length", "Noisegrind (early); Death Metal/Goregrind (later)")						,
						Release::new("Visonfethacsis", "Waltzes in Daguerreotype").with_metallum("https://www.metal-archives.com/bands/Visonfethacsis/3540479184", "https://www.metal-archives.com/albums/Visonfethacsis/Waltzes_in_Daguerreotype/1280173", "Full-length", "Raw Black Metal")						,
						Release::new("Kerbmaldarr", "V").with_metallum("https://www.metal-archives.com/bands/Kerbmaldarr/3540465480", "https://www.metal-archives.com/albums/Kerbmaldarr/V/1280204", "EP", "Stoner Metal"),
					]),
				])),
				(Month::November, Releases::from([
					(1, vec![
						Release::new("Firemage", "Ignis").with_metallum("https://www.metal-archives.com/bands/Firemage/3540485750", "https://www.metal-archives.com/albums/Firemage/Ignis/1245724", "EP", "Folk Metal"),
						Release::new("Timo Tolkki", "Classical Variations and Themes 2: Ultima Thule").with_metallum("https://www.metal-archives.com/bands/Timo_Tolkki/2564", "https://www.metal-archives.com/albums/Timo_Tolkki/Classical_Variations_and_Themes_2%3A_Ultima_Thule/1246796", "Full-length", "Neoclassical Heavy Metal/Shred (early); Melodic Rock/Ambient (later)"),
						Release::new("Frostbite", "Relentless Grief").with_metallum("https://www.metal-archives.com/bands/Frostbite/3540359836", "https://www.metal-archives.com/albums/Frostbite/Relentless_Grief/1250374", "Full-length", "Black Metal"),
						Release::new("Brothers of Metal", "Fimbulvinter").with_metallum("https://www.metal-archives.com/bands/Brothers_of_Metal/3540425155", "https://www.metal-archives.com/albums/Brothers_of_Metal/Fimbulvinter/1255447", "Full-length", "Heavy/Power Metal"),
						Release::new("Bombus", "Your Blood").with_metallum("https://www.metal-archives.com/bands/Bombus/3540362871", "https://www.metal-archives.com/albums/Bombus/Your_Blood/1258108", "Full-length", "Heavy Metal/Hard Rock"),
						Release::new("Black Aeons", "Entering the Shadows").with_metallum("https://www.metal-archives.com/bands/Black_Aeons/3540512981", "https://www.metal-archives.com/albums/Black_Aeons/Entering_the_Shadows/1258542", "Full-length", "Black/Death Metal"),
						Release::new("Slaughter the Giant", "Abomination").with_metallum("https://www.metal-archives.com/bands/Slaughter_the_Giant/3540454219", "https://www.metal-archives.com/albums/Slaughter_the_Giant/Abomination/1259553", "Full-length", "Melodic Death/Black Metal"),
						Release::new("Carved Memories", "The Moirai").with_metallum("https://www.metal-archives.com/bands/Carved_Memories/3540424917", "https://www.metal-archives.com/albums/Carved_Memories/The_Moirai/1261274", "Full-length", "Melodic Death Metal"),
						Release::new("Vampirska", "A Liminal Heart Paints the Deepest Shade of Serenity").with_metallum("https://www.metal-archives.com/bands/Vampirska/3540467693", "https://www.metal-archives.com/albums/Vampirska/A_Liminal_Heart_Paints_the_Deepest_Shade_of_Serenity/1261367", "Full-length", "Raw Black Metal"),
						Release::new("Tommy Concrete", "Unrelapsed").with_metallum("https://www.metal-archives.com/bands/Tommy_Concrete/3540420016", "https://www.metal-archives.com/albums/Tommy_Concrete/Unrelapsed/1261525", "Full-length", "Progressive Metal"),
						Release::new("Nachtmystium", "Blight Privilege").with_metallum("https://www.metal-archives.com/bands/Nachtmystium/4866", "https://www.metal-archives.com/albums/Nachtmystium/Blight_Privilege/1261606", "Full-length", "Black Metal (early); Experimental/Psychedelic Black Metal (later)"),
						Release::new("Invictus", "Despair").with_metallum("https://www.metal-archives.com/bands/Invictus/3540515095", "https://www.metal-archives.com/albums/Invictus/Despair/1261794", "Full-length", "Melodic Death Metal/Metalcore"),
						Release::new("Stahlkeller", "Huckepack").with_metallum("https://www.metal-archives.com/bands/Stahlkeller/3540512770", "https://www.metal-archives.com/albums/Stahlkeller/Huckepack/1263502", "EP", "Thrash Metal"),
						Release::new("Wampyric Rites", "Summoning the Beasts in the Night of Lycanthropic Moon").with_metallum("https://www.metal-archives.com/bands/Wampyric_Rites/3540459552", "https://www.metal-archives.com/albums/Wampyric_Rites/Summoning_the_Beasts_in_the_Night_of_Lycanthropic_Moon/1263560", "EP", "Raw Black Metal"),
						Release::new("Mánþiel", "Odes Past & Mysticism from the Southern Lands").with_metallum("https://www.metal-archives.com/bands/M%C3%A1n%C3%BEiel/3540448717", "https://www.metal-archives.com/albums/M%C3%A1n%C3%BEiel/Odes_Past_%26_Mysticism_from_the_Southern_Lands/1263561", "Full-length", "Black Metal"),
						Release::new("Paganizer", "Flesh Requiem").with_metallum("https://www.metal-archives.com/bands/Paganizer/3370", "https://www.metal-archives.com/albums/Paganizer/Flesh_Requiem/1263831", "Full-length", "Death Metal"),
						Release::new("Dragoncorpse", "The Fall of House Abbarath").with_metallum("https://www.metal-archives.com/bands/Dragoncorpse/3540507059", "https://www.metal-archives.com/albums/Dragoncorpse/The_Fall_of_House_Abbarath/1270627", "EP", "Symphonic Metalcore/Power Metal"),
						Release::new("Gravekvlt", "Full Moon Fever").with_metallum("https://www.metal-archives.com/bands/Gravekvlt/3540515290", "https://www.metal-archives.com/albums/Gravekvlt/Full_Moon_Fever/1270714", "Full-length", "Black 'n' Roll"),
						Release::new("Everto Signum", "Beastiary").with_metallum("https://www.metal-archives.com/bands/Everto_Signum/3540346565", "https://www.metal-archives.com/albums/Everto_Signum/Beastiary/1270760", "Full-length", "Black Metal"),
						Release::new("Tribulation", "Sub Rosa in Aeternum").with_metallum("https://www.metal-archives.com/bands/Tribulation/39589", "https://www.metal-archives.com/albums/Tribulation/Sub_Rosa_in_Aeternum/1270805", "Full-length", "Death Metal (early); Gothic Metal (later)"),
						Release::new("Viikate", "Hiljainen").with_metallum("https://www.metal-archives.com/bands/Viikate/15287", "https://www.metal-archives.com/albums/Viikate/Hiljainen/1271760", "Full-length", "Melodic Heavy Metal with Schlager influences"),
						Release::new("Burning Sky", "Despair Of The Damned").with_metallum("https://www.metal-archives.com/bands/Burning_Sky/3540528550", "https://www.metal-archives.com/albums/Burning_Sky/Despair_Of_The_Damned/1272551", "Live album", "Death Metal/Hardcore/Sludge"),
						Release::new("Skullovich", "The Age of Steel").with_metallum("https://www.metal-archives.com/bands/Skullovich/3540481741", "https://www.metal-archives.com/albums/Skullovich/The_Age_of_Steel/1272997", "Full-length", "Thrash/Speed Metal"),
						Release::new("Thyrathen", "Lakonic").with_metallum("https://www.metal-archives.com/bands/Thyrathen/3540497806", "https://www.metal-archives.com/albums/Thyrathen/Lakonic/1273262", "Full-length", "Black Metal"),
						Release::new("Necrotic Divinity", "Morbid Fascination").with_metallum("https://www.metal-archives.com/bands/Necrotic_Divinity/3540498194", "https://www.metal-archives.com/albums/Necrotic_Divinity/Morbid_Fascination/1273479", "EP", "Death Metal"),
						Release::new("Splendidula", "Behind My Semblance").with_metallum("https://www.metal-archives.com/bands/Splendidula/3540365081", "https://www.metal-archives.com/albums/Splendidula/Behind_My_Semblance/1273831", "Single", "Stoner/Doom/Sludge Metal"),
						Release::new("Anomalie", "Riverchild").with_metallum("https://www.metal-archives.com/bands/Anomalie/3540376036", "https://www.metal-archives.com/albums/Anomalie/Riverchild/1273838", "Full-length", "Post-Black Metal"),
						Release::new("Cień", "Maledicto").with_metallum("https://www.metal-archives.com/bands/Cie%C5%84/3540327513", "https://www.metal-archives.com/albums/Cie%C5%84/Maledicto/1274381", "Full-length", "Black Metal"),
						Release::new("Vessel", "The Somnifer").with_metallum("https://www.metal-archives.com/bands/Vessel/3540356373", "https://www.metal-archives.com/albums/Vessel/The_Somnifer/1274456", "Full-length", "Heavy Metal (early); Stoner Metal/Rock (later)"),
						Release::new("Assassin", "Skullblast").with_metallum("https://www.metal-archives.com/bands/Assassin/642", "https://www.metal-archives.com/albums/Assassin/Skullblast/1274582", "EP", "Thrash Metal"),
						Release::new("Vimbulnatt", "Der dunklen Tugenden. Der Urgrund").with_metallum("https://www.metal-archives.com/bands/Vimbulnatt/3540465905", "https://www.metal-archives.com/albums/Vimbulnatt/Der_dunklen_Tugenden._Der_Urgrund/1274756", "Full-length", "Black Metal"),
						Release::new("InnerWish", "Ash of Eternal Flame").with_metallum("https://www.metal-archives.com/bands/InnerWish/14916", "https://www.metal-archives.com/albums/InnerWish/Ash_of_Eternal_Flame/1274878", "Full-length", "Heavy/Power Metal"),
						Release::new("Epitaph", "Path to Oblivion").with_metallum("https://www.metal-archives.com/bands/Epitaph/100645", "https://www.metal-archives.com/albums/Epitaph/Path_to_Oblivion/1275170", "Full-length", "Doom Metal"),
						Release::new("Anthesis", "Tension Between Rot and Genesis").with_metallum("https://www.metal-archives.com/bands/Anthesis/3540257804", "https://www.metal-archives.com/albums/Anthesis/Tension_Between_Rot_and_Genesis/1275902", "Full-length", "Sludge Metal/Grindcore/Hardcore"),
						Release::new("Qaalm", "Grave Impressions of an Unbroken Arc").with_metallum("https://www.metal-archives.com/bands/Qaalm/3540491272", "https://www.metal-archives.com/albums/Qaalm/Grave_Impressions_of_an_Unbroken_Arc/1276584", "Full-length", "Sludge/Doom Metal"),
						Release::new("From the Vastland", "Tenebrous Shadow").with_metallum("https://www.metal-archives.com/bands/From_the_Vastland/3540331603", "https://www.metal-archives.com/albums/From_the_Vastland/Tenebrous_Shadow/1277031", "Full-length", "Black Metal"),
						Release::new("DreamLongDead", "Derelict").with_metallum("https://www.metal-archives.com/bands/DreamLongDead/3540353184", "https://www.metal-archives.com/albums/DreamLongDead/Derelict/1277731", "Full-length", "Doom/Death Metal"),
						Release::new("Rotborn", "Shrapnels of a Panic Spiral").with_metallum("https://www.metal-archives.com/bands/Rotborn/3540500762", "https://www.metal-archives.com/albums/Rotborn/Shrapnels_of_a_Panic_Spiral/1278029", "Full-length", "Death Metal"),
						Release::new("Mitochondrion", "Vitriseptome").with_metallum("https://www.metal-archives.com/bands/Mitochondrion/14732", "https://www.metal-archives.com/albums/Mitochondrion/Vitriseptome/1278033", "Full-length", "Death/Black Metal"),
						Release::new("The Bottle Doom Lazy Band", "Clans of the Alphane Moon").with_metallum("https://www.metal-archives.com/bands/The_Bottle_Doom_Lazy_Band/95783", "https://www.metal-archives.com/albums/The_Bottle_Doom_Lazy_Band/Clans_of_the_Alphane_Moon/1278467", "Full-length", "Doom Metal"),
						Release::new("Dying Hydra", "Strange and Beautiful Things").with_metallum("https://www.metal-archives.com/bands/Dying_Hydra/3540441179", "https://www.metal-archives.com/albums/Dying_Hydra/Strange_and_Beautiful_Things/1278651", "EP", "Atmospheric Sludge Metal"),
						Release::new("Nolove", "Alone / Forgive me...").with_metallum("https://www.metal-archives.com/bands/Nolove/3540531420", "https://www.metal-archives.com/albums/Nolove/Alone_-_Forgive_me.../1278798", "Split", "Experimental/Depressive Black Metal, Post-Rock"),
						Release::new("Cryptic Brood", "Necrotic Flesh Bacteria").with_metallum("https://www.metal-archives.com/bands/Cryptic_Brood/3540371413", "https://www.metal-archives.com/albums/Cryptic_Brood/Necrotic_Flesh_Bacteria/1278879", "Full-length", "Death/Doom Metal"),
						Release::new("Methchrist", "Acephalic Thanatocracy").with_metallum("https://www.metal-archives.com/bands/Methchrist/3540440713", "https://www.metal-archives.com/albums/Methchrist/Acephalic_Thanatocracy/1278958", "EP", "Black/Death Metal"),
						Release::new("Crucifixion Ritual", "Desecration of the Angels").with_metallum("https://www.metal-archives.com/bands/Crucifixion_Ritual/3540488301", "https://www.metal-archives.com/albums/Crucifixion_Ritual/Desecration_of_the_Angels/1279047", "EP", "Death/Black Metal"),
						Release::new("Angantyr", "Indsigt").with_metallum("https://www.metal-archives.com/bands/Angantyr/3987", "https://www.metal-archives.com/albums/Angantyr/Indsigt/1279184", "Full-length", "Black Metal"),
						Release::new("Trillion Ton Beryllium Ships", "The Mind Like Fire Unbound").with_metallum("https://www.metal-archives.com/bands/Trillion_Ton_Beryllium_Ships/3540489751", "https://www.metal-archives.com/albums/Trillion_Ton_Beryllium_Ships/The_Mind_Like_Fire_Unbound/1279233", "Full-length", "Stoner/Doom Metal"),
						Release::new("Ian Highhill", "Death Sentence").with_metallum("https://www.metal-archives.com/bands/Ian_Highhill/3540483797", "https://www.metal-archives.com/albums/Ian_Highhill/Death_Sentence/1279615", "Single", "Heavy/Doom Metal/Hard Rock"),
						Release::new("Children of the Frost", "Last Winter's Child").with_metallum("https://www.metal-archives.com/bands/Children_of_the_Frost/3540466099", "https://www.metal-archives.com/albums/Children_of_the_Frost/Last_Winter%27s_Child/1279755", "EP", "Symphonic Metal"),
						Release::new("The Fallen Prophets", "Primordial Instinct").with_metallum("https://www.metal-archives.com/bands/The_Fallen_Prophets/3540408648", "https://www.metal-archives.com/albums/The_Fallen_Prophets/Primordial_Instinct/1279756", "EP", "Melodic Death Metal/Deathcore"),
						Release::new("Frankenbok", "Demon Tantrum").with_metallum("https://www.metal-archives.com/bands/Frankenbok/114192", "https://www.metal-archives.com/albums/Frankenbok/Demon_Tantrum/1279921", "Single", "Groove Metal"),
						Release::new("Putridarium", "Necrologia del sadismo: Excerpts from a Deranged Mind").with_metallum("https://www.metal-archives.com/bands/Putridarium/3540497604", "https://www.metal-archives.com/albums/Putridarium/Necrologia_del_sadismo%3A_Excerpts_from_a_Deranged_Mind/1280013", "Full-length", "Death/Doom Metal"),
						Release::new("Recidivist", "Madness Malformed").with_metallum("https://www.metal-archives.com/bands/Recidivist/3540507967", "https://www.metal-archives.com/albums/Recidivist/Madness_Malformed/1280166", "Full-length", "Death Metal"),
						Release::new("Drift of Genes", "Room").with_metallum("https://www.metal-archives.com/bands/Drift_of_Genes/3540350103", "https://www.metal-archives.com/albums/Drift_of_Genes/Room/1280196", "Full-length", "Brutal Death Metal"),
					]),
					(2, vec![
						Release::new("Ethereal", "Downfall").with_metallum("https://www.metal-archives.com/bands/Ethereal/7428", "https://www.metal-archives.com/albums/Ethereal/Downfall/1256633", "Full-length", "Progressive Gothic/Doom Metal"),
						Release::new("Lenguaje de Viboras", "Kira").with_metallum("https://www.metal-archives.com/bands/Lenguaje_de_Viboras/3540500217", "https://www.metal-archives.com/albums/Lenguaje_de_Viboras/Kira/1262099", "EP", "Sludge/Stoner Metal"),
						Release::new("Raptore", "Renaissance").with_metallum("https://www.metal-archives.com/bands/Raptore/3540383257", "https://www.metal-archives.com/albums/Raptore/Renaissance/1270237", "Full-length", "Heavy Metal"),
						Release::new("Abomination Impurity", "Crawling In The Depth").with_metallum("https://www.metal-archives.com/bands/Abomination_Impurity/3540424519", "https://www.metal-archives.com/albums/Abomination_Impurity/Crawling_In_The_Depth/1274915", "EP", "Brutal Death Metal"),
					]),
					(3, vec![
						Release::new("Deadspace", "The Dark Enlightenment").with_metallum("https://www.metal-archives.com/bands/Deadspace/3540395373", "https://www.metal-archives.com/albums/Deadspace/The_Dark_Enlightenment/1274150", "Full-length", "Depressive Black/Gothic Metal"),
						Release::new("Steam Slicer", "Beyond the Rivers").with_metallum("https://www.metal-archives.com/bands/Steam_Slicer/3540526163", "https://www.metal-archives.com/albums/Steam_Slicer/Beyond_the_Rivers/1277965", "Full-length", "Progressive Metal"),
					]),
					(6, vec![
						Release::new("Naoki Morioka", "Absolutes").with_metallum("https://www.metal-archives.com/bands/Naoki_Morioka/3540540627", "https://www.metal-archives.com/albums/Naoki_Morioka/Absolutes/1271798", "EP", "Progressive/Power Metal"),
					]),
					(7, vec![
						Release::new("Suidakra", "Darkanakrad").with_metallum("https://www.metal-archives.com/bands/Suidakra/487", "https://www.metal-archives.com/albums/Suidakra/Darkanakrad/1273715", "Full-length", "Melodic Death/Black/Folk Metal")						,
					]),
					(8, vec![
						Release::new("Tungsten", "The Grand Inferno").with_metallum("https://www.metal-archives.com/bands/Tungsten/3540508487", "https://www.metal-archives.com/albums/Tungsten/The_Grand_Inferno/1237489", "Full-length", "Heavy/Power Metal, Hard Rock")						,
						Release::new("Distant Past", "Solaris").with_metallum("https://www.metal-archives.com/bands/Distant_Past/3540310836", "https://www.metal-archives.com/albums/Distant_Past/Solaris/1239684", "Full-length", "Progressive/Melodic Heavy Metal"),
						Release::new("Earthburner", "Permanent Dawn").with_metallum("https://www.metal-archives.com/bands/Earthburner/3540445126", "https://www.metal-archives.com/albums/Earthburner/Permanent_Dawn/1254552", "Full-length", "Death Metal/Grindcore")						,
						Release::new("Klone", "The Unseen").with_metallum("https://www.metal-archives.com/bands/Klone/18519", "https://www.metal-archives.com/albums/Klone/The_Unseen/1259421", "Full-length", "Progressive Groove Metal (early); Progressive Metal/Rock (later)")						,
						Release::new("Molder", "Catastrophic Reconfiguration").with_metallum("https://www.metal-archives.com/bands/Molder/3540437246", "https://www.metal-archives.com/albums/Molder/Catastrophic_Reconfiguration/1260154", "Full-length", "Death/Thrash Metal"),
						Release::new("Make Them Suffer", "Make Them Suffer").with_metallum("https://www.metal-archives.com/bands/Make_Them_Suffer/3540328594", "https://www.metal-archives.com/albums/Make_Them_Suffer/Make_Them_Suffer/1262589", "Full-length", "Symphonic Deathcore (early); Deathcore/Metalcore (later)")						,
						Release::new("Sólstafir", "Hin helga kv​ö​l").with_metallum("https://www.metal-archives.com/bands/S%C3%B3lstafir/3213", "https://www.metal-archives.com/albums/S%C3%B3lstafir/Hin_helga_kv%E2%80%8B%C3%B6%E2%80%8Bl/1263912", "Full-length", "Viking/Black Metal (early); Post-Metal/Rock (later)")						,
						Release::new("Yoth Iria", "Blazing Inferno").with_metallum("https://www.metal-archives.com/bands/Yoth_Iria/3540451390", "https://www.metal-archives.com/albums/Yoth_Iria/Blazing_Inferno/1266395", "Full-length", "Black Metal")						,
						Release::new("Valontuoja", "Luonnon armoilla").with_metallum("https://www.metal-archives.com/bands/Valontuoja/3540549976", "https://www.metal-archives.com/albums/Valontuoja/Luonnon_armoilla/1266689", "Full-length", "Black Metal"),
						Release::new("Ad Vitam Infernal", "Le ballet des anges").with_metallum("https://www.metal-archives.com/bands/Ad_Vitam_Infernal/3540461752", "https://www.metal-archives.com/albums/Ad_Vitam_Infernal/Le_ballet_des_anges/1269921", "Full-length", "Death Metal")						,
						Release::new("The Body", "The Crying Out of Things").with_metallum("https://www.metal-archives.com/bands/The_Body/51680", "https://www.metal-archives.com/albums/The_Body/The_Crying_Out_of_Things/1270216", "Full-length", "Experimental Sludge/Doom Metal/Noise/Industrial")						,
						Release::new("Delain", "Dance with the Devil").with_metallum("https://www.metal-archives.com/bands/Delain/10897", "https://www.metal-archives.com/albums/Delain/Dance_with_the_Devil/1270363", "EP", "Symphonic Metal/Rock")						,
						Release::new("Witchpit", "Forever Spoken").with_metallum("https://www.metal-archives.com/bands/Witchpit/3540498556", "https://www.metal-archives.com/albums/Witchpit/Forever_Spoken/1270486", "Full-length", "Stoner/Sludge Metal")						,
						Release::new("Chaos Invocation", "Wherever We Roam...").with_metallum("https://www.metal-archives.com/bands/Chaos_Invocation/3540292309", "https://www.metal-archives.com/albums/Chaos_Invocation/Wherever_We_Roam.../1270511", "Full-length", "Black Metal")						,
						Release::new("Paragon", "Metalation").with_metallum("https://www.metal-archives.com/bands/Paragon/365", "https://www.metal-archives.com/albums/Paragon/Metalation/1270513", "Full-length", "Power/Speed Metal"),
						Release::new("Massacre", "Necrolution").with_metallum("https://www.metal-archives.com/bands/Massacre/281", "https://www.metal-archives.com/albums/Massacre/Necrolution/1270551", "Full-length", "Death Metal"),
						Release::new("Witnesses", "Joy").with_metallum("https://www.metal-archives.com/bands/Witnesses/3540450514", "https://www.metal-archives.com/albums/Witnesses/Joy/1270803", "Full-length", "Ambient/Electronic, Melodic Doom Metal"),
						Release::new("Impellitteri", "War Machine").with_metallum("https://www.metal-archives.com/bands/Impellitteri/320", "https://www.metal-archives.com/albums/Impellitteri/War_Machine/1270894", "Full-length", "Heavy/Power Metal/Shred")						,
						Release::new("Stranger Vision", "Faust - Act​​ I Prelude to Darkness").with_metallum("https://www.metal-archives.com/bands/Stranger_Vision/3540485212", "https://www.metal-archives.com/albums/Stranger_Vision/Faust_-_Act%E2%80%8B%E2%80%8B_I_Prelude_to_Darkness/1272179", "Full-length", "Melodic Heavy Metal")						,
						Release::new("Codespeaker", "Scavenger").with_metallum("https://www.metal-archives.com/bands/Codespeaker/3540514945", "https://www.metal-archives.com/albums/Codespeaker/Scavenger/1272427", "Full-length", "Sludge/Post-Metal")						,
						Release::new("Alarum", "Recontinue").with_metallum("https://www.metal-archives.com/bands/Alarum/2352", "https://www.metal-archives.com/albums/Alarum/Recontinue/1272934", "Full-length", "Progressive/Thrash Metal/Fusion")						,
						Release::new("Ershetu", "Yomi").with_metallum("https://www.metal-archives.com/bands/Ershetu/3540532792", "https://www.metal-archives.com/albums/Ershetu/Yomi/1274010", "Full-length", "Progressive Black Metal")						,
						Release::new("Disparaged", "Down the Heavens").with_metallum("https://www.metal-archives.com/bands/Disparaged/13515", "https://www.metal-archives.com/albums/Disparaged/Down_the_Heavens/1274529", "Full-length", "Death Metal")						,
						Release::new("Moss upon the Skull", "Quest for the Secret Fire").with_metallum("https://www.metal-archives.com/bands/Moss_upon_the_Skull/3540389867", "https://www.metal-archives.com/albums/Moss_upon_the_Skull/Quest_for_the_Secret_Fire/1274611", "Full-length", "Progressive Death Metal"),
						Release::new("Ploughshare", "Second Wound").with_metallum("https://www.metal-archives.com/bands/Ploughshare/3540426072", "https://www.metal-archives.com/albums/Ploughshare/Second_Wound/1274710", "Full-length", "Black/Death Metal"),
						Release::new("Shrykull", "Beyond Subconscious Realms").with_metallum("https://www.metal-archives.com/bands/Shrykull/3540420154", "https://www.metal-archives.com/albums/Shrykull/Beyond_Subconscious_Realms/1274717", "Full-length", "Black/Death Metal/Grindcore")						,
						Release::new("Valkyrie's Fire", "Ascension").with_metallum("https://www.metal-archives.com/bands/Valkyrie%27s_Fire/3540551424", "https://www.metal-archives.com/albums/Valkyrie%27s_Fire/Ascension/1275151", "EP", "Symphonic Power Metal")						,
						Release::new("Seven Kingdoms", "The Square").with_metallum("https://www.metal-archives.com/bands/Seven_Kingdoms/108082", "https://www.metal-archives.com/albums/Seven_Kingdoms/The_Square/1275197", "EP", "Power/Thrash Metal")						,
						Release::new("Legendarium", "For Eternal Glory").with_metallum("https://www.metal-archives.com/bands/Legendarium/3540453156", "https://www.metal-archives.com/albums/Legendarium/For_Eternal_Glory/1275672", "Full-length", "Melodic Death/Heavy Metal"),
						Release::new("Tenebrisme", "Sisyphe").with_metallum("https://www.metal-archives.com/bands/Tenebrisme/3540513859", "https://www.metal-archives.com/albums/Tenebrisme/Sisyphe/1278684", "Full-length", "Atmospheric Black Metal"),
						Release::new("Nurcry", "Renacer").with_metallum("https://www.metal-archives.com/bands/Nurcry/3540513088", "https://www.metal-archives.com/albums/Nurcry/Renacer/1279844", "Full-length", "Heavy/Power Metal")						,
					]),
					(9, vec![
						Release::new("Morgue Walker", "No One Left Alive").with_metallum("https://www.metal-archives.com/bands/Morgue_Walker/3540437978", "https://www.metal-archives.com/albums/Morgue_Walker/No_One_Left_Alive/1276734", "EP", "Blackened Death Metal/Grindcore"),
					]),
					(10, vec![
						Release::new("Hamerhaai", "Tand om Tand").with_metallum("https://www.metal-archives.com/bands/Hamerhaai/3540503680", "https://www.metal-archives.com/albums/Hamerhaai/Tand_om_Tand/1280316", "EP", "Death Metal/Hardcore"),
					]),
					(11, vec![
						Release::new("Blaze the Thunder", "The Bewildered Herd").with_metallum("https://www.metal-archives.com/bands/Blaze_the_Thunder/3540455595", "https://www.metal-archives.com/albums/Blaze_the_Thunder/The_Bewildered_Herd/1219700", "Full-length", "Heavy Metal/Punk"),
						Release::new("Forja", "Món oblidat").with_metallum("https://www.metal-archives.com/bands/Forja/3540372628", "https://www.metal-archives.com/albums/Forja/M%C3%B3n_oblidat/1240116", "Full-length", "Folk/Power Metal"),
						Release::new("Super Monster Party", "Rage Quit").with_metallum("https://www.metal-archives.com/bands/Super_Monster_Party/3540486947", "https://www.metal-archives.com/albums/Super_Monster_Party/Rage_Quit/1270303", "Full-length", "Progressive Heavy/Power/Death Metal"),
						Release::new("Succumbence", "Succumbence").with_metallum("https://www.metal-archives.com/bands/Succumbence/3540549030", "https://www.metal-archives.com/albums/Succumbence/Succumbence/1272810", "Full-length", "Black Metal, Folk/Ambient"),
					]),
					(12, vec![
						Release::new("Apocryphal", "Facing the End").with_metallum("https://www.metal-archives.com/bands/Apocryphal/23658", "https://www.metal-archives.com/albums/Apocryphal/Facing_the_End/1254484", "Full-length", "Atmospheric Death Metal")						,
						Release::new("Gauntlet Rule", "After the Kill").with_metallum("https://www.metal-archives.com/bands/Gauntlet_Rule/3540488334", "https://www.metal-archives.com/albums/Gauntlet_Rule/After_the_Kill/1270575", "Full-length", "Heavy Metal")						,
						Release::new("Space Mirrors", "Nexus Between Space and Art").with_metallum("https://www.metal-archives.com/bands/Space_Mirrors/88857", "https://www.metal-archives.com/albums/Space_Mirrors/Nexus_Between_Space_and_Art/1278530", "Full-length", "Progressive Metal/Space Rock")						,
						Release::new("A la Carte", "Born to Entertain").with_metallum("https://www.metal-archives.com/bands/A_la_Carte/3540510514", "https://www.metal-archives.com/albums/A_la_Carte/Born_to_Entertain/1278646", "Full-length", "Death Metal")						,
						Release::new("Doubting Thompson", "Lizard Brain Directives").with_metallum("https://www.metal-archives.com/bands/Doubting_Thompson/3540439679", "https://www.metal-archives.com/albums/Doubting_Thompson/Lizard_Brain_Directives/1280020", "EP", "Thrash Metal/Crossover"),
					]),
					(13, vec![
						Release::new("Incisor", "Harvester Of Indecent Letany").with_metallum("https://www.metal-archives.com/bands/Incisor/3540276836", "https://www.metal-archives.com/albums/Incisor/Harvester_Of_Indecent_Letany/1280142", "Compilation", "Death Metal"),
					]),
					(14, vec![
						Release::new("Kromlek", "III-III & Upphaf").with_metallum(
							"https://www.metal-archives.com/bands/Kromlek/54054",
							"https://www.metal-archives.com/albums/Kromlek/III-III_%26_Upphaf/1274743",
							"Full-length",
							"Viking/Folk/Black Metal"
						),
						Release::new("Lying Figures", "Inheritance").with_metallum(
							"https://www.metal-archives.com/bands/Lying_Figures/3540367177",
							"https://www.metal-archives.com/albums/Lying_Figures/Inheritance/1277142",
							"Full-length",
							"Doom/Death Metal"
						  ),
					]),
					(15, vec![
						Release::new("Warfarer", "A Tale Beyond the Pale").with_metallum("https://www.metal-archives.com/bands/Warfarer/3540462212", "https://www.metal-archives.com/albums/Warfarer/A_Tale_Beyond_the_Pale/1256668", "Full-length", "Melodic Death/Folk Metal"),
						Release::new("Odyrmos", "The Neverending Journey").with_metallum("https://www.metal-archives.com/bands/Odyrmos/3540490499", "https://www.metal-archives.com/albums/Odyrmos/The_Neverending_Journey/1265060", "Demo", "Atmospheric Black Metal, Dark Ambient"),
						Release::new("Aptorian Demon", "Liv tar slutt").with_metallum("https://www.metal-archives.com/bands/Aptorian_Demon/45799", "https://www.metal-archives.com/albums/Aptorian_Demon/Liv_tar_slutt/1265538", "Full-length", "Black Metal"),
						Release::new("Thy Catafalque", "XII: A gyönyörü álmok ezután jönnek").with_metallum("https://www.metal-archives.com/bands/Thy_Catafalque/31620", "https://www.metal-archives.com/albums/Thy_Catafalque/XII%3A_A_gy%C3%B6ny%C3%B6r%C3%BC_%C3%A1lmok_ezut%C3%A1n_j%C3%B6nnek/1265915", "Full-length", "Avant-garde Metal"),
						Release::new("Toxaemia", "Rejected Souls of Kerberus").with_metallum("https://www.metal-archives.com/bands/Toxaemia/21464", "https://www.metal-archives.com/albums/Toxaemia/Rejected_Souls_of_Kerberus/1266096", "Full-length", "Death Metal"),
						Release::new("Veilburner", "The Duality of Decapitation and Wisdom").with_metallum("https://www.metal-archives.com/bands/Veilburner/3540385778", "https://www.metal-archives.com/albums/Veilburner/The_Duality_of_Decapitation_and_Wisdom/1269329", "Full-length", "Black/Death Metal"),
						Release::new("Starchaser", "Into the Great Unknown").with_metallum("https://www.metal-archives.com/bands/Starchaser/3540505939", "https://www.metal-archives.com/albums/Starchaser/Into_the_Great_Unknown/1269976", "Full-length", "Heavy Metal"),
						Release::new("The Foreshadowing", "New Wave Order").with_metallum("https://www.metal-archives.com/bands/The_Foreshadowing/108312", "https://www.metal-archives.com/albums/The_Foreshadowing/New_Wave_Order/1270024", "Full-length", "Gothic/Doom Metal"),
						Release::new("The Mosaic Window", "Hemasanctum").with_metallum("https://www.metal-archives.com/bands/The_Mosaic_Window/3540494835", "https://www.metal-archives.com/albums/The_Mosaic_Window/Hemasanctum/1270721", "Full-length", "Melodic Black Metal"),
						Release::new("Mammoth Grinder", "Undying Spectral Resonance").with_metallum("https://www.metal-archives.com/bands/Mammoth_Grinder/3540294181", "https://www.metal-archives.com/albums/Mammoth_Grinder/Undying_Spectral_Resonance/1270876", "EP", "Hardcore Punk/Sludge Metal (early); Death Metal/Hardcore (later)"),
						Release::new("Tribal Gaze / Deadbody", "Deadbody / Tribal Gaze").with_metallum("https://www.metal-archives.com/bands/Tribal_Gaze/3540483981", "https://www.metal-archives.com/albums/Tribal_Gaze_-_Deadbody/Deadbody_-_Tribal_Gaze/1272378", "Split", "Death Metal | Death Metal/Hardcore"),
						Release::new("Monolithe", "Black Hole District").with_metallum("https://www.metal-archives.com/bands/Monolithe/13707", "https://www.metal-archives.com/albums/Monolithe/Black_Hole_District/1272417", "Full-length", "Funeral Doom Metal (early); Melodic Death/Doom Metal (later)"),
						Release::new("As I Lay Dying", "Through Storms Ahead").with_metallum("https://www.metal-archives.com/bands/As_I_Lay_Dying/20825", "https://www.metal-archives.com/albums/As_I_Lay_Dying/Through_Storms_Ahead/1272713", "Full-length", "Metalcore"),
						Release::new("Faüst", "Death Galore").with_metallum("https://www.metal-archives.com/bands/Fa%C3%BCst/3540473797", "https://www.metal-archives.com/albums/Fa%C3%BCst/Death_Galore/1272986", "Full-length", "Thrash Metal"),
						Release::new("Worm Shepherd", "Hunger").with_metallum("https://www.metal-archives.com/bands/Worm_Shepherd/3540500546", "https://www.metal-archives.com/albums/Worm_Shepherd/Hunger/1274050", "Full-length", "Symphonic Deathcore"),
						Release::new("Thanatos", "Four Decades of Death").with_metallum("https://www.metal-archives.com/bands/Thanatos/293", "https://www.metal-archives.com/albums/Thanatos/Four_Decades_of_Death/1274598", "Compilation", "Death/Thrash Metal"),
						Release::new("Synthwailer", "Cruciform").with_metallum("https://www.metal-archives.com/bands/Synthwailer/3540486233", "https://www.metal-archives.com/albums/Synthwailer/Cruciform/1275480", "Full-length", "Symphonic Power/Heavy Metal"),
						Release::new("Nolove", "La mort nous a séparés").with_metallum("https://www.metal-archives.com/bands/Nolove/3540531420", "https://www.metal-archives.com/albums/Nolove/La_mort_nous_a_s%C3%A9par%C3%A9s/1275727", "Single", "Experimental/Depressive Black Metal, Post-Rock"),
						Release::new("Empires of Eden", "Guardians of Time").with_metallum("https://www.metal-archives.com/bands/Empires_of_Eden/3540282087", "https://www.metal-archives.com/albums/Empires_of_Eden/Guardians_of_Time/1277228", "Full-length", "Melodic Power Metal"),
						Release::new("Time Lurker", "Emprise").with_metallum("https://www.metal-archives.com/bands/Time_Lurker/3540419145", "https://www.metal-archives.com/albums/Time_Lurker/Emprise/1277910", "Full-length", "Atmospheric Black Metal"),
						Release::new("Trollcave", "Adoration of the Abyssal Trespasser").with_metallum("https://www.metal-archives.com/bands/Trollcave/3540499736", "https://www.metal-archives.com/albums/Trollcave/Adoration_of_the_Abyssal_Trespasser/1278354", "EP", "Funeral Doom/Death Metal"),
						Release::new("Wasted Youth", "Young and Bored - The Complete Wasted Youth").with_metallum("https://www.metal-archives.com/bands/Wasted_Youth/14351", "https://www.metal-archives.com/albums/Wasted_Youth/Young_and_Bored_-_The_Complete_Wasted_Youth/1278681", "Compilation", "Hardcore Punk (early); Thrash Metal (later)"),
						Release::new("Primal Code", "Opaque Fixation").with_metallum("https://www.metal-archives.com/bands/Primal_Code/3540509020", "https://www.metal-archives.com/albums/Primal_Code/Opaque_Fixation/1278865", "Full-length", "Death Metal"),
						Release::new("Opus Irae", "Into the Endless Night").with_metallum("https://www.metal-archives.com/bands/Opus_Irae/3540405662", "https://www.metal-archives.com/albums/Opus_Irae/Into_the_Endless_Night/1279021", "Full-length", "Symphonic Black Metal"),
						Release::new("Spider God", "Possess the Devil").with_metallum("https://www.metal-archives.com/bands/Spider_God/3540476120", "https://www.metal-archives.com/albums/Spider_God/Possess_the_Devil/1279119", "Full-length", "Melodic Black Metal"),
						Release::new("Thunder and Lightning", "Of Wrath and Ruin").with_metallum("https://www.metal-archives.com/bands/Thunder_and_Lightning/50661", "https://www.metal-archives.com/albums/Thunder_and_Lightning/Of_Wrath_and_Ruin/1279545", "Full-length", "Melodic Power Metal"),
						Release::new("Apocalypse", " Pandæmonium").with_metallum("https://www.metal-archives.com/bands/Apocalypse/3540449107", "https://www.metal-archives.com/albums/Apocalypse/Pand%C3%A6monium/1279607", "Full-length", "Black/Viking Metal, Death/Thrash Metal"),
						Release::new("Oriska", "Oriska").with_metallum("https://www.metal-archives.com/bands/Oriska/3540534992", "https://www.metal-archives.com/albums/Oriska/Oriska/1279811", "Full-length", "Post-Black/Doom Metal"),
						Release::new("Violent Definition", "Progressive Obsoletion").with_metallum("https://www.metal-archives.com/bands/Violent_Definition/3540303408", "https://www.metal-archives.com/albums/Violent_Definition/Progressive_Obsoletion/1280450", "Full-length", "Thrash Metal"),
						Release::new("Sergeant Thunderhoof", "The Ghost of Badon Hill").with_metallum("https://www.metal-archives.com/bands/Sergeant_Thunderhoof/3540379484", "https://www.metal-archives.com/albums/Sergeant_Thunderhoof/The_Ghost_of_Badon_Hill/1280491", "Full-length", "Psychedelic Stoner/Doom Metal"),
					]),
					(19, vec![
						Release::new("Miseri Silentium", "Live at Darkness Conspiracy 2024").with_metallum("https://www.metal-archives.com/bands/Miseri_Silentium/3540501270", "https://www.metal-archives.com/albums/Miseri_Silentium/Live_at_Darkness_Conspiracy_2024/1278607", "Live album", "Black Metal"),
						Release::new("Stenched", "Purulence Gushing from the Coffin").with_metallum("https://www.metal-archives.com/bands/Stenched/3540526785", "https://www.metal-archives.com/albums/Stenched/Purulence_Gushing_from_the_Coffin/1279043", "Full-length", "Death Metal"),
						Release::new("Chain Wolf / Nuke / Evil Army / Whipstriker", "Metal Punk, Vol. I").with_metallum("https://www.metal-archives.com/bands/Chain_Wolf/3540479708", "https://www.metal-archives.com/albums/Chain_Wolf_-_Nuke_-_Evil_Army_-_Whipstriker/Metal_Punk%2C_Vol._I/1279083", "Split", "Thrash Metal/Crossover | Speed Metal | Thrash Metal | Heavy/Speed Metal"),
					]),
					(20, vec![
						Release::new("Sunrot / Body Void", "SUNROT // BODY VOID").with_metallum("https://www.metal-archives.com/bands/Sunrot/3540392162", "https://www.metal-archives.com/albums/Sunrot_-_Body_Void/SUNROT_--_BODY_VOID/1279072", "Split", "Sludge/Drone/Doom Metal | Sludge/Drone/Doom Metal")						,
						Release::new("Machete Tactics", "Infinite Terror").with_metallum("https://www.metal-archives.com/bands/Machete_Tactics/3540511890", "https://www.metal-archives.com/albums/Machete_Tactics/Infinite_Terror/1280003", "Full-length", "Death/Thrash Metal"),
					]),
					(21, vec![
						Release::new("Accu§er", "Rebirthless").with_metallum("https://www.metal-archives.com/bands/Accu%C2%A7er/1323", "https://www.metal-archives.com/albums/Accu%C2%A7er/Rebirthless/1279759", "Full-length", "Thrash Metal (early); Groove/Thrash Metal (later)"),
					]),
					(22, vec![
						Release::new("Exuvial", "The Hive Mind Chronicles Part I - Parasitica").with_metallum("https://www.metal-archives.com/bands/Exuvial/3540547260", "https://www.metal-archives.com/albums/Exuvial/The_Hive_Mind_Chronicles_Part_I_-_Parasitica/1259846", "Full-length", "Progressive Death Metal"),
						Release::new("Opeth", "The Last Will and Testament").with_metallum("https://www.metal-archives.com/bands/Opeth/38", "https://www.metal-archives.com/albums/Opeth/The_Last_Will_and_Testament/1260410", "Full-length", "Progressive Death Metal, Progressive Rock"),
						Release::new("Fellowship", "The Skies Above Eternity").with_metallum("https://www.metal-archives.com/bands/Fellowship/3540465789", "https://www.metal-archives.com/albums/Fellowship/The_Skies_Above_Eternity/1260533", "Full-length", "Symphonic Power Metal"),
						Release::new("Maat", "From Origin to Decay").with_metallum("https://www.metal-archives.com/bands/Maat/3540318059", "https://www.metal-archives.com/albums/Maat/From_Origin_to_Decay/1262661", "Full-length", "Death Metal")						,
						Release::new("Defeated Sanity", "Chronicles of Lunacy").with_metallum("https://www.metal-archives.com/bands/Defeated_Sanity/11052", "https://www.metal-archives.com/albums/Defeated_Sanity/Chronicles_of_Lunacy/1263562", "Full-length", "Technical Brutal Death Metal")						,
						Release::new("Silent Winter", "Utopia").with_metallum("https://www.metal-archives.com/bands/Silent_Winter/84549", "https://www.metal-archives.com/albums/Silent_Winter/Utopia/1267863", "Full-length", "Power/Progressive Metal")						,
						Release::new("10,000 Years", "All Quiet on the Final Frontier").with_metallum("https://www.metal-archives.com/bands/10%2C000_Years/3540470669", "https://www.metal-archives.com/albums/10%2C000_Years/All_Quiet_on_the_Final_Frontier/1267969", "Full-length", "Stoner Metal"),
						Release::new("Sign of the Jackal", "Heavy Metal Survivors").with_metallum("https://www.metal-archives.com/bands/Sign_of_the_Jackal/3540276003", "https://www.metal-archives.com/albums/Sign_of_the_Jackal/Heavy_Metal_Survivors/1269924", "Full-length", "Heavy Metal"),
						Release::new("Ante-Inferno", "Death's Soliloquy").with_metallum("https://www.metal-archives.com/bands/Ante-Inferno/3540437083", "https://www.metal-archives.com/albums/Ante-Inferno/Death%27s_Soliloquy/1269933", "Full-length", "Black Metal"),
						Release::new("Dawn of Destiny", "IX").with_metallum("https://www.metal-archives.com/bands/Dawn_of_Destiny/63866", "https://www.metal-archives.com/albums/Dawn_of_Destiny/IX/1270195", "Full-length", "Power Metal"),
						Release::new("High Warden", "Astral Iron").with_metallum("https://www.metal-archives.com/bands/High_Warden/3540516616", "https://www.metal-archives.com/albums/High_Warden/Astral_Iron/1270525", "Full-length", "Doom Metal"),
						Release::new("Iniquitous Savagery", "Edifice of Vicissitudes").with_metallum("https://www.metal-archives.com/bands/Iniquitous_Savagery/3540347826", "https://www.metal-archives.com/albums/Iniquitous_Savagery/Edifice_of_Vicissitudes/1270756", "Full-length", "Brutal Death Metal"),
						Release::new("Orso", "Caffè?").with_metallum("https://www.metal-archives.com/bands/Orso/3540453553", "https://www.metal-archives.com/albums/Orso/Caff%C3%A8%3F/1272317", "Full-length", "Atmospheric Sludge/Post-Metal")						,
						Release::new("Aeon Gods", "King of Gods").with_metallum("https://www.metal-archives.com/bands/Aeon_Gods/3540539147", "https://www.metal-archives.com/albums/Aeon_Gods/King_of_Gods/1272325", "Full-length", "Symphonic Power Metal"),
						Release::new("Tyrannic", "Tyrannic Desolation").with_metallum("https://www.metal-archives.com/bands/Tyrannic/3540359737", "https://www.metal-archives.com/albums/Tyrannic/Tyrannic_Desolation/1273132", "Full-length", "Black/Doom Metal"),
						Release::new("Panzerfaust", "The Suns of Perdition - Chapter IV: To Shadow Zion").with_metallum("https://www.metal-archives.com/bands/Panzerfaust/84605", "https://www.metal-archives.com/albums/Panzerfaust/The_Suns_of_Perdition_-_Chapter_IV%3A_To_Shadow_Zion/1274867", "Full-length", "Black Metal"),
						Release::new("Artery", "Last Chance").with_metallum("https://www.metal-archives.com/bands/Artery/99430", "https://www.metal-archives.com/albums/Artery/Last_Chance/1275281", "Full-length", "Death Metal/Metalcore")						,
						Release::new("Slať", "Elegie propastná").with_metallum("https://www.metal-archives.com/bands/Sla%C5%A5/3540494185", "https://www.metal-archives.com/albums/Sla%C5%A5/Elegie_propastn%C3%A1/1275735", "Full-length", "Sludge/Stoner Metal")						,
						Release::new("Repuked", "Club Squirting Blood").with_metallum("https://www.metal-archives.com/bands/Repuked/3540260296", "https://www.metal-archives.com/albums/Repuked/Club_Squirting_Blood/1276166", "Full-length", "Death Metal")						,
						Release::new("Carnal Savagery", "Graveworms, Cadavers, Coffins and Bones").with_metallum("https://www.metal-archives.com/bands/Carnal_Savagery/3540465378", "https://www.metal-archives.com/albums/Carnal_Savagery/Graveworms%2C_Cadavers%2C_Coffins_and_Bones/1276300", "Full-length", "Death Metal"),
						Release::new("Xandria", "Universal Tales").with_metallum("https://www.metal-archives.com/bands/Xandria/3718", "https://www.metal-archives.com/albums/Xandria/Universal_Tales/1278151", "EP", "Symphonic Metal"),
						Release::new("DeadlySins", "Age of Revelation").with_metallum("https://www.metal-archives.com/bands/DeadlySins/3540264962", "https://www.metal-archives.com/albums/DeadlySins/Age_of_Revelation/1278463", "Full-length", "Thrash Metal"),
						Release::new("Gutless", "High Impact Violence").with_metallum("https://www.metal-archives.com/bands/Gutless/3540448353", "https://www.metal-archives.com/albums/Gutless/High_Impact_Violence/1278469", "Full-length", "Death Metal"),
						Release::new("KingCrown", "Nova Atlantis").with_metallum("https://www.metal-archives.com/bands/KingCrown/3540459814", "https://www.metal-archives.com/albums/KingCrown/Nova_Atlantis/1279273", "Full-length", "Heavy/Power Metal"),
						Release::new("Golgothan Remains", "Bearer of Light, Matriarch of Death").with_metallum("https://www.metal-archives.com/bands/Golgothan_Remains/3540411725", "https://www.metal-archives.com/albums/Golgothan_Remains/Bearer_of_Light%2C_Matriarch_of_Death/1279728", "EP", "Death Metal"),
						Release::new("Kruelty", "Profane Usurpation").with_metallum("https://www.metal-archives.com/bands/Kruelty/3540459922", "https://www.metal-archives.com/albums/Kruelty/Profane_Usurpation/1280137", "EP", "Death/Doom Metal/Hardcore")						,
					]),
					(24, vec![
						Release::new("Echoes of Gehenna", "The Dreaming Void").with_metallum("https://www.metal-archives.com/bands/Echoes_of_Gehenna/3540537411", "https://www.metal-archives.com/albums/Echoes_of_Gehenna/The_Dreaming_Void/1279983", "Full-length", "Ambient/Atmospheric Black/Doom Metal"),
					]),
					(25, vec![
						Release::new("Grapeshot", "Oblivion").with_metallum("https://www.metal-archives.com/bands/Grapeshot/3540391802", "https://www.metal-archives.com/albums/Grapeshot/Oblivion/1279219", "EP", "Thrash/Groove Metal"),
					]),
					(26, vec![
						Release::new("Soulskinner", "Gloryfied by the Light").with_metallum("https://www.metal-archives.com/bands/Soulskinner/60709", "https://www.metal-archives.com/albums/Soulskinner/Gloryfied_by_the_Light/1278853", "Full-length", "Death Metal"),
					]),
					(27, vec![
						Release::new("Old Wainds", "Stormheart").with_metallum("https://www.metal-archives.com/bands/Old_Wainds/9384", "https://www.metal-archives.com/albums/Old_Wainds/Stormheart/1266948", "Full-length", "Black Metal"),
						Release::new("Empty Throne", "Unholy").with_metallum("https://www.metal-archives.com/bands/Empty_Throne/3540487993", "https://www.metal-archives.com/albums/Empty_Throne/Unholy/1273471", "Full-length", "Melodic/Blackened Death Metal"),
						Release::new("Endemic", "Fetid Plagues").with_metallum("https://www.metal-archives.com/bands/Endemic/3540534289", "https://www.metal-archives.com/albums/Endemic/Fetid_Plagues/1275021", "EP", "Brutal Death Metal/Grindcore"),
						Release::new("Fessus / Kill the Lord", "Decrowned II: Trinity Ablaze / Pilgrims of Morbidity").with_metallum("https://www.metal-archives.com/bands/Fessus/3540530647", "https://www.metal-archives.com/albums/Fessus_-_Kill_the_Lord/Decrowned_II%3A_Trinity_Ablaze_-_Pilgrims_of_Morbidity/1276530", "Split", "Death Metal | Death Metal"),
					]),
					(28, vec![
						Release::new("Völva", "Desires Profane").with_metallum("https://www.metal-archives.com/bands/V%C3%B6lva/3540460662", "https://www.metal-archives.com/albums/V%C3%B6lva/Desires_Profane/1279742", "Full-length", "Black Metal/Crust"),
					]),
					(29, vec![
						Release::new("Brimstone", "Brimstone").with_metallum("https://www.metal-archives.com/bands/Brimstone/3540397193", "https://www.metal-archives.com/albums/Brimstone/Brimstone/1243932", "Full-length", "Heavy/Southern Metal")						,
						Release::new("Frostmoon Eclipse", "Funerals of Old").with_metallum("https://www.metal-archives.com/bands/Frostmoon_Eclipse/3243", "https://www.metal-archives.com/albums/Frostmoon_Eclipse/Funerals_of_Old/1261617", "Boxed set", "Black Metal")						,
						Release::new("Hidden Mothers", "Erosion / Avulsion").with_metallum("https://www.metal-archives.com/bands/Hidden_Mothers/3540474312", "https://www.metal-archives.com/albums/Hidden_Mothers/Erosion_-_Avulsion/1261674", "Full-length", "Post-Black Metal")						,
						Release::new("Noitasapatti", "Sankarin matka").with_metallum("https://www.metal-archives.com/bands/Noitasapatti/3540462670", "https://www.metal-archives.com/albums/Noitasapatti/Sankarin_matka/1269453", "Full-length", "Black Metal")						,
						Release::new("Ritual Fog", "But Merely Flesh").with_metallum("https://www.metal-archives.com/bands/Ritual_Fog/3540502793", "https://www.metal-archives.com/albums/Ritual_Fog/But_Merely_Flesh/1270856", "Full-length", "Death Metal")						,
						Release::new("Vargålder", "Framåt skrider dödens tider").with_metallum("https://www.metal-archives.com/bands/Varg%C3%A5lder/3540542653", "https://www.metal-archives.com/albums/Varg%C3%A5lder/Fram%C3%A5t_skrider_d%C3%B6dens_tider/1270858", "Full-length", "Black Metal"),
						Release::new("Konkhra", "Sad Plight of Lucifer").with_metallum("https://www.metal-archives.com/bands/Konkhra/2252", "https://www.metal-archives.com/albums/Konkhra/Sad_Plight_of_Lucifer/1272640", "Full-length", "Death Metal")						,
						Release::new("Mezmerizer", "Whispers of Leviathan").with_metallum("https://www.metal-archives.com/bands/Mezmerizer/3540333193", "https://www.metal-archives.com/albums/Mezmerizer/Whispers_of_Leviathan/1273473", "Full-length", "Melodic Groove Metal")						,
						Release::new("The Gates of Slumber", "The Gates of Slumber").with_metallum("https://www.metal-archives.com/bands/The_Gates_of_Slumber/24781", "https://www.metal-archives.com/albums/The_Gates_of_Slumber/The_Gates_of_Slumber/1274894", "Full-length", "Doom Metal")						,
						Release::new("Fire Action", "Until the Heat Dies").with_metallum("https://www.metal-archives.com/bands/Fire_Action/3540450339", "https://www.metal-archives.com/albums/Fire_Action/Until_the_Heat_Dies/1275435", "Full-length", "Heavy Metal")						,
						Release::new("Nolove", "Nostalgia").with_metallum("https://www.metal-archives.com/bands/Nolove/3540531420", "https://www.metal-archives.com/albums/Nolove/Nostalgia/1275850", "Single", "Experimental/Depressive Black Metal, Post-Rock")						,
						Release::new("Inverted Cross", "Eternal Flames of Hell").with_metallum("https://www.metal-archives.com/bands/Inverted_Cross/3540439945", "https://www.metal-archives.com/albums/Inverted_Cross/Eternal_Flames_of_Hell/1276299", "Full-length", "Black/Speed Metal")						,
						Release::new("Festergore", "Constellation of Endless Blight").with_metallum("https://www.metal-archives.com/bands/Festergore/3540515287", "https://www.metal-archives.com/albums/Festergore/Constellation_of_Endless_Blight/1276618", "Full-length", "Death Metal")						,
						Release::new("Mefitis", "The Skorian // The Greyleer").with_metallum("https://www.metal-archives.com/bands/Mefitis/3540270942", "https://www.metal-archives.com/albums/Mefitis/The_Skorian_--_The_Greyleer/1276741", "Full-length", "Death/Black Metal")						,
						Release::new("Cryptorium", "Descent into Lunacy").with_metallum("https://www.metal-archives.com/bands/Cryptorium/3540527310", "https://www.metal-archives.com/albums/Cryptorium/Descent_into_Lunacy/1276916", "Full-length", "Death Metal"),
						Release::new("Havoc", "The Demos").with_metallum("https://www.metal-archives.com/bands/Havoc/1865", "https://www.metal-archives.com/albums/Havoc/The_Demos/1276987", "Compilation", "Heavy/Power Metal"),
						Release::new("Steel Inferno", "Rush of Power").with_metallum("https://www.metal-archives.com/bands/Steel_Inferno/3540375858", "https://www.metal-archives.com/albums/Steel_Inferno/Rush_of_Power/1277494", "Full-length", "Heavy Metal")						,
						Release::new("Dark Embrace", "Land of Witches").with_metallum("https://www.metal-archives.com/bands/Dark_Embrace/16738", "https://www.metal-archives.com/albums/Dark_Embrace/Land_of_Witches/1277722", "Full-length", "Gothic/Doom Metal (early); Symphonic/Melodic Death Metal (later)")						,
						Release::new("Feral Forms", "Through Demonic Spell").with_metallum("https://www.metal-archives.com/bands/Feral_Forms/3540534641", "https://www.metal-archives.com/albums/Feral_Forms/Through_Demonic_Spell/1277872", "Full-length", "Black/Death Metal")						,
						Release::new("Pestilent Hex", "Sorceries of Sanguine & Shadow").with_metallum("https://www.metal-archives.com/bands/Pestilent_Hex/3540505533", "https://www.metal-archives.com/albums/Pestilent_Hex/Sorceries_of_Sanguine_%26_Shadow/1277924", "Full-length", "Melodic/Symphonic Black Metal")						,
						Release::new("Vidres a la Sang", "Virtut del desencís").with_metallum("https://www.metal-archives.com/bands/Vidres_a_la_Sang/23342", "https://www.metal-archives.com/albums/Vidres_a_la_Sang/Virtut_del_desenc%C3%ADs/1278256", "Full-length", "Black/Death Metal")						,
						Release::new("Starmonger", "Occultation").with_metallum("https://www.metal-archives.com/bands/Starmonger/3540479252", "https://www.metal-archives.com/albums/Starmonger/Occultation/1278353", "Full-length", "Psychedelic Stoner Metal/Rock")						,
						Release::new("Nebelkrähe", "Entfremdet (2024)").with_metallum("https://www.metal-archives.com/bands/Nebelkr%C3%A4he/93741", "https://www.metal-archives.com/albums/Nebelkr%C3%A4he/Entfremdet_%282024%29/1279064", "Full-length", "Black Metal"),
						Release::new("Eard", "Melancholia").with_metallum("https://www.metal-archives.com/bands/Eard/3540493153", "https://www.metal-archives.com/albums/Eard/Melancholia/1279735", "Full-length", "Atmospheric Black Metal")						,
						Release::new("Scythrow", "Blameless Severed Extremities").with_metallum("https://www.metal-archives.com/bands/Scythrow/3540490581", "https://www.metal-archives.com/albums/Scythrow/Blameless_Severed_Extremities/1279765", "Full-length", "Thrash/Heavy Metal (early); Death Metal (later)"),
						Release::new("Filii Nigrantium Infernalium", "Pérfida Contracção do Aço").with_metallum("https://www.metal-archives.com/bands/Filii_Nigrantium_Infernalium/11012", "https://www.metal-archives.com/albums/Filii_Nigrantium_Infernalium/P%C3%A9rfida_Contrac%C3%A7%C3%A3o_do_A%C3%A7o/1279781", "Full-length", "Black Metal (early); Black/Heavy/Thrash Metal (later)"),
						Release::new("Necronomicon Ex Mortis", "The Mother of Death").with_metallum("https://www.metal-archives.com/bands/Necronomicon_Ex_Mortis/3540521389", "https://www.metal-archives.com/albums/Necronomicon_Ex_Mortis/The_Mother_of_Death/1279863", "EP", "Death Metal"),
						Release::new("Thy Legion", "Grand Cosmic Funeral").with_metallum("https://www.metal-archives.com/bands/Thy_Legion/111995", "https://www.metal-archives.com/albums/Thy_Legion/Grand_Cosmic_Funeral/1280354", "Full-length", "Black/Death Metal"),
						Release::new("Droid Killer", "Delete Everything").with_metallum("https://www.metal-archives.com/bands/Droid_Killer/3540500325", "https://www.metal-archives.com/albums/Droid_Killer/Delete_Everything/1280355", "Single", "Doom/Death Metal"),
						Release::new("Gorgon", "For Those Who Stay").with_metallum("https://www.metal-archives.com/bands/Gorgon/11808", "https://www.metal-archives.com/albums/Gorgon/For_Those_Who_Stay/1280449", "Full-length", "Black Metal")						,
					]),
					(30, vec![
						Release::new("Funereality", "Through the Black Holes of the Dead").with_metallum("https://www.metal-archives.com/bands/Funereality/3540308459", "https://www.metal-archives.com/albums/Funereality/Through_the_Black_Holes_of_the_Dead/1261104", "Full-length", "Death Metal"),
						Release::new("Heathen Deity", "Satan's Kingdom").with_metallum("https://www.metal-archives.com/bands/Heathen_Deity/8606", "https://www.metal-archives.com/albums/Heathen_Deity/Satan%27s_Kingdom/1263526", "Full-length", "Black Metal"),
						Release::new("Deheubarth", "Revel in Occult Chambers").with_metallum("https://www.metal-archives.com/bands/Deheubarth/3540394048", "https://www.metal-archives.com/albums/Deheubarth/Revel_in_Occult_Chambers/1274491", "Full-length", "Black/Doom Metal"),
						Release::new("Duister Maanlicht", "Werken van de duisternis").with_metallum("https://www.metal-archives.com/bands/Duister_Maanlicht/3540398447", "https://www.metal-archives.com/albums/Duister_Maanlicht/Werken_van_de_duisternis/1277628", "Full-length", "Raw Black Metal"),
					]),
				])),
				(Month::December, Releases::from([
					(1, vec![
						Release::new("Nordic Twilight", "Nordic Twilight").with_metallum("https://www.metal-archives.com/bands/Nordic_Twilight/3540522394", "https://www.metal-archives.com/albums/Nordic_Twilight/Nordic_Twilight/1275890", "Full-length", "Symphonic Power Metal"),
						Release::new("Caixão", "Demos 2017 - 2024").with_metallum("https://www.metal-archives.com/bands/Caix%C3%A3o/3540484933", "https://www.metal-archives.com/albums/Caix%C3%A3o/Demos_2017_-_2024/1279111", "Compilation", "Black/Death Metal"),
					]),
					(5, vec![
						Release::new("Horrorborn", "Illuminating Doom").with_metallum("https://www.metal-archives.com/bands/Horrorborn/3540493577", "https://www.metal-archives.com/albums/Horrorborn/Illuminating_Doom/1255329", "Full-length", "Symphonic Black Metal/Deathcore")						,
					]),
					(6, vec![
						Release::new("Fermentor", "Release Me").with_metallum("https://www.metal-archives.com/bands/Fermentor/3540404731", "https://www.metal-archives.com/albums/Fermentor/Release_Me/1273180", "Full-length", "Death Metal"),
						Release::new("Asterise", "Tale of a Wandering Soul").with_metallum("https://www.metal-archives.com/bands/Asterise/3540516247", "https://www.metal-archives.com/albums/Asterise/Tale_of_a_Wandering_Soul/1274138", "Full-length", "Power Metal"),
						Release::new("Nanowar of Steel", "XX Years of Steel").with_metallum("https://www.metal-archives.com/bands/Nanowar_of_Steel/3540261755", "https://www.metal-archives.com/albums/Nanowar_of_Steel/XX_Years_of_Steel/1274421", "Compilation", "Heavy/Power Metal/Hard Rock"),
						Release::new("The Old Dead Tree", "Second Thoughts").with_metallum("https://www.metal-archives.com/bands/The_Old_Dead_Tree/8297", "https://www.metal-archives.com/albums/The_Old_Dead_Tree/Second_Thoughts/1274538", "Full-length", "Gothic Metal"),
						Release::new("Pillar of Light", "Caldera").with_metallum("https://www.metal-archives.com/bands/Pillar_of_Light/3540548599", "https://www.metal-archives.com/albums/Pillar_of_Light/Caldera/1274923", "Full-length", "Doom/Sludge/Post-Metal"),
						Release::new("Nolove", "Nobody Can Save You").with_metallum("https://www.metal-archives.com/bands/Nolove/3540531420", "https://www.metal-archives.com/albums/Nolove/Nobody_Can_Save_You/1276304", "Full-length", "Experimental/Depressive Black Metal, Post-Rock"),
						Release::new("Tethra", "Withered Heart Standing").with_metallum("https://www.metal-archives.com/bands/Tethra/3540317188", "https://www.metal-archives.com/albums/Tethra/Withered_Heart_Standing/1276477", "Full-length", "Death/Doom Metal"),
						Release::new("Night in Gales", "Shadowreaper").with_metallum("https://www.metal-archives.com/bands/Night_in_Gales/817", "https://www.metal-archives.com/albums/Night_in_Gales/Shadowreaper/1276707", "Full-length", "Melodic Death Metal"),
						Release::new("Within Silence", "The Eclipse of Worlds").with_metallum("https://www.metal-archives.com/bands/Within_Silence/3540396066", "https://www.metal-archives.com/albums/Within_Silence/The_Eclipse_of_Worlds/1276946", "Full-length", "Power Metal"),
						Release::new("Tarja", "Rocking Heels: Live at Hellfest").with_metallum("https://www.metal-archives.com/bands/Tarja/110710", "https://www.metal-archives.com/albums/Tarja/Rocking_Heels%3A_Live_at_Hellfest/1277113", "Live album", "Symphonic Metal/Rock, Neoclassical"),
						Release::new("Aara", "Eiger").with_metallum("https://www.metal-archives.com/bands/Aara/3540451086", "https://www.metal-archives.com/albums/Aara/Eiger/1277867", "Full-length", "Atmospheric Black Metal"),
						Release::new("Panzerchrist", "Maleficium - Part 1").with_metallum("https://www.metal-archives.com/bands/Panzerchrist/2864", "https://www.metal-archives.com/albums/Panzerchrist/Maleficium_-_Part_1/1278810", "Full-length", "Death/Black Metal"),
						Release::new("Tales of Blood", "Breath of Repugnance").with_metallum("https://www.metal-archives.com/bands/Tales_of_Blood/10656", "https://www.metal-archives.com/albums/Tales_of_Blood/Breath_of_Repugnance/1278857", "Full-length", "Death Metal"),
						Release::new("Desert Near the End", "Tides of Time").with_metallum("https://www.metal-archives.com/bands/Desert_Near_the_End/3540370893", "https://www.metal-archives.com/albums/Desert_Near_the_End/Tides_of_Time/1280001", "Full-length", "Power/Thrash Metal"),
					]),
					(7, vec![
						Release::new("STP", "Maoist Jihad: Death to Collaborators").with_metallum("https://www.metal-archives.com/bands/STP/3540534096", "https://www.metal-archives.com/albums/STP/Maoist_Jihad%3A_Death_to_Collaborators/1260731", "Demo", "Raw Black Metal"),
					]),
					(12, vec![
						Release::new("New Mexican Doom Cult", "From the Crypt").with_metallum(
							"https://www.metal-archives.com/bands/New_Mexican_Doom_Cult/3540451389",
							"https://www.metal-archives.com/albums/New_Mexican_Doom_Cult/From_the_Crypt/1215886",
							"Compilation",
							"Stoner/Doom Metal"
						),
						Release::new("Kombat / False Mutation", "Monument of Abomination").with_metallum(
							"https://www.metal-archives.com/bands/Kombat/3540454097",
							"https://www.metal-archives.com/albums/Kombat_-_False_Mutation/Monument_of_Abomination/1280105",
							"Split",
							"Death Metal/Crossover | Death Metal"
						  ),
					]),
					(13, vec![
						Release::new("Mörk Gryning", "Fasornas tid").with_metallum(
							"https://www.metal-archives.com/bands/M%C3%B6rk_Gryning/1430",
							"https://www.metal-archives.com/albums/M%C3%B6rk_Gryning/Fasornas_tid/1272673",
							"Full-length",
							"Black Metal"
						),
						Release::new("Injector", "Endless Scorn").with_metallum(
							"https://www.metal-archives.com/bands/Injector/3540368269",
							"https://www.metal-archives.com/albums/Injector/Endless_Scorn/1274964",
							"Full-length",
							"Thrash Metal"
						),
						Release::new("A Dead Poem", "Abstract Existence").with_metallum(
							"https://www.metal-archives.com/bands/A_Dead_Poem/3540533977",
							"https://www.metal-archives.com/albums/A_Dead_Poem/Abstract_Existence/1278158",
							"Full-length",
							"Doom/Black/Gothic Metal"
						),
						Release::new("Magic Kingdom", "Blaze of Rage").with_metallum(
							"https://www.metal-archives.com/bands/Magic_Kingdom/14327",
							"https://www.metal-archives.com/albums/Magic_Kingdom/Blaze_of_Rage/1279280",
							"Full-length",
							"Power Metal"
						),
						Release::new("Misanthropy", "The Ever​-​Crushing Weight of Stagnance").with_metallum(
							"https://www.metal-archives.com/bands/Misanthropy/3540372393",
							"https://www.metal-archives.com/albums/Misanthropy/The_Ever%E2%80%8B-%E2%80%8BCrushing_Weight_of_Stagnance/1279340",
							"Full-length",
							"Thrash Metal (early); Progressive Death Metal (later)"
						  ),
					]),
					(15, vec![
						Release::new("Grollheim", "Funebres Nuptiae").with_metallum("https://www.metal-archives.com/bands/Grollheim/3540533123", "https://www.metal-archives.com/albums/Grollheim/Funebres_Nuptiae/1256160", "Full-length", "Raw Black Metal, Dungeon Synth"),
					]),
					(20, vec![
						Release::new("Vinodium", "¿En que mundo vivimos?").with_metallum("https://www.metal-archives.com/bands/Vinodium/3540460500", "https://www.metal-archives.com/albums/Vinodium/%C2%BFEn_que_mundo_vivimos%3F/1275758", "Full-length", "Heavy/Thrash Metal")						,
						Release::new("Lights to Remain", "Damnation").with_metallum("https://www.metal-archives.com/bands/Lights_to_Remain/3540527030", "https://www.metal-archives.com/albums/Lights_to_Remain/Damnation/1278979", "Full-length", "Melodic Death Metal"),
						Release::new("Hexenbrett", "Dritte Beschw​ö​rung: Dem Teufel eine Tochter").with_metallum("https://www.metal-archives.com/bands/Hexenbrett/3540449256", "https://www.metal-archives.com/albums/Hexenbrett/Dritte_Beschw%E2%80%8B%C3%B6%E2%80%8Brung%3A_Dem_Teufel_eine_Tochter/1280236", "Full-length", "Black/Heavy Metal")						,
					]),
					(27, vec![
						Release::new("Bolvag", "Sad Dark Descent into the Dungeon Dream").with_metallum("https://www.metal-archives.com/bands/Bolvag/3540518676", "https://www.metal-archives.com/albums/Bolvag/Sad_Dark_Descent_into_the_Dungeon_Dream/1212539", "Demo", "Raw Black Metal/Ambient"),
						Release::new("Dominum", "The Dead Don't Die").with_metallum("https://www.metal-archives.com/bands/Dominum/3540534931", "https://www.metal-archives.com/albums/Dominum/The_Dead_Don%27t_Die/1280194", "Full-length", "Heavy/Power Metal, Hard Rock"),
					]),
					(28, vec![
						Release::new("Nicolas Waldo", "World on Fire").with_metallum(
							"https://www.metal-archives.com/bands/Nicolas_Waldo/63495",
							"https://www.metal-archives.com/albums/Nicolas_Waldo/World_on_Fire/1266767",
							"Full-length",
							"Shred/Heavy Metal"
						  ),
					]),
				])),
            ]),
        };
        compare_calendars(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_2025_calendar_ok() -> Result<()> {
        let client = MockClient::new();

        let got = scrape(&client, 2025).await?;

        let want = Calendar {
            year: 2025,
            data: CalendarData::from([
                (
                    Month::January,
                    Releases::from([
                        (
                            1,
                            vec![
                                Release::new("Death Cult 69", "The Way of All Flesh").with_metallum("https://www.metal-archives.com/bands/Death_Cult_69/3540500536", "https://www.metal-archives.com/albums/Death_Cult_69/The_Way_of_All_Flesh/1279066", "Full-length", "Doom Metal"),
                                Release::new("Estuarine", "Corporeal Furnace").with_metallum("https://www.metal-archives.com/bands/Estuarine/3540371923", "https://www.metal-archives.com/albums/Estuarine/Corporeal_Furnace/1279216", "Full-length", "Experimental/Technical Death Metal/Grindcore"),
                                Release::new("Hazzerd", "The 3rd Dimension").with_metallum("https://www.metal-archives.com/bands/Hazzerd/3540393393", "https://www.metal-archives.com/albums/Hazzerd/The_3rd_Dimension/1280443", "Full-length", "Thrash Metal"),
                            ],
                        ),
                        (
                            3,
                            vec![
                                Release::new("Aeonian Sorrow", "From the Shadows").with_metallum("https://www.metal-archives.com/bands/Aeonian_Sorrow/3540438810", "https://www.metal-archives.com/albums/Aeonian_Sorrow/From_the_Shadows/1279038", "EP", "Gothic/Doom/Death Metal"),
                                Release::new("Faidra", "Dies Irae").with_metallum("https://www.metal-archives.com/bands/Faidra/3540461589", "https://www.metal-archives.com/albums/Faidra/Dies_Irae/1279060", "EP", "Atmospheric Black Metal"),
                            ],
                        ),
                        (
                            10,
                            vec![Release::new("The Halo Effect", "March of the Unheard").with_metallum("https://www.metal-archives.com/bands/The_Halo_Effect/3540497081", "https://www.metal-archives.com/albums/The_Halo_Effect/March_of_the_Unheard/1278174", "Full-length", "Melodic Death Metal")],
                        ),
                        (
                            17,
                            vec![
                                Release::new("Grave Digger", "Bone Collector").with_metallum("https://www.metal-archives.com/bands/Grave_Digger/391", "https://www.metal-archives.com/albums/Grave_Digger/Bone_Collector/1278105", "Full-length", "Speed Metal (early); Heavy/Power Metal (later)"),
                                Release::new("Tokyo Blade", "Time Is the Fire").with_metallum("https://www.metal-archives.com/bands/Tokyo_Blade/1444", "https://www.metal-archives.com/albums/Tokyo_Blade/Time_Is_the_Fire/1278108", "Full-length", "NWOBHM, Heavy Metal"),
                                Release::new("Pestilent Scars", "Meadows of Misfortune").with_metallum("https://www.metal-archives.com/bands/Pestilent_Scars/3540541501", "https://www.metal-archives.com/albums/Pestilent_Scars/Meadows_of_Misfortune/1278178", "Full-length", "Melodic Death Metal"),
                            ],
                        ),
                        (
                            24,
                            vec![
                                Release::new("Harakiri for the Sky", "Scorched Earth").with_metallum("https://www.metal-archives.com/bands/Harakiri_for_the_Sky/3540354014", "https://www.metal-archives.com/albums/Harakiri_for_the_Sky/Scorched_Earth/1278125", "Full-length", "Post-Black Metal"),
                                Release::new(
                                    "Avatarium",
                                    "Between You, God, the Devil and the Dead",
                                ).with_metallum("https://www.metal-archives.com/bands/Avatarium/3540369803", "https://www.metal-archives.com/albums/Avatarium/Between_You%2C_God%2C_the_Devil_and_the_Dead/1278862", "Full-length", "Doom Metal/Rock"),
                                Release::new("Wardruna", "Birna").with_metallum("https://www.metal-archives.com/bands/Wardruna/3540272417", "https://www.metal-archives.com/albums/Wardruna/Birna/1279979", "Full-length", "Folk/Ambient"),
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
                                Release::new("Atlas Ashes", "New World").with_metallum("https://www.metal-archives.com/bands/Atlas_Ashes/3540539374", "https://www.metal-archives.com/albums/Atlas_Ashes/New_World/1276845", "Full-length", "Melodic Death Metal"),
                                Release::new("Lacuna Coil", "Sleepless Empire").with_metallum("https://www.metal-archives.com/bands/Lacuna_Coil/124", "https://www.metal-archives.com/albums/Lacuna_Coil/Sleepless_Empire/1278153", "Full-length", "Gothic Metal/Rock (early); Alternative Rock/Metal (later)"),
                            ],
                        ),
                        (
                            21,
                            vec![Release::new(
                                "Defiled Serenity",
                                "Within the Slumber of the Mind",
                            ).with_metallum("https://www.metal-archives.com/bands/Defiled_Serenity/3540552035", "https://www.metal-archives.com/albums/Defiled_Serenity/Within_the_Slumber_of_the_Mind/1278765", "Full-length", "Melodic Death Metal")],
                        ),
                        (
                            28,
                            vec![
                                Release::new("Dimman", "Consciousness").with_metallum("https://www.metal-archives.com/bands/Dimman/3540396979", "https://www.metal-archives.com/albums/Dimman/Consciousness/1278875", "Full-length", "Melodic Death Metal"),
                                Release::new("Timecode", "La Ruptura Del Equilibrio").with_metallum("https://www.metal-archives.com/bands/Timecode/5146", "https://www.metal-archives.com/albums/Timecode/La_Ruptura_Del_Equilibrio/1280262", "Full-length", "Death Metal"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::March,
                    Releases::from([(28, vec![Release::new("Arch Enemy", "Blood Dynasty").with_metallum("https://www.metal-archives.com/bands/Arch_Enemy/10", "https://www.metal-archives.com/albums/Arch_Enemy/Blood_Dynasty/1278155", "Full-length", "Melodic Death Metal")])]),
                ),
            ]),
        };
        compare_calendars(got, want);
        Ok(())
    }
}
