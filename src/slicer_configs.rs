/// Handles parsing and writing individual PrusaSlicer configuration files
use ini_core::{Item, Parser};
use std::fs::read_to_string;

/// Represents a PrusaSlicer configuration property within a section
#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: Option<String>,
}

/// Represents a PrusaSlicer configuration section within a file
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub properties: Vec<Property>,
}

/// Represents a PrusaSlicer configuration file
#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub properties: Vec<Property>,
    pub sections: Vec<Section>,
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

        Ok(file)
    }

    /// Add a property to a string formatted as follows
    /// `<key> = <value>\n`
    /// Where key and value are strings.
    /// value defaults to "" if it is None on the given property
    ///
    /// # Arguments
    /// * `str` - String to add property to
    /// * `prop` - Property to add
    ///
    /// # Example
    /// ```rust
    /// let mut str = String::new();
    ///
    /// add_property(&mut str,
    ///     &Property {
    ///         key: "foo".to_string(),
    ///         value: Some("boo".to_string())
    ///     });
    ///
    /// assert_eq!(str, "foo = boo\n");
    /// ```
    fn add_property(str: &mut String, prop: &Property) {
        // convert String value to str or default to ""
        let value = match &prop.value {
            Some(value) => value.as_str(),
            None => "",
        };

        // Format using '<key> = <value>'
        let formated = format!("{} = {}\n", prop.key, value);
        str.push_str(&formated);
    }

    /// Add a section to a string formatted as follows
    /// `[<name>]\n`
    /// Where name is a string
    /// 
    /// # Arguments
    /// * `str` - String to add section to
    /// * `section` - Section to add
    ///
    /// # Example
    /// ```rust
    /// let mut str = String::new();
    /// 
    /// add_section(&mut str,
    ///     &Section {
    ///         name: "foo".to_string(),
    ///         properties: Vec::new()
    ///     });
    ///
    /// assert_eq!(str, "[foo]\n");
    /// ```
    fn add_section(str: &mut String, section: &Section) {
        // Format using '[<name>]'
        let formated = format!("\n[{}]\n", section.name);

        str.push_str(&formated);

        // add section properties
        for p in section.properties.iter() {
            Self::add_property(str, &p);
        }
    }

    /// Format a PrusaSlicer configuration file
    /// 
    /// # Arguments
    /// * `out` - String to append formating to
    ///
    /// # Example
    /// ```rust
    /// let mut str = String::new();
    /// 
    /// let input = File {
    ///     path: "foo.ini".to_string(),
    ///     properties: Vec::new(),
    ///     sections: Vec::new()
    /// };
    /// 
    /// input.format(&mut str);
    /// 
    /// assert_eq!(str, "");
    /// ```
      
    pub fn format(&self, out: &mut String) {
        // add global properties
        for p in self.properties.iter() {
            Self::add_property(out, &p);
        }

        // add sections
        for s in self.sections.iter() {
            Self::add_section(out, &s)
        }
    }
}
