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
            <h3>"A systems programmer who loves Rust"</h3>

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
                <a class="devicon-github-original" href="https://github.com/LegitCamper"></a>
                <a
                    class="devicon-linkedin-plain"
                    href="https://www.linkedin.com/in/sawyerbristol/"
                ></a>
                <a class="devicon-twitter-original" href="https://x.com/BristolSawyer"></a>
            </div>

            <div class="scroll-down">
                <a href="#about_me">
                    "Scroll Down" <span class="mouse">
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
        <style>
            "#about_me {
              color: #cad3f5;
              vertical-align: middle;
              flex-wrap: wrap;
            }"
        </style>
        <div class="container" id="about_me">
            <h1>"About Me"</h1>

            <style>
                ".about_me_box{
                background: #494d64; 
                border-radius: 30px;
                width:50%;
                margin: 10px;
                padding: 10px;
                display: flex;
                flex-direction: row;
                justify-content: space-around;
                align-items: center;
                }"
            </style>
            <div class="about_me_box">
                <img src="assets/me_programming.jpg" width="145" height="210" />
                <style>
                    ".bio{
                    display: flex;
                    width: 25%;
                    flex-direction: column;
                    justify-content: space-around;
                    align-items: center;
                    padding: 10px;
                    margin: 50;
                    }
                    .bio p {
                    overflow-wrap: break-word;
                    white-space: normal;
                    }"
                </style>
                <div class="bio">
                    <p>
                        "A passionate systems programmer
                        with a strong interest in DevOps, 
                        eager to apply my skills to
                        contribute and solve difficult problems."
                    </p>

                    <style>
                        ".resume_download {
                        background-color: #b7bdf8;
                        border-radius: 30px;
                        color: #181926;
                        padding: 12px 20px;
                        cursor: pointer;
                        font-size: 15px;
                        }"
                    </style>
                    <button class="resume_download" onclick="location.href='assets/resume.pdf'">
                        "Download Resume"
                    </button>
                </div>

                <style>
                    ".skills {
                    display: flex;
                    flex-direction: column;
                    justify-content: space-around;
                    align-items: center;
                    padding: 10px;
                    align-items: start;
                    margin: 50;
                    }
                    .skills p {
                    text-align: left
                    padding: 10px;
                    margin: 10px;
                    }"        
                </style>
                <div class="skills">
                    <p align="center" style="font-size: 25px;">"Skills"</p>
                    <p>"• Programming Languages: Bash, Rust, Golang, C, Python, Lua, Nix/NixOs"</p>
                    <p>"• Deployment Technologies: Kubernetes, Docker, GitHub Actions, Azure Dev Ops, Ansible, CI/CD"</p>
                    <p>"• Infrastructure: AWS, Azure, Google Cloud, Digital Ocean"</p>
                    <p>"• Networking: Nginx/Traefik(HTTP, HTTPS, TCP, UDP routing), IPTables, Tailscale"</p>
                    <p>"• Interests: Systems & Embedded programming, Cyber-security, and self-hosting"</p>
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
