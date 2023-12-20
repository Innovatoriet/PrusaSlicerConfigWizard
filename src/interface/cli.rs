
pub mod cli {
    use clap::{Args, Parser, Subcommand};

    const AUTHOR: &str  = "Linus Michelsson, (Innovatoriet)";
    const VERSION: &str = "0.0.1-alpha1";
    const ABOUT: &str   = "Command line interface for managing prusa slicer configurations using git repositories";

    #[derive(Parser, Debug)]
    #[command(author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
    pub struct Opts {
        #[command(subcommand)]
        command: Commands,
    }


    #[derive(Args, Debug)]
    struct Init {}


    #[derive(Args, Debug)]
    struct Update {}


    #[derive(Subcommand, Debug)]
    enum Commands {
        Init(Init),
        Update(Update),
    }
}
