use leptos::prelude::*;

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
            <Arcade></Arcade>
            <Projects></Projects>

        </ErrorBoundary>
    }
}

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <style>
            "#intro {
                color: #cad3f5;
                vertical-align: middle;
                flex-wrap: wrap;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                text-align: center;
                min-height: 100vh;
            }"
        </style>
        <div id="intro">
            <img src="assets/headshot.jpg" width="200" height="300" />
            <h1>"Sawyer Bristol"</h1>
            <h3>"A systems' programmer who loves Rust"</h3>
            <img src="https://img.shields.io/github/stars/LegitCamper" />

            <style>
                ".socials {
                    display: flex;
                    flex-direction: row;
                    justify-content: space-around;
                    align-items: center;
                    font-size: 40px;
                    padding: 10px;
                    margin: 50;
                }  
                .socials a {
                    color: white;
                    text-decoration: none;
                    padding: 20px;
                }"
            </style>
            <div class="socials">
                <a class="devicon-github-original" href="https://github.com/LegitCamper" />
                <a
                    class="devicon-linkedin-plain"
                    href="https://www.linkedin.com/in/sawyerbristol/"
                ></a>
                <a class="devicon-twitter-original" href="https://x.com/BristolSawyer" />
            </div>

            <div class="scroll-down">
                <a href="#about_me">
                    "Scroll Down" <span class="mouse">
                        <span class="wheel" />
                    </span>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <style>
            "#about_me {
                  color: #cad3f5;
                  vertical-align: middle;
                  flex-wrap: wrap;
            }"
        </style>
        <div class="container" id="about_me">
            <h1>"About Me"</h1>
            <div class="box">
                <img src="assets/me_programming.jpg" width="145" height="210" />
                <p>
                    "As an experienced Rust developer with a passion for systems development,
                    embedded devices, DevOps, and self-hosting, I am dedicated to creating safe 
                    and high-performant software solutions that solve real world problems. 
                    With a strong drive to learn and grow as a programmer, I am constantly 
                    make a meaningful impact on people's livesseeking out new challenges and 
                    opportunities to improve my skills and expertise. 
                    Check out my resume or projects below for more examples of what I build!"
                </p>

                <style>
                    ".resume_download {
                        background-color: #b7bdf8;
                        border-radius: 30px;
                        color: #181926;
                        padding: 12px 20px;
                        cursor: pointer;
                        font-size: 15px;
                        max-width: 500px;
                    }"
                </style>
                <button class="resume_download" onclick="location.href='assets/resume.pdf'">
                    "Download Resume"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Arcade() -> impl IntoView {
    view! {
        <style>
            "#arcade {
                  color: #cad3f5;
                  vertical-align: middle;
                  flex-wrap: wrap;
            }"
        </style>
        <div class="container" id="arcade">
            <h1>"Arcade"</h1>
            <div class="box">
                <div>
                    <canvas id="snake-canvas" width="1280" height="720"></canvas>
                    <script type="module">
                        r#"
                        import init from './snake.js'
                        init()
                        "#
                    </script>
                    </div>
            </div>
        </div>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <style>
            "#projects {
                color: #cad3f5;
                vertical-align: middle;
                flex-wrap: wrap;
            }"
        </style>
        <div class="container" id="projects">
            <h1>"Projects"</h1>
            <div class="box">
                <h3>"Icy Browser & Components"</h3>
                <style>
                    ".icy_browser_img {
                        max-width: 85%;
                        min-width: 225px;
                        height: auto;
                    }"
                </style>
                <img
                    class="icy_browser_img"
                    src="https://github.com/LegitCamper/icy_browser/blob/main/assets/basic_browser.png?raw=true"
                />
                <a href="https://github.com/LegitCamper/iced_webview">"Iced Webview"</a>
                <p>"A Iced library to embed webpages in native iced applications"</p>

                <a href="https://github.com/LegitCamper/icy_browser">"Icy_browser"</a>
                <p>
                    "An Iced library built on Iced web view to build browser. Includes tab bar, navigation bar, and bookmark bar"
                </p>

                <a href="https://github.com/LegitCamper/rust-browser">"Rust browser"</a>
                <p>
                    "Uses the above two libraries to create my own shortcut driven browser in Rust!"
                </p>
            </div>

            <div class="box">
                <h3>
                    <a href="https://github.com/LegitCamper/homelab">"My homelab repo"</a>
                </h3>
                <p>
                    "This is repo where I host all the containers
                     I use in my Homelab along with all the Ansible 
                     scripts and CI/CD that makes the magic happen.
                     This is also the server I am using to 
                     host this very site. Magical!"
                </p>
            </div>

            <div class="box">
                <h3>
                    <a href="https://github.com/LegitCamper/portfolio">"My portfolio site!"</a>
                </h3>
                <p>
                    "I can't forget about this site!
                    A static site written in Rust and WebAssembly. 
                    It uses the Leptos framework to create
                    blazingly fast static sites like this one.
                    I am using github actions to build a new container
                    and my homelab repo to automatically run ansible 
                    and redeploy this site"
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar">
            <a href="#intro">Home</a>
            <a href="#about_me">About Me</a>
            <a href="#arcade">Arcade</a>
            <a href="#projects">Projects</a>
        </div>
    }
}
