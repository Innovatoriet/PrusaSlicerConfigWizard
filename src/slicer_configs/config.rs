/// Handles parsing and writing individual PrusaSlicer configuration files
use ini_core::{Item, Parser};
use std::fs::read_to_string;

/// Represents a PrusaSlicer configuration property within a section
#[derive(Debug, Clone)]
pub struct Property {
    key: String,
    value: Option<String>,
}

/// Represents a PrusaSlicer configuration section within a file
#[derive(Debug, Clone)]
pub struct Section {
    name: String,
    properties: Vec<Property>,
}

/// Represents a PrusaSlicer configuration file
#[derive(Debug, Clone)]
pub struct File {
    path: String,
    properties: Vec<Property>,
    sections: Vec<Section>,
}

impl File {
    /// Reads a PrusaSlicer configuration file from a path
    pub fn from_path(path: &str) -> Result<Self, &'static str> {
        // this can probably be done in a better way but i happen to be lazy today
        let contents = {
            match read_to_string(path) {
                Ok(t) => t,
                Err(_) => return Err("Failed to read file"),
            }
        };

        let parser = Parser::new(contents.as_str())
            // config files format their values '<key> = <value>', so wee need to trim away the
            // whitespaces
            .auto_trim(true)
            // prusa slicer config files use # for comments
            .comment_char(b'#');

        // object to parse into
        let mut file = File {
            path: path.to_string(),
            properties: Vec::new(),
            sections: Vec::new(),
        };

        // Flag for if we are currently in a section or if the properties being read are global
        let mut in_section = false;

        // Metadata about the current section
        let mut section = Section {
            name: String::new(),
            properties: Vec::new(),
        };

        // Parse parsed lines
        parser.for_each(|line| {
            match line {
                Item::SectionEnd => {
                    // Push saved section and reset
                    match in_section {
                        true => file.sections.push(section.clone()),
                        _ => (),
                    }

                    in_section = false;
                }

                Item::Section(name) => {
                    // Start off new section
                    section = Section {
                        name: name.to_string(),
                        properties: Vec::new(),
                    };

                    in_section = true;
                }

                Item::Property(key, value) => {
                    // get value
                    let value = match value {
                        Some(v) if v.is_empty() => None,
                        Some(v) => Some(v.to_string()),
                        None => None,
                    };

                    // Compute property
                    let property = Property {
                        key: key.to_string(),
                        value,
                    };

                    // Push to global or section properties depending on flag
                    match in_section {
                        true => &mut section.properties,
                        false => &mut file.properties,
                    }
                    .push(property);
                }

                _ => {}
            };
        });

        dbg!(&file);
        Ok(file)
    }
}
