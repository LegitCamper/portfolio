pub mod pages;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    StaticSegment,
};

// Top-Level pages
use pages::{home::Home, not_found::NotFound};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! { styles,
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />

                <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" />

                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap"
                    rel="stylesheet"
                />

                // github code graph
                <link rel="stylesheet" href="https://ghcg.lyova24.ru/static/gh.css" />

                <Style>
                    "body {
                        background-color: #24273a;
                        font-family: 'JetBrains Mono', monospace;
                        text-align: center;
                    }
                    h1 {
                        font-size: 2.5rem;
                    }

                    h2 {
                        font-size: 2rem;
                    }

                    p {
                        line-height: 1.6;
                    }"
                </Style>

            </head>
            <body>
                <App />
                <script src="https://ghcg.lyova24.ru/static/gh.js"></script>
            </body>
        </html>
    }
}

#[component]
fn Cert_Redirect(url: &'static str) -> impl IntoView {
    view! {
        If you are not redirected automatically,
        <a href=url>follow the link</a>
        <main>
            <Meta charset="utf-8" />
            <Meta name="description" content="A Cert short link redirection" />
            <Meta http_equiv="refresh" content=format!("3;url={}", url) />
        </main>
    }
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Portfolio | Sawyer Bristol" />

        <Router>
            <Routes fallback=NotFound>
                <Route path=StaticSegment("") view=Home />

                <Route
                    path=StaticSegment("secplus")
                    view=|| {
                        view! { <Redirect path="/assets/SecurityPlus.pdf" /> }
                    }
                />
                <Route
                    path=StaticSegment("ccna")
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://www.credly.com/earner/earned/badge/5fb6df62-6a0c-4726-ba18-f53ac658848e" />
                        }
                    }
                />
                <Route
                    path=StaticSegment("conf")
                    view=|| {
                        view! { <Redirect path="/assets/AzureConfig.pdf" /> }
                    }
                />
                <Route
                    path=StaticSegment("oss")
                    view=|| {
                        view! { <Redirect path="/assets/AzureOss.pdf" /> }
                    }
                />
                <Route
                    path=StaticSegment("itfplus")
                    view=|| {
                        view! { <Redirect path="/assets/ITFPlus.pdf" /> }
                    }
                />
            </Routes>
        </Router>
    }
}
