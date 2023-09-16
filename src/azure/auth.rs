use std::env;
use std::process::{ Command };
use json;

const AZURE_CLI_PATH: &str = "C:\\Program Files\\Microsoft SDKs\\Azure\\CLI2\\wbin\\az.cmd";

pub fn login() {
    let output = Command::new(AZURE_CLI_PATH)
        .arg("login")
        .output()
        .expect("Failed to execute az login");

    if !output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let user_info = String::from_utf8_lossy(&output.stdout);

    // Parse the JSON
    let user_info_json = json::parse(&user_info);

    match user_info_json {
        Ok(json::JsonValue::Array(mut user_info_array)) => {
            if let Some(user_obj) = user_info_array.pop() {
                // Access additional fields as needed
                if let Some(name) = user_obj["user"]["name"].as_str() {
                    println!("User Name: {}", name);
                } else {
                    println!("User name not found in JSON.");
                }

                if let Some(cloud_name) = user_obj["cloudName"].as_str() {
                    println!("Cloud Name: {}", cloud_name);
                } else {
                    println!("Cloud name not found in JSON.");
                }

                // Access more fields in a similar manner
            } else {
                println!("Empty JSON array.");
            }
        }
        _ => {
            println!("Invalid JSON structure.");
        }
    }

    println!("Login complete.");
}

pub fn logout() {
    let output = Command::new(AZURE_CLI_PATH)
        .arg("logout")
        .output()
        .expect("Failed to execute az logout");

    if !output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Logout complete.");
}

pub fn check() {
    let output = Command::new(AZURE_CLI_PATH)
        .arg("account")
        .arg("show")
        .output()
        .expect("Failed to execute az account show");

    if !output.status.success() {
        login();
    }
}

pub fn whoami() {
    let output = Command::new(AZURE_CLI_PATH)
        .arg("account")
        .arg("show")
        .output()
        .expect("Failed to execute az account show");

    if !output.status.success() {
        println!(
            "You are currently not logged in. Please run {} login",
            env::args().nth(0).unwrap()
        );
        return;
    }

    let user_info = String::from_utf8_lossy(&output.stdout);
    println!("User Info: {}", user_info);
}
