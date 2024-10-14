use std::process::Command;

fn main() {
    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "install", "./web/app"])
        .status()
        .expect("Failed to install npm packages");

    if !status.success() {
        panic!("npm install failed")
    }

    let status = Command::new("npm")
        .args(["--prefix", "./web/app", "run", "build"])
        .status()
        .expect("Failed to build the web app");

    if !status.success() {
        panic!("npm build failed");
    }
}
