
mod interface;
use clap::Parser;


fn main() {
    interface::cli::cli::Opts::parse();
}
