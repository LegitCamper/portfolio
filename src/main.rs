use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <Html lang="en" dir="ltr" attr:data-theme="dark" />
        <Title text="Portfolio | Sawyer Bristol" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes>
                <Route path="/" view=Home />

                <Route
                    path="/secplus"
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://www.credly.com/earner/earned/badge/ee6d361d-f5a0-477e-8781-dac694de1576" />
                        }
                    }
                />
                <Route
                    path="/ccna"
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://www.credly.com/earner/earned/badge/5fb6df62-6a0c-4726-ba18-f53ac658848e" />
                        }
                    }
                />
                <Route
                    path="/conf"
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://certificates.cloudacademy.com/f6dc9411a22f9bd18e3da429caf6690cf3264e4c.pdf" />
                        }
                    }
                />
                <Route
                    path="/oss"
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://certificates.cloudacademy.com/bc74fa5d870d4a33fab262d7820e685999c6bb1d.pdf" />
                        }
                    }
                />
                <Route
                    path="/itfplus"
                    view=|| {
                        view! {
                            <Cert_Redirect url="https://www.credly.com/earner/earned/badge/1e791160-f8c8-4306-87a1-f3a480898ad6" />
                        }
                    }
                />

                <Route path="/*" view=NotFound />
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
