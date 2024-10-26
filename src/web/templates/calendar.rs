use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::model::{Artist, Release};

pub fn feeds(date: &str, releases: Vec<(Release, Artist)>) -> Markup {   
    html!(
        (DOCTYPE)
        html lang="en" {
           head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title{ "Releases " (date) }
           } 
           body {
            ol {
                @for (release, artist) in releases {
                    (PreEscaped(release.to_html(&artist)))
                }
            }
           }
        }
    )
}
