use maud::{html, Markup, DOCTYPE};

use crate::{config::config, web::templates::Page};

pub fn layout(title: &str, is_show_nav: bool, page: Page, content: Markup) -> Markup {
    html!(
        (DOCTYPE)
        html lang="en" {
            (head(title))
            @if is_show_nav {
                (nav(page))
            }
            body class="h-screen font-sans anti-aliased" {
                main class="grid h-full w-full md:grid-cols-12" {
                    (content)
                    @if is_show_nav {
                        (footer())
                    }
                }
            }
            script defer src="/public/js/core.min.js" {}
        }
    )
}

pub fn head(title: &str) -> Markup {
    html!(
        head {
            title {
                @if title.is_empty() {
                    "Heavy Metal Releases"
                } @else {
                    (title) " | Heavy Metal Releases"
                }
            }
            meta charset="UTF-8";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            meta name="description" content="Be notified of new heavy metal album releases.";
            meta name="keywords" content="heavy metal, album releases, automation";
            link rel="canonical" href="https://metal.musicavis.ca/";
            link rel="icon" href="/public/favicon.png" type="image/x-icon";
            link rel="stylesheet" href="/public/css/tailwind.css";
            link rel="alternate" type="application/rss+xml" title="Heavy Metal Releases Feed" href=(format!("{}/calendar/feed.xml", config().BASE_URL));
            script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous" {}
            script src="https://unpkg.com/htmx-ext-multi-swap@2.0.0/multi-swap.js" {}
            script src="https://unpkg.com/hyperscript.org@0.9.13" {}
        }
    )
}

pub(crate) fn nav(page: Page) -> Markup {
    let nav_items = nav_items(page);

    html!(
        nav {
            div class="navbar bg-base-200" {
                div class="navbar-start" {
                    img src="/public/img/logo-64x64.png" alt="logo" class="w-[2.5rem]";
                    a href="/" class="btn btn-ghost text-xl" { "Heavy Metal Releases" }
                }
                div class="navbar-end" {
                    div class="dropdown dropdown-end" {
                        div tabindex="0" role="button" class="btn btn-ghost lg:hidden" {
                            svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor" {
                                path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h8m-8 6h16";
                            }
                        }
                        ul tabindex="0" class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow" {
                            (nav_items)
                        }
                    }
                }
                div class="navbar-end hidden lg:flex" {
                    ul class="menu menu-horizontal px-1" {
                        (nav_items)
                    }
                }
            }
        }
    )
}

fn nav_items(page: Page) -> Markup {
    html!(
        li {
            a href="/" class={
                @if page == Page::Home { "font-bold"}
                @if page != Page::Home { " hover:text-gray-800 dark:hover:text-gray-300" }
            } { "Home" }
        }
        li {
            a href="/calendar" class={
                @if page == Page::Calendar { "font-bold"}
                @if page != Page::Calendar { " hover:text-gray-800 dark:hover:text-gray-300" }
            } { "Calendar" }
        }
        li {
            a href="/about" class={
                @if page == Page::About { "font-bold"}
                @if page != Page::Home { " hover:text-gray-800 dark:hover:text-gray-300" }
            } { "About" }
        }
        li {
            a href="/contact" class={
                @if page == Page::Contact { "font-bold"}
                @if page != Page::Home { " hover:text-gray-800 dark:hover:text-gray-300" }
            } { "Contact" }
        }
    )
}

fn footer() -> Markup {
    html!(
        footer class="col-span-12 bg-gray-100 dark:bg-black" {
            div class="container mx-auto pt-10 pb-6" {
                div class="flex flex-wrap" {
                    div class="w-full md:w-1/3 text-center md:text-center" {
                        h5 class="uppercase mb-6 font-bold"{
                            "Links"
                        }
                        ul class="mb-4" {
                            li class="mt-2" {
                                a href="/contact" class="hover:underline hover:text-orange-500" {
                                    "Support"
                                }
                            }
                            li class="mt-2" {
                                a href="/sitemap.xml" class="hover:underline hover:text-orange-500" {
                                    "Sitemap"
                                }
                            }
                        }
                    }
                    div class="w-full md:w-1/3 text-center md:text-center" {
                        h5 class="uppercase mb-6 font-bold" {
                            "Legal"
                        }
                        ul class="mb-4" {
                            li class="mt-2" {
                                a href="/tos" class="hover:underline hover:text-orange-500" {
                                    "Terms of Service"
                                }
                            }
                            li class="mt-2" {
                                a href="/privacy" class="hover:underline hover:text-orange-500" {
                                    "Privacy Policy"
                                }
                            }
                        }
                    }
                    div class="w-full md:w-1/3 text-center md:text-center" {
                        h5 class="uppercase mb-6 font-bold" {
                            "Service"
                        }
                        ul class="mb-4" {
                            li class="mt-2" {
                                a href="/about" class="hover:underline hover:text-orange-500" {
                                    "About Us"
                                }
                            }
                            li class="mt-2" {
                                a href="/contact" class="hover:underline hover:text-orange-500" {
                                    "Contact"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
