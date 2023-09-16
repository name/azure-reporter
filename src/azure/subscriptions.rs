use std::env;
use std::process::{ Command };
use json;

const AZURE_CLI_PATH: &str = "C:\\Program Files\\Microsoft SDKs\\Azure\\CLI2\\wbin\\az.cmd";

pub fn list() {
    println!("Getting subscriptions...");
    let output = Command::new(AZURE_CLI_PATH)
        .arg("account")
        .arg("list")
        .output()
        .expect("Failed to execute az account list");

    if !output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let subscriptions = String::from_utf8_lossy(&output.stdout);
    println!("Subscriptions: {}", subscriptions);
}
