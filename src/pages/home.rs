use leptos::prelude::*;
use styled::style;

#[component]
pub fn Home() -> impl IntoView {
    let styles = style! {
        .site-wrapper {
            padding: var(--space-lg);
            max-width: 1200px;
            margin: 0 auto;
        }
        .section {
            margin-bottom: var(--space-lg);
            padding-top: var(--space-md);
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            text-align: center;
            min-height: 25vh;
            color: #cad3f5;
            vertical-align: middle;
            flex-wrap: wrap;
        }
        .section:first-of-type {
            margin-top: 100px;
        }
        .section img {
            max-width: 100%;
            height: auto;
            display: block;
            gap: 1rem;
            margin: 0 auto;
        }
        .section a {
            color: "#3489ee";
        }
        .section p {
            text-align: left;
            padding: 10px;
            margin: 10px;
            overflow-wrap: break-word;
            white-space: normal;
        }
        .section_box {
            background: #494d64;
            border-radius: 30px;
            width: 80%;
            margin: 25px auto;
            padding: 20px;
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
            display: flex;
            flex-direction: column;
            gap: 20px;
        }
    };
    styled::view! {
        styles,
        <div class="site-wrapper">
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
                <Skills></Skills>
                <Projects></Projects>

            </ErrorBoundary>
        </div>
    }
}

#[component]
pub fn Intro() -> impl IntoView {
    let styles = style! {
        .socials {
            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: center;
            gap: 1rem;
            margin-top: 1rem;
        }
        .socials a {
            color: white;
            text-decoration: none;
        }
        .socials a img {
            height: 30px;
            width: auto;
        }
        .socials a.devicon-linkedin-plain {
            font-size: 30px;
            color: white;
        }
        .algora-icon {
            width: 40px;
            height: 40px;
            cursor: pointer;
            fill: white;
        }
        #gh {
            margin: 2rem 0;
        }
    };
    styled::view! { styles,
        <div id="intro" class="section">
            <img src="assets/headshot.jpg" width="200" height="300" />
            <h1>"Sawyer Bristol"</h1>
            <h3>"Systems Programmer Specializing in Rust"</h3>

            <div class="socials">
                <a href="https://github.com/LegitCamper" target="_blank" rel="noopener noreferrer">
                    <img src="https://img.shields.io/github/stars/LegitCamper?style=social" alt="GitHub stars" />
                </a>
                <a
                    class="devicon-linkedin-plain"
                    href="https://www.linkedin.com/in/sawyerbristol/"
                ></a>
                <a href="https://algora.io/profile/LegitCamper" target="_blank" rel="noopener noreferrer" aria-label="Algora profile">
                    <img
                        src="https://algora.io/favicon.ico"
                        alt="Algora logo"
                        width="40"
                        height="40"
                        style="border-radius: 6px;"
                    />
                </a>
            </div>

            <div id="gh" data-login="LegitCamper"></div>
        </div>
    }
}

#[component]
pub fn AboutMe() -> impl IntoView {
    let styles = style! {
        .resume_download {
            background-color: #b7bdf8;
            border-radius: 30px;
            color: #181926;
            padding: 12px 20px;
            cursor: pointer;
            font-size: 15px;
        }
    };
    styled::view! { styles,
        <div class="section" id="about_me">
            <div class="section_box">
                <h1>"About Me"</h1>
                <img src="assets/me_programming.jpg" width="145" height="210" />
                <div class="bio">
                    <p>
                        "A passionate systems programmer
                        with a strong interest in DevOps, 
                        eager to apply my skills to
                        contribute and solve difficult problems."
                    </p>

                    <button class="resume_download" onclick="location.href='assets/resume.pdf'">
                        "Download Resume"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <div class="section" id="skills">
            <div class="section_box">
                <h1>"Skills"</h1>
                <div class="skills">
                    <p>
                        "• Programming Languages: Bash, Rust, Golang, C, Python, Lua, Nix/NixOs"
                    </p>
                    <p>
                        "• Deployment Technologies: Kubernetes, Docker, GitHub Actions, Azure Dev Ops, Ansible, CI/CD"
                    </p>
                    <p>"• Infrastructure: AWS, Azure, Google Cloud, Digital Ocean"</p>
                    <p>
                        "• Networking: Nginx/Traefik(HTTP, HTTPS, TCP, UDP routing), IPTables, Tailscale"
                    </p>
                    <p>
                        "• Interests: Systems & Embedded programming, Cyber-security, and self-hosting"
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let styles = style! {
        .project-card {
            background: #5a5f7a;
            border-radius: 20px;
            padding: 15px;
            display: flex;
            flex-direction: column;
            align-items: center;
            box-shadow: 0 4px 8px rgba(0,0,0,0.3);
        }

        .project-card a {
            color: "#3489ee";
            text-decoration: none;
        }
        .project-card a:hover {
            text-decoration: underline;
        }

        .project-card img {
            max-width: 100%;
            height: auto;
            border-radius: 15px;
            margin-bottom: 10px;
        }
    };
    styled::view! { styles,
        <div class="section" id="projects">
            <div class="section_box">
                <h1>"Projects"</h1>

                <div class="project-card icy_browser">
                    <h3>"Icy Browser & Components"</h3>
                    <img
                        src="https://github.com/LegitCamper/icy_browser/blob/main/assets/basic_browser.png?raw=true"
                        alt="Icy Browser screenshot"
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

                <div class="project-card homelab">
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

                <div class="project-card portfolio">
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
        </div>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let styles = style! {
        .navbar {
            overflow: hidden;
            background-color: #181926;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            width: 100vw;
            margin: 0;
            padding: 0;
        }

        .navbar a {
            float: left;
            display: block;
            color: #cad3f5;
            text-align: center;
            padding: 14px 16px;
            text-decoration: none;
        }

        .navbar a:hover {
            background: #939ab7;
        }
    };
    styled::view! { styles,
        <div class="navbar">
            <a href="#intro">Home</a>
            <a href="#about_me">About Me</a>
            <a href="#skills">Skills</a>
            <a href="#projects">Projects</a>
        </div>
    }
}
