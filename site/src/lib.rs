use leptos::prelude::*;
use leptos_meta::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    StaticSegment,
};

pub mod pages;
// Top-Level pages
use pages::home::Home;
use pages::not_found::NotFound;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
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
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio.css" />

        // <Html lang="en" dir="ltr" attr:data-theme="dark" />
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
                <Route
                    path=StaticSegment("linuxplus")
                    view=|| {
                        view! { <Redirect path="/assets/LinuxPlus.pdf" /> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
