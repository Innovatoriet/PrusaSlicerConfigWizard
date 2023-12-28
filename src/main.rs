
mod interface;
mod slicer_configs;


const PATH: &str = "/Users/06limi01/Library/Application Support/PrusaSlicer/PrusaSlicer.ini";


fn main() {
    // Read file to string
    let input = match slicer_configs::File::from_path(PATH) {
        Ok(file) => file,
        Err(msg) => panic!("Failed to parse: {}", msg),
    };

    let mut out = String::new();
    
    input.format(&mut out);

    dbg!(out);

}
