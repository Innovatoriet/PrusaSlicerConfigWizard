use std::{collections::HashMap, fs::read_to_string};

mod interface;
mod slicer_configs;

const PATH: &str = "";

// Make sure the given given printer exists and it has the given nozzle
fn has_printer_and_nozzle<'a>(
    printers: &mut HashMap<&'a str, Option<Vec<&'a str>>>,
    name: &'a str,
    nozzle: &'a str,
) {
    // Check if the printer exists, if it doesn't add it with the given nozzle and return
    let printer = printers.entry(name).or_insert(None);

    // check if the printer has nozzles
    if let Some(nozzles) = printer {
        // Check if it has the given nozzle, if it doesn't add it and return
        if !nozzles.contains(&nozzle) {
            nozzles.push(nozzle);
        }
        return;
    }

    // If the printer has no nozzles, add the given nozzle
    printers.insert(name, Some(vec![nozzle]));
}

fn main() {
    let contents = read_to_string(PATH).expect("Something went wrong reading the file");

    let config = slicer_configs::ConfigFile::parse(&contents).expect("Failed to parse config");
    let mut map = config.to_map();
    let prusa_vendor = map
        .sections
        .entry("vendor:PrusaResearch")
        .or_insert(HashMap::new());

    has_printer_and_nozzle(prusa_vendor, "model:MK4IS", "0.2");
    has_printer_and_nozzle(prusa_vendor, "model:MK3S", "0.2");

    dbg!(map.to_file());
}
