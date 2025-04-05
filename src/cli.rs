use clap::{Arg, ArgAction, Command, value_parser};
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Network {
    TestNet,
    MainnetAlpha,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::TestNet => write!(f, "testnet"),
            Network::MainnetAlpha => write!(f, "mainnet-alpha"),
        }
    }
}

impl std::str::FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "testnet" => Ok(Network::TestNet),
            "mainnet-alpha" => Ok(Network::MainnetAlpha),
            _ => Err(format!("Unknown network: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub dry_run: bool,
    pub verbose: bool,
    pub restore: bool,
    pub network: Option<Network>,
    pub list_networks: bool,
}

pub fn print_networks_info() {
    println!("{}", "Available Sonic Networks:".cyan().bold());
    println!("\n{}", "Testnet".yellow());
    println!("RPC URL: {}", "https://api.testnet.sonic.game".bright_green());
    println!("Usage: {} sonic-migrate --network testnet", "Example:".italic());

    println!("\n{}", "Mainnet Alpha".yellow());
    println!("RPC URL: {}", "https://api.mainnet-alpha.sonic.game".bright_green());
    println!("Usage: {} sonic-migrate --network mainnet-alpha", "Example:".italic());
}

impl Config {
    pub fn new() -> Self {
        let matches = Command::new("sonic-migrate")
            .version("0.1.3")
            .author("Shivam Soni <shivamssoni6@gmail.com>")
            .about("Migrates Solana Anchor projects to Sonic Network")
            .arg(
                Arg::new("path")
                    .help("Path to the Anchor project")
                    .default_value(".")
                    .index(1),
            )
            .arg(
                Arg::new("dry-run")
                    .long("dry-run")
                    .help("Show changes without applying them")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .help("Enable detailed logging")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("restore")
                    .long("restore")
                    .help("Restore from backup")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("network")
                    .long("network")
                    .short('n')
                    .help("Target Sonic network (testnet, mainnet-alpha)")
                    .value_parser(value_parser!(Network)),
            )
            .arg(
                Arg::new("networks")
                    .long("networks")
                    .help("List available networks and their RPC URLs")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        let list_networks = matches.get_flag("networks");
        if list_networks {
            print_networks_info();
            std::process::exit(0);
        }

        Config {
            path: matches.get_one::<String>("path").unwrap().to_string(),
            dry_run: matches.get_flag("dry-run"),
            verbose: matches.get_flag("verbose"),
            restore: matches.get_flag("restore"),
            network: matches.get_one::<Network>("network").cloned(),
            list_networks,
        }
    }
}