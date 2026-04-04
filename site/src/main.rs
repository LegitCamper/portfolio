use rocket::fs::{FileServer, Options};

#[macro_use]
extern crate rocket;

#[cfg(debug_assertions)]
const PUBLIC_DIR: &str = "site/public";

#[cfg(not(debug_assertions))]
const PUBLIC_DIR: &str = "public";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![home, cs50x, linuxplus, secplus, conf, oss, itfplus, ccna],
        )
        .mount("/", FileServer::new(PUBLIC_DIR, Options::None))
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}

macro_rules! redirect_route {
    ($name:ident, $path:literal, $target:literal) => {
        #[get($path)]
        fn $name() -> rocket::response::Redirect {
            rocket::response::Redirect::to($target)
        }
    };
}

redirect_route!(home, "/", "/home.html");
redirect_route!(cs50x, "/cs50x", "/assets/CS50x.pdf");
redirect_route!(linuxplus, "/linuxplus", "/assets/LinuxPlus.pdf");
redirect_route!(secplus, "/secplus", "/assets/SecurityPlus.pdf");
redirect_route!(conf, "/conf", "/assets/AzureConfig.pdf");
redirect_route!(oss, "/oss", "/assets/AzureOss.pdf");
redirect_route!(itfplus, "/itfplus", "/assets/ITFPlus.pdf");
redirect_route!(
    ccna,
    "/ccna",
    "https://www.credly.com/earner/earned/badge/5fb6df62-6a0c-4726-ba18-f53ac658848e"
);
