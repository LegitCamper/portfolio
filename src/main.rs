use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

mod pages;
// Top-Level pages
use pages::home::Home;
use pages::not_found::NotFound;

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
        // <Html lang="en" dir="ltr" attr:data-theme="dark" />
        <Title text="Portfolio | Sawyer Bristol" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("") view=Home />

                <Route
                    path=path!("secplus")
                    view=|| {
                        view! {
                            <Redirect path="/assets/SecurityPlus.pdf" />
                        }
                    }
                />
                <Route
                    path=path!("ccna")
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://www.credly.com/earner/earned/badge/5fb6df62-6a0c-4726-ba18-f53ac658848e" />
                        }
                    }
                />
                <Route
                    path=path!("conf")
                    view=|| {
                        view! {
                            <Redirect path="/assets/AzureConfig.pdf" />
                        }
                    }
                />
                <Route
                    path=path!("oss")
                    view=|| {
                        view! {
                            <Redirect path="/assets/AzureOss.pdf" />
                        }
                    }
                />
                <Route
                    path=path!("itfplus")
                    view=|| {
                        view! {
                            <Redirect path="/assets/ITFPlus.pdf" />
                        }
                    }
                />
            </Routes>
        </Router>
    }
}

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
