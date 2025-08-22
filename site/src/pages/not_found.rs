use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! { <h1>"Uh oh!" <br /> "I couldn't find that page!"</h1> }
}
