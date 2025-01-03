use std::process::Command;

fn main() {
    let status = Command::new("npm").args(["-v"]).status()
        .expect("NPOM not installed");

    if !status.success() {
        eprintln!("npm failed with status: {}", status);
        std::process::exit(1);
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
