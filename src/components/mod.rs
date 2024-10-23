use leptos::*;

#[component]
pub fn ProgressBar(progress: i32) -> impl IntoView {
    view! { <progress max="50" value=progress /> }
}
