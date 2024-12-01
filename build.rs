use std::path::Path;

fn main() {
    let dir = std::env::var("OUT_DIR").unwrap();
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    std::process::Command::new("git")
        .args(["clone", "https://github.com/LegitCamper/resume", &dir])
        .spawn()
        .expect("Failed to clone resume repo");
    std::process::Command::new("typst")
        .args([
            "compile",
            &format!("{}/resume/resume.typ", &dir),
            &format!("{}/{}", cargo_dir, "public/assets/resume.pdf"),
        ])
        .spawn()
        .expect("Failed to compile resume");

    assert!(Path::new(&cargo_dir)
        .join("public/assets/resume.pdf")
        .exists());
}
