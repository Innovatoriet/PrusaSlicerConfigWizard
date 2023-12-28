/// Command line interface specifications
///
/// Defines the options for interacting with the cli

use clap::{Args, Parser, Subcommand};

// Global Constants
const AUTHOR: &str  = "Linus Michelsson, (Innovatoriet)";
const VERSION: &str = "0.0.1-alpha1";
const ABOUT: &str   = "Command line interface for managing prusa slicer configurations using git repositories";

/// Root CLI
///
/// Does not take any arguments and doesn't do anything itself, all functionality is handled by
/// subcommands
#[derive(Parser, Debug)]
#[command(author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    command: Commands,
}


/// Setup new configuration
#[derive(Args, Debug)]
struct Init {}


/// Update configuration settings
#[derive(Args, Debug)]
struct Update {}


/// Upgrade installed configurations
#[derive(Args, Debug)]
struct Upgrade {}


/// Manage installed configs
#[derive(Args, Debug)]
struct Configs {}


#[derive(Subcommand, Debug)]
enum Commands {
    Init(Init),
    Update(Update),
    Configs(Configs),
    Upgrade(Upgrade),
}
