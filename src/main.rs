
mod interface;
use clap::Parser;


fn main() {
    interface::cli::Opts::parse();
}
