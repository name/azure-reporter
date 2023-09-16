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

    // Get the user info from the output (strip the brackets at the start and end to make it valid JSON)
    let user_info = &String::from_utf8_lossy(&output.stdout)[1..output.stdout.len() - 3];

    // Parse the JSON
    let user_info_json = json::parse(&user_info);

    let username = match user_info_json {
        Ok(json::JsonValue::Object(ref user_info)) => {
            if let Some(name) = user_info["user"]["name"].as_str() {
                name.to_string() // Clone the name
            } else {
                println!("Username not found in JSON.");
                return;
            }
        }
        Ok(_) => {
            println!("Invalid JSON structure.");
            return;
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            return;
        }
    };

    let subscription_name = match user_info_json {
        Ok(json::JsonValue::Object(ref user_info)) => {
            if let Some(subscription) = user_info["name"].as_str() {
                subscription.to_string() // Clone the subscription
            } else {
                println!("Subscription not found in JSON.");
                return;
            }
        }
        Ok(_) => {
            println!("Invalid JSON structure.");
            return;
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            return;
        }
    };

    let subscription_id = match user_info_json {
        Ok(json::JsonValue::Object(ref user_info)) => {
            if let Some(subscription) = user_info["id"].as_str() {
                subscription.to_string() // Clone the subscription
            } else {
                println!("Subscription not found in JSON.");
                return;
            }
        }
        Ok(_) => {
            println!("Invalid JSON structure.");
            return;
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            return;
        }
    };

    println!(
        "You are logged in as {} to subscription '{}' ({})",
        username,
        subscription_name,
        subscription_id
    );
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

    // Get the user info from the output
    let user_info = String::from_utf8_lossy(&output.stdout);

    // Parse the JSON
    let user_info_json = json::parse(&user_info);

    let username = match user_info_json {
        Ok(json::JsonValue::Object(ref user_info)) => {
            if let Some(name) = user_info["user"]["name"].as_str() {
                name.to_string() // Clone the name
            } else {
                println!("Username not found in JSON.");
                return;
            }
        }
        Ok(_) => {
            println!("Invalid JSON structure.");
            return;
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            return;
        }
    };

    let subscription_name = match user_info_json {
        Ok(json::JsonValue::Object(ref user_info)) => {
            if let Some(subscription) = user_info["name"].as_str() {
                subscription.to_string() // Clone the subscription
            } else {
                println!("Subscription not found in JSON.");
                return;
            }
        }
        Ok(_) => {
            println!("Invalid JSON structure.");
            return;
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            return;
        }
    };

    let subscription_id = match user_info_json {
        Ok(json::JsonValue::Object(ref user_info)) => {
            if let Some(subscription) = user_info["id"].as_str() {
                subscription.to_string() // Clone the subscription
            } else {
                println!("Subscription not found in JSON.");
                return;
            }
        }
        Ok(_) => {
            println!("Invalid JSON structure.");
            return;
        }
        Err(err) => {
            println!("Error parsing JSON: {}", err);
            return;
        }
    };

    println!(
        "You are logged in as {} to subscription '{}' ({})",
        username,
        subscription_name,
        subscription_id
    );
}
