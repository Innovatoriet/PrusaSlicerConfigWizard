use std::fs::read_to_string;

mod interface;
mod slicer_configs;


const PATH: &str = "";

fn main() {
    let contents = read_to_string(PATH).expect("Something went wrong reading the file");

    let config = slicer_configs::Config::parse(&contents).expect("Failed to parse config");

    let mut out = String::new();
    config.format(&mut out);
    println!("{}", out);
}
