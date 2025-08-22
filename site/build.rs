use std::path::Path;

fn main() {
    let dir = std::env::var("OUT_DIR").unwrap();
    std::process::Command::new("git")
        .args(["clone", "https://github.com/LegitCamper/resume", &dir])
        .spawn()
        .expect("Failed to clone resume repo")
        .wait()
        .unwrap();

    let resume =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("public/assets/resume.pdf");
    std::process::Command::new("typst")
        .args([
            "compile",
            &format!("{}/resume.typ", &dir),
            resume.as_os_str().to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to compile resume")
        .wait()
        .unwrap();

    assert!(resume.exists());
}
