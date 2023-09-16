use std::env;
use std::process::{ Command };

extern crate json;

mod azure;

const AZURE_CLI_PATH: &str = "C:\\Program Files\\Microsoft SDKs\\Azure\\CLI2\\wbin\\az.cmd";

fn main() {
    // Access command-line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check that the Azure CLI is installed
    match azure_check_cli() {
        Ok(_) => {
            // Azure CLI is installed, proceed with other checks
            if args.len() <= 1 {
                print_usage();
                return;
            }

            // Check that the user is logged in
            azure::auth::check();

            // Get the package and install parameters
            let action = &args[1];

            match action.as_str() {
                "whoami" => {
                    azure::auth::whoami();
                }
                "login" => {
                    azure::auth::login();
                }
                "logout" => {
                    azure::auth::logout();
                }
                "subscriptions" => {
                    azure_get_subscriptions();
                }
                _ => {
                    println!("Unknown action: {}", action);
                    print_usage();
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn print_usage() {
    println!("Usage: {} <action>", env::args().nth(0).unwrap());
    println!("Actions:");
    println!("  whoami - Show information about the currently logged in user");
    println!("  login - Log in to Azure");
    println!("  logout - Log out of Azure");
    println!("  subscriptions - List available subscriptions");
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

fn azure_check_cli() -> Result<(), String> {
    let output = Command::new(AZURE_CLI_PATH).arg("--version").output();

    match output {
        Ok(output) if output.status.success() => { Ok(()) }
        _ => {
            Err(
                "The Azure 64-bit CLI is not installed. Please install it from https://docs.microsoft.com/en-us/cli/azure/install-azure-cli".to_string()
            )
        }
    }
}
