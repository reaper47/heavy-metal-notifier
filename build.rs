use std::{env, process, process::Command};

fn main() {
    if env::var("SKIP_BUILD_RS").is_ok() {
        println!("Skipping build.rs tasks");
        return;
    }

    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "install", "./web/app"])
        .status()
        .expect("Failed to install npm packages");

    if !status.success() {
        println!("npm install failed with status: {status}");
        process::exit(1);
    }

    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "run", "build"])
        .status()
        .expect("Failed to build the web app");

    if !status.success() {
        println!("npm build failed with status: {status}");
        process::exit(1);
    }
}
