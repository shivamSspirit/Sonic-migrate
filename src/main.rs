use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::exit;
use std::time::Duration;

mod cli;
mod errors;
mod migration;
use cli::{Config, Network};
use migration::{restore_backup, run_migration};

fn main() {
    let config = Config::new();

    // The networks command is handled in Config::new()
    // If we got here, it wasn't called or it would have exited

    if config.verbose {
        println!("{}", "Starting sonic-migrate...".cyan());
        if let Some(network) = config.network {
            println!("{}", format!("Target network: {}", network).cyan());
        } else {
            println!("{}", "Using default network (testnet)".cyan());
        }
    }

    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("Failed to create progress style")
            .tick_chars("/|\\- "),
    );

    if config.restore {
        progress.set_message("Restoring from backup...");
        progress.enable_steady_tick(Duration::from_millis(100));
        match restore_backup(&config.path) {
            Ok(_) => {
                progress.finish_with_message("Backup restored successfully.".green().to_string());
                println!("{}", "Restore complete.".green());
            }
            Err(e) => {
                progress.finish_with_message("Restore failed.".red().to_string());
                eprintln!("{}", e.to_string().red());
                exit(1);
            }
        }
        return;
    }

    let network_name = config.network.map_or("testnet".to_string(), |n| n.to_string());
    progress.set_message(format!("Migrating project to Sonic {}...", network_name));
    progress.enable_steady_tick(Duration::from_millis(100));

    match run_migration(&config) {
        Ok(_) => {
            progress.finish_with_message("Migration completed successfully.".green().to_string());
            println!("{}", "Migration successful!".green());
            println!("{}", "Next steps:".yellow());
            println!("1. Update your dependencies.");
            println!("2. Test your project.");
            println!("3. Deploy to Sonic {} Network.", network_name);
            
            // Display RPC URL info
            println!("\n{}", "Network Information:".cyan());
            match config.network {
                Some(Network::MainnetAlpha) => {
                    println!("Mainnet Alpha RPC URL: {}", "https://api.mainnet-alpha.sonic.game".bright_green());
                }
                _ => {
                    println!("Testnet RPC URL: {}", "https://api.testnet.sonic.game".bright_green());
                }
            }
            
            // Add migration reminder
            println!("\n{}", "To learn more about additional networks, run:".yellow());
            println!("  sonic-migrate --networks");
        }
        Err(e) => {
            progress.finish_with_message("Migration failed.".red().to_string());
            eprintln!("{}", e.to_string().red());
            exit(1);
        }
    }
}


