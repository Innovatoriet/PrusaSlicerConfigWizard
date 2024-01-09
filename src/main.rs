use std::{collections::HashMap, fs::read_to_string};

mod interface;
mod slicer_configs;
mod utils;
mod config;

/// Adds a printer with the given nozzle to the given HashMap
/// If the printer already exists, it checks if the nozzle exists
/// and if it doesn't it adds it
///
/// # Arguments
///
/// * `printers` - A mutable reference to a HashMap that stores the printers and nozzles
/// * `name` - The name of the printer
/// * `nozzle` - The nozzle of the printer
///
/// # Example
///
/// ```rust
/// let mut printers = HashMap::new();
///
/// has_printer_and_nozzle(&mut printers, "model:MK3S", "0.4");
///
/// assert_eq!(printers.get("model:MK3S"), Some(&Some(vec!["0.4"])));
/// ```
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

    let config = config::load().unwrap();

    dbg!(config.data);
}
