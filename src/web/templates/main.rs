use axum::{extract::State, http::HeaderMap, Form};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use tracing::warn;
use super::{core::layout, Page};
use crate::{
    config::config,
    web::{templates::core::footer, AppState},
};
use crate::support::email::send_email;

/// Generates the main landing page of the application.
pub async fn index(headers: HeaderMap, State(state): State<AppState>) -> Markup {
    let body = html!(
        section class="col-span-12 py-16 md:py-16" style="background: linear-gradient(90deg, #FF4646 0%, #6A6A6A 100%)" {
            div class="container mx-auto px-6" {
                h2 class="text-xl font-bold mb-2 text-white md:text-4xl" {
                    "Are you tired of missing out on your favorite heavy metal bands' latest album releases due to life's busyness?"
                }
                h3 class="text-lg mb-8 md:text-2xl dark:text-white" {
                    "Never miss a headbang-worthy album again. Stay in tune with our band release notifier!"
                }
                p href="/start" class="flex bg-white font-bold rounded-full py-4 px-8 shadow-lg uppercase tracking-wider max-w-72 dark:bg-black" {
                    "Subscribe via"
                    a href=(format!("{}/calendar/feed.xml", config().BASE_URL)) style="padding-left: 12px" {
                        img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Feed-icon.svg/128px-Feed-icon.svg.png" height="32px" width="32px" alt="rss icon";
                    }
                }
            }
        }
        section class="col-span-12 container mx-auto px-6 p-10" {
            h2 class="text-2xl font-bold text-center mb-8 md:text-4xl" {
                "How it works"
            }
            div class="flex items-center flex-wrap mb-8 md:mb-20" {
                div class="w-full md:w-1/2" {
                    h4 class="text-lg font-bold mb-3 md:text-3xl" {
                        "Subscribe to the feed"
                    }
                    div class="md:hidden" {
                        div class="flex" {
                            div class="mb-8" {
                                (rss_apps(&state.bands, &state.genres))
                            }
                            div {
                                img src="/public/img/day-of-tentacle.png" alt="Monitoring" style="height: 10rem; width: 30rem;";
                            }
                        }
                    }
                    div class="hidden md:block" {
                        (rss_apps(&state.bands, &state.genres))
                    }
                }
                div class="hidden md:block w-full md:w-1/2" {
                    img src="/public/img/day-of-tentacle.png" alt="Monitoring";
                }
            }
            div class="items-center mb-20 flex md:flex-wrap" {
                div class="md:w-1/2" {
                    img src="/public/img/guitarist.jpg" alt="Reporting";
                }
                div class="w-full pl-2 md:w-1/2 md:pl-10" {
                    h4 class="text-xl font-bold mb-3 md:text-3xl" {
                        "Continue hustling"
                    }
                    p class="mb-8" {
                        "Missing metal album releases is now a thing of the past because your RSS app will notify you whenever a band releases an album."
                    }
                }
            }
        }
        section class="col-span-12 bg-gray-100 dark:bg-black" {
            div class="container mx-auto px-6 py-16" {
                h2 class="text-2xl font-bold text-center mb-8 md:text-4xl" {
                    "Important Tips"
                }
                div class="flex flex-wrap" {
                    div class="w-full px-2 mb-4 md:w-1/2 dark:bg-black" {
                        div class="bg-white rounded shadow py-2 dark:bg-red-900" {
                            p class="text-base px-6 mb-5" {
                                "We pull our information straight from Wikipedia's authoritative"
                                (PreEscaped("<a href=\"https://en.wikipedia.org/wiki/2024_in_heavy_metal_music\" target=\"_blank\" class=\"text-red-500 hover:text-red-700\"> metal album release page</a>"))
                                ", so you can be sure you're getting accurate and up-to-date info. Never miss out on a killer album drop again!"
                            }
                        }
                    }
                    div class="w-full px-2 mb-4 md:w-1/2 dark:bg-black" {
                        div class="bg-white rounded shadow py-2 dark:bg-red-900" {
                            p class="text-base px-6 mb-5" {
                                "Just like how metal bands rely on brutal riffs and bone-crushing drum beats to create "
                                "their music, they also rely on the support of their fans to keep the metal scene alive. "
                                "Don't be afraid to throw down some cash and show off your metal pride. Your support could "
                                "be the one that fuels their next epic album or tour. Horns up, metalheads! \\m/"
                            }
                        }
                    }
                }
            }
        }
        section class="col-span-12" style="background: linear-gradient(90deg, #3D3D3D 0%, #D73737 100%)" {
            div class="container mx-auto px-6 text-center py-16" {
                h2 class="text-2xl mb-6 font-bold text-center text-white md:text-4xl" {
                    "Never Miss a Beat"
                }
                h3 class="text-lg mt-4 mb-6 text-white md:text-2xl" {
                    "Keep track of the latest heavy metal album releases with our notifier, and never lose your headbanging rhythm again!"
                }
                div class="flex justify-center" {
                    p href="/start" class="flex bg-white font-bold rounded-full py-4 px-8 shadow-lg uppercase tracking-wider max-w-72 dark:bg-black" {
                        "Subscribe via"
                        a href=(format!("{}/calendar/feed.xml", config().BASE_URL)) style="padding-left: 12px" {
                            img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/Feed-icon.svg/128px-Feed-icon.svg.png" height="32px" width="32px" alt="rss icon";
                        }
                    }
                }
            }
        }
    );

    match headers.get("HX-Request") {
        Some(_) => html!(
            title hx-swap-oob="true" { "Home | Heavy Metal Releases" }
            (body)
            (footer())
        ),
        None => layout("Home", true, Page::Home, body),
    }
}

fn rss_apps(bands: &Vec<String>, genres: &[String; 46]) -> Markup {
    html!(
        p {
            "The only thing you must do is install an RSS app and add the "
            a href=(format!("{}/calendar/feed.xml", config().BASE_URL)) class="text-blue-600 visited:text-purple-600" { (format!("{}/calendar/feed.xml", config().BASE_URL)) }
            " feed. You may also customize your list according to the bands and genres you wish to track."
        }
        div class="my-4" {
            p class="font-bold text-center mb-1" { "Customize your feed" }
            form hx-post="/calendar/feed.xml" hx-swap="none" {
                select class="select select-bordered w-full min-h-72 md:w-1/2" name="bands" multiple {
                    option disabled selected class="truncate" { "Choose bands to follow (CRTL+Click)" }
                    option { "All" }
                    option { "None" }
                    @for band in bands {
                        option { (band) }
                    }
                }
                select class="select select-bordered w-full min-h-72 md:w-1/2" name="genres" multiple {
                    option disabled selected class="truncate" { "Choose genres to follow (CRTL+Click)" }
                    option { "All" }
                    option { "None" }
                    @for genre in genres {
                        option { (genre) }
                    }
                }
                button type="submit" class="btn btn-wide w-full mt-1" {
                    "Generate Feed"
                }
            }
            input #custom_link readonly type="text" placeholder="Your custom link to copy" class="input input-bordered w-full mt-1";
        }
        p { "Example RSS apps:" }
        p {
            b {"Android:" }
            ul class="list-disc" {
                li {
                    a class="text-blue-600 visited:text-purple-600" href="https://play.google.com/store/apps/details?id=com.nononsenseapps.feeder.play&hl=en_CA" target="_blank" { "Feeder" }
                }
                li {
                    a class="text-blue-600 visited:text-purple-600" href="https://play.google.com/store/apps/details?id=com.innologica.inoreader&hl=en_CA&pli=1" target="_blank" { "Inoreader" }
                }
            }
        }
        br;
        p {
            b {"iOS:" }
            ul class="list-disc" {
                li {
                    a class="text-blue-600 visited:text-purple-600" href="http://www.rssowl.org/" target="_blank" { "RSSOwl" }
                }
            }
        }
        br;
        p {
            b {"Linux:" }
            ul class="list-disc" {
                li {
                    a class="text-blue-600 visited:text-purple-600" href="https://gfeeds.gabmus.org/" target="_blank" { "GNOME Feeds" }
                }
                li {
                    a class="text-blue-600 visited:text-purple-600" href="http://www.rssowl.org/" target="_blank" { "RSSOwl" }
                }
            }
        }
        br;
        p {
            b {"Windows:" }
            ul class="list-disc" {
                li {
                    a class="text-blue-600 visited:text-purple-600" href="http://feedreader.com/" target="_blank" { "Feedreader" }
                }
                li {
                    a class="text-blue-600 visited:text-purple-600" href="http://www.rssowl.org/" target="_blank" { "RSSOwl" }
                }
            }
        }
    )
}

/// Generates the "About Us" page of the application.
pub async fn about_handler(headers: HeaderMap) -> Markup {
    let body = html!(
        section class="col-span-12" style="background: linear-gradient(90deg, #D73737 0%, #3D3D3D 100%)" {}
        section class="col-span-12 container mx-auto px-6 p-10" {
            div class="flex items-center flex-wrap mb-20" {
                div class="w-full md:w-1/2" {
                    h4 class="text-3xl  font-bold mb-3" {
                        "About us"
                    }
                    p class="mb-8" {
                        "At Heavy Metal Releases Notifier, we know that heavy metal fans are always on the hunt for the latest "
                        "and greatest albums of the day, month and year. That's why we created a service that notifies you "
                        "when new heavy metal band albums are released."
                    }
                    p class="mb-8" {
                        "Our service is fully open source and available on "
                        (PreEscaped("<a href=\"https://github.com/reaper47/heavy-metal-notifier\" target=\"_blank\" class=\"text-blue-500 hover:text-blue-800\">GitHub</a>"))
                        ". We believe in the power of community and collaboration, and we invite you to join us in improving and expanding our "
                        "service to make it even better for heavy metal fans everywhere."
                    }
                }
                div class="w-full md:w-1/2 flex justify-center" {
                    img src="/public/img/bell-pepper.jpg" alt="A rocking, red bell pepper";
                }
            }
        }
    );

    match headers.get("HX-Request") {
        Some(_) => html!(
            title hx-swap-oob="true" { "About | Heavy Metal Releases" }
            (body)
            (footer())
        ),
        None => layout("About", true, Page::About, body),
    }
}

/// Generates the "Contact Us" page of the application.
pub async fn contact_handler(headers: HeaderMap) -> Markup {
    let body = html!(
        section class="col-span-12" style="background: linear-gradient(90deg, #D73737 0%, #3D3D3D 100%)" {}
        section class="col-span-12 container mx-auto px-6 p-10" {
            div class="flex items-center flex-wrap mb-20" {
                div class="w-full md:w-1/2" {
                    h4 class="text-3xl  font-bold mb-3" {
                        "Contact us"
                    }
                    p class="mb-4" {
                        "To address any inquiries, please send a message to us directly from the form below."
                    }
                    form class="w-full md:w-3/4 bg-white p-6 rounded-lg shadow-md mb-8 dark:bg-black"
                         hx-post="/contact" hx-swap="none"
                         _="on htmx:afterRequest reset() me then call alert('Message sent. We will come back to you shortly.')" {
                        div class="mb-4" {
                            label class="block font-bold mb-2" for="email" {
                                "Email"
                            }
                            input
                                class="border border-gray-400 p-2 w-full"
                                type="email"
                                id="email"
                                name="email"
                                placeholder="your@email.com"
                                required;
                        }
                        div class="mb-4" {
                            label class="block font-bold mb-2" for="message" { "Message" }
                            textarea
                                class="border border-gray-400 p-2 w-full h-32"
                                id="message"
                                name="message"
                                placeholder="Hello Metal Releases, I have something to say."
                                required {}
                        }
                        div class="text-right" {
                            button
                                class="w-full bg-indigo-500 text-white py-2 px-4 rounded-full hover:bg-indigo-600"
                                type="submit"
                            {
                                "Submit"
                            }
                        }
                    }
                }
                div class="w-full md:w-1/2 flex justify-center" {
                    img src="/public/img/dicoo.png" alt="Monitoring";
                }
            }
        }
    );

    match headers.get("HX-Request") {
        Some(_) => html!(
            title hx-swap-oob="true" { "Contact Us | Heavy Metal Releases" }
            (body)
            (footer())
        ),
        None => layout("Contact Us", true, Page::Contact, body),
    }
}

/// Represents a form submission for the "Contact Us" page.
#[derive(Debug, Deserialize)]
pub struct ContactUsForm {
    email: String,
    message: String,
}

/// Handles form submission from the "Contact Us" page.
pub async fn contact_post_handler(Form(contact_us): Form<ContactUsForm>) {
    match &config().smtp {
        Some(smtp) => {
            let email = contact_us.email.clone();
            let message = contact_us.message.clone();

            tokio::task::spawn(async move {
                send_email(smtp, email, message)
            });
        },
        None => {
            warn!("Email feature is disabled. Message: {:?}", contact_us);
        },
    }
}

/// Generates the Terms of Service page of the application.
pub async fn tos(headers: HeaderMap) -> Markup {
    let body = html!(
        section class="col-span-12 py-16" style="background: linear-gradient(90deg, #D73737 0%, #3D3D3D 100%)" {}
        section class="col-span-12 container mx-auto px-6 p-10" {
            div class="flex items-center flex-wrap mb-20" {
                div class="w-full md:w-1/2" {
                    h4 class="text-3xl  font-bold mb-3" {
                        "Terms of Service"
                    }
                    p class="mb-8" {
                        "Please read through our Terms of Service carefully before using our service."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Scope of Service"
                    }
                    p class="mb-8" {
                        "The website, Heavy Metal Releases Notifier, offers a service to automatically notify you of new heavy metal "
                        "album releases throughout the current year. The user will be notified via an RSS app on new releases."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Intellectual Property Rights"
                    }
                    p class="mb-8" {
                        "The user is free to use this program for whatever reason because it is licensed under the MIT license. "
                        "The user acknowledges that any content submitted or transferred to the website becomes the property of "
                        "the one who hosts an instance of Heavy Metal Releases Notifier and may be used for any lawful purpose "
                        "without compensation to the user. Heavy Metal Releases Notifier reserves the right to refuse service "
                        "to any user `for` any reason at any time. The images are from "
                        (PreEscaped("<a href=\"https://pixabay.com\">Pixabay</a>"))
                        "and "
                        (PreEscaped("<a href=\"https://commons.wikimedia.org/wiki/Main_Page\">Wikimedia Commons</a>"))
                        "."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "User Conduct"
                    }
                    p class="mb-8" {
                        "The user must respect basic human interactions when using the website or contacting support. "
                        "This includes not using offensive language, making personal attacks, or engaging in any behavior "
                        "that may be considered harassing or threatening. Heavy Metal Releases Notifier reserves the right to "
                        "terminate the user's account if they violate these terms of conduct."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Disclaimer of Warranties"
                    }
                    p class="mb-8" {
                        "The website is provided on an \"as is\" and \"as available\" basis. Heavy Metal Releases Notifier makes no "
                        "representations or warranties of any kind, express or implied, as to the operation of the website "
                        "or the information, content, materials, or products included on the website. The user acknowledges "
                        "that their use of the website is at their own risk. Heavy Metal Releases Notifier does not warrant that "
                        "the website will be uninterrupted or error-free, and Heavy Metal Releases Notifier will not be liable "
                        "for any interruptions or errors."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Limitation of Liability"
                    }
                    p class="mb-8" {
                        "Heavy Metal Releases Notifier is not responsible for any missed headbanging to the newest heavy metal "
                        "releases. The user acknowledges that Heavy Metal Releases Notifier will not be liable for any damages "
                        "of any kind arising from the use of the website and from excessive headbanging."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Disputes"
                    }
                    p class="mb-8" {
                        "Any dispute arising from the use of this website or its services will be governed by the laws of "
                        "the jurisdiction in which Heavy Metal Releases Notifier is located. The user agrees to submit to the "
                        "jurisdiction of the courts in that jurisdiction for resolution of any such dispute."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Changes to the TOS"
                    }
                    p class="mb-8" {
                        "Heavy Metal Releases Notifier reserves the right to modify these terms of service at any time. Any changes "
                        "will be communicated to users through the website. The user's continued use of the website following "
                        "any changes to the TOS constitutes acceptance of those changes."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Governing Law"
                    }
                    p class="mb-8" {
                        "This Terms of Service agreement and the relationship between the user and Heavy Metal Releases Notifier "
                        "shall be governed by and construed in accordance with the laws of heavy metal. The user "
                        "agrees to submit to the jurisdiction of the courts located in Heaven for any and all "
                        "disputes arising from or related to the use of the website and its services."
                    }
                    h5 class="text-3xl  font-bold mb-3" {
                        "Contact Information"
                    }
                    p class="mb-8" {
                        "If you have any questions or concerns regarding these Terms of Service agreement, please contact Heavy Metal Releases Notifier at "
                        (PreEscaped("<a href=\"mailto:metal.releases.666@gmail.com\" target=\"_blank\" class=\"text-blue-500 hover:text-blue-600\" aria-label=\"Email metal.releases.666@gmail.com\">metal.releases.666@gmail.com</a>"))
                        "."
                    }
                }
                div class="w-full md:w-1/2" {
                    img src="/public/img/dicoo.png" alt="Monitoring";
                }
            }
        }
    );

    match headers.get("HX-Request") {
        Some(_) => html!(
            title hx-swap-oob="true" { "Terms of Service | Heavy Metal Releases" }
            (body)
            (footer())
        ),
        None => layout("Terms of Service", true, Page::Other, body),
    }
}
