use crate::components::ProgressBar;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <NavBar></NavBar>
            <Intro></Intro>
            <AboutMe></AboutMe>
            <Projects></Projects>

        </ErrorBoundary>
    }
}

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="container" id="intro">
            <img src="assets/headshot.jpg" width="200" height="300" />
            <h1>"Sawyer Bristol"</h1>
            <h3>
                "A systems programmer who loves Rust"
            </h3>

            <div class="socials">
                <a class="devicon-github-original" href="https://github.com/LegitCamper"></a>
                <a
                    class="devicon-linkedin-plain"
                    href="https://www.linkedin.com/in/sawyerbristol/"
                ></a>
                <a class="devicon-twitter-original" href="https://x.com/BristolSawyer"></a>
            </div>

            <div class="scroll-down">
                <a href="#about_me">"Scroll Down"
                    <span class="mouse">
                        <span class="wheel"></span>
                    </span>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <div class="container" id="about_me">
            <h1>"About Me"</h1>

            <div class="languages">
            <a>"Rust"</a>
            <ProgressBar progress=75></ProgressBar>
            </div>

            <a href="assets/resume.pdf">"Download my resume"</a>
        </div>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="container" id="projects">
            <h1>"Projects"</h1>
        </div>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar">
            <a href="#intro">Home</a>
            <a href="#about_me">About Me</a>
            <a href="#projects">Projects</a>
            <a href="#contact">Contact</a>
        </div>
    }
}
