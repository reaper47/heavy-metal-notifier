use std::process::Command;

fn main() {
    if std::env::var("SKIP_BUILD_RS").is_ok() {
        println!("Skipping build.rs tasks");
        return;
    }

    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "install", "./web/app"])
        .status()
        .expect("Failed to install npm packages");

    if !status.success() {
        eprintln!("npm install failed with status: {}", status);
        std::process::exit(1);
    }

    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "run", "build"])
        .status()
        .expect("Failed to build the web app");

    if !status.success() {
        eprintln!("npm build failed with status: {}", status);
        std::process::exit(1);
    }
}
