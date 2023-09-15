use std::env;
use std::process::{ Command };

const AZURE_CLI_PATH: &str = "C:\\Program Files\\Microsoft SDKs\\Azure\\CLI2\\wbin\\az.cmd";

fn main() {
    // Access command-line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check that the Azure CLI is installed
    match check_preconditions() {
        Ok(_) => {
            // Azure CLI is installed, proceed with other checks
            if args.len() != 2 {
                println!("Usage: {} <action>", args[0]);
                println!("Where <action> is one of:");
                println!("  login");
                println!("  logout");
                return;
            }

            // Get the package and install parameters
            let action = &args[1];

            match action.as_str() {
                "login" => {
                    azure_login();
                }
                "logout" => {
                    azure_logout();
                }
                "subscriptions" => {
                    azure_check_login();
                    azure_get_subscriptions();
                }
                _ => {
                    println!("Unknown action: {}", action);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn azure_get_subscriptions() {
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

fn azure_check_login() {
    println!("Checking login status...");
    let output = Command::new(AZURE_CLI_PATH)
        .arg("account")
        .arg("show")
        .output()
        .expect("Failed to execute az account show");

    if !output.status.success() {
        azure_login();
    }
}

fn azure_login() {
    println!("Logging in...");
    let output = Command::new(AZURE_CLI_PATH)
        .arg("login")
        .output()
        .expect("Failed to execute az login");

    if !output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let user_info = String::from_utf8_lossy(&output.stdout);
    println!("User Info: {}", user_info);

    println!("Login complete.");
}

fn azure_logout() {
    println!("Logging out...");
    let output = Command::new(AZURE_CLI_PATH)
        .arg("logout")
        .output()
        .expect("Failed to execute az logout");

    if !output.status.success() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Logout complete.");
}

fn check_preconditions() -> Result<(), String> {
    let output = Command::new(AZURE_CLI_PATH).arg("--version").output();

    match output {
        Ok(output) if output.status.success() => {
            println!("Azure CLI is installed.");
            Ok(())
        }
        _ => {
            Err(
                "The Azure 64-bit CLI is not installed. Please install it from https://docs.microsoft.com/en-us/cli/azure/install-azure-cli".to_string()
            )
        }
    }
}
