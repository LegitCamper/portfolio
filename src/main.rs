use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

// Top-Level pages
use pages::home::Home;
use pages::not_found::NotFound;

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
