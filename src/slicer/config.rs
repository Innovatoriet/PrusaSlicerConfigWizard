use std::collections::HashMap;

/// Handles parsing and writing individual PrusaSlicer configuration files
use ini_core::{Item, Parser};

/// Represents a PrusaSlicer configuration property within a section
///
#[derive(Debug, Clone)]
pub struct Property<'a> {
    pub key: &'a str,
    pub value: Option<Vec<&'a str>>,
}

/// Represents a PrusaSlicer configuration section within a file
#[derive(Debug, Clone)]
pub struct Section<'a> {
    pub name: &'a str,
    pub properties: Vec<Property<'a>>,
}

/// Represents a PrusaSlicer configuration file
/// Can be converted to a ConfigMap
#[derive(Debug, Clone)]
pub struct ConfigFile<'a> {
    pub properties: Vec<Property<'a>>,
    pub sections: Vec<Section<'a>>,
}

/// Represents a PrusaSlicer configuration file using hasmaps
/// Can be converted to a ConfigFile
#[derive(Debug, Clone)]
pub struct ConfigMap<'a> {
    pub properties: HashMap<&'a str, Option<Vec<&'a str>>>,
    pub sections: HashMap<&'a str, HashMap<&'a str, Option<Vec<&'a str>>>>,
}

impl<'a> ConfigFile<'a> {
    /// Reads a PrusaSlicer configuration file from a path
    pub fn parse(contents: &'a str) -> Result<ConfigFile, &'static str> {
        let parser = Parser::new(contents)
            // config files format their values '<key> = <value>', so wee need to trim away the
            // whitespaces
            .auto_trim(true)
            // prusa slicer config files use # for comments
            .comment_char(b'#');

        // object to parse into
        let mut file = ConfigFile {
            properties: Vec::new(),
            sections: Vec::new(),
        };

        // Flag for if we are currently in a section or if the properties being read are global
        let mut in_section = false;

        // Metadata about the current section
        let mut section = Section {
            name: "",
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
                        name,
                        properties: Vec::new(),
                    };

                    in_section = true;
                }

                Item::Property(key, value) => {
                    // get value
                    let value = match value {
                        Some(v) => Some(v.split(';').collect()),
                        None => None,
                    };

                    // Compute property
                    let property = Property { key, value };

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

    /// Converts a ConfigFile to a ConfigMap
    pub fn to_map(&self) -> ConfigMap {
        // Create new ConfigMap
        let mut map = ConfigMap {
            properties: HashMap::new(),
            sections: HashMap::new(),
        };

        // Add global properties
        for p in self.properties.iter() {
            map.properties.insert(p.key, p.value.clone());
        }

        // Add sections
        for s in self.sections.iter() {
            // Create new section hashmap
            let mut section = HashMap::new();

            // Add section properties
            for p in s.properties.iter() {
                section.insert(p.key, p.value.clone());
            }

            // Add section to map
            map.sections.insert(s.name, section);
        }

        map
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
            Some(value) => value.join(";"),
            None => String::new(),
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

impl ConfigMap<'_> {
    /// Converts a grop of ConfigMap properties to a group of ConfigFile properties
    /// Loops over each hashmap entry and converts it to a ConfigFile property by taking the has
    /// key as the property key and the value as the property value
    ///
    /// # Arguments
    /// * `properties` - Hasmap of properties to convert
    ///
    /// # Returns
    /// * `Vec<Property>` - Vector of converted properties
    ///
    /// # Example
    /// ```rust
    /// let mut map = HashMap::new();
    /// map.insert("foo", Some(vec!["bar"]));
    ///
    /// let properties = ConfigMap::properties_to_file(&map);
    ///
    /// assert_eq!(properties, vec![Property {
    ///     key: "foo",
    ///     value: Some(vec!["bar"])
    /// }]);
    /// ```
    fn properties_to_file<'a>(
        properties: &HashMap<&'a str, Option<Vec<&'a str>>>,
    ) -> Vec<Property<'a>> {
        let props = properties
            .iter()
            // convert all hasmap entries to ConfigFile properties
            .map(|(key, value)| Property {
                key,
                value: value.clone(),
            });

        props.collect()
    }

    /// Converts a ConfigMap section to a ConfigFile section
    ///
    /// This function is i shorthand for constructing a section given a name and a hasmap of
    /// properties
    ///
    /// `ConfigMap::properties_to_file` is used to convert the section properties and the section
    /// is then constructed as normal.
    ///
    /// # Arguments
    /// * `name` - Name of the section
    /// * `section` - Hasmap of the section properties
    ///
    /// # Returns
    /// * `Section` - Constructed section
    ///
    /// # Example
    /// ```rust
    /// let mut map = HashMap::new();
    /// map.insert("foo", Some(vec!["bar"]));
    ///
    /// let section = ConfigMap::section_to_file("section", &map);
    ///
    /// assert_eq!(section, Section {
    ///     name: "section",
    ///     properties: vec![Property {
    ///         key: "foo",
    ///         value: Some(vec!["bar"])
    ///     }]
    /// });
    /// ```
    fn section_to_file<'a>(
        name: &'a str,
        section: &HashMap<&'a str, Option<Vec<&'a str>>>,
    ) -> Section<'a> {
        let properties = Self::properties_to_file(section);

        Section { name, properties }
    }

    /// Converts a ConfigMap to a ConfigFile
    ///
    /// The sections are sorted alphabetically
    /// The global properties and section properties are converted using `ConfigMap::properties_to_file`
    /// and `ConfigMap::section_to_file` respectively
    ///
    /// # Returns
    /// * `ConfigFile` - Converted ConfigFile
    ///
    /// # Example
    /// ```rust
    /// let mut global = HashMap::new();
    /// global.insert("foo", Some(vec!["bar"]));
    ///
    /// let mut map = HashMap::new();
    /// map.insert("foo", Some(vec!["bar"]));
    ///
    /// let mut sections = HashMap::new();
    /// sections.insert("section", map);
    ///
    /// let map = ConfigMap {
    ///     properties: global,
    ///     sections
    /// };
    ///
    /// let file = map.to_file();
    ///
    /// assert_eq!(file, ConfigFile {
    ///     properties: vec![Property {
    ///         key: "foo",
    ///         value: Some(vec!["bar"])
    ///     }],
    ///     sections: vec![Section {
    ///         name: "section",
    ///         properties: vec![Property {
    ///             key: "foo",
    ///             value: Some(vec!["bar"])
    ///         }]
    ///     }]
    /// });
    /// ```
    pub fn to_file(&self) -> ConfigFile {
        // Create new ConfigFile
        let mut file = ConfigFile {
            properties: Vec::new(),
            sections: Vec::new(),
        };

        // Add global properties
        file.properties = Self::properties_to_file(&self.properties);

        // Add sections
        file.sections = self.sections
            .iter()
            .map(|(name, props)| Self::section_to_file(name, props))
            .collect();

        // Sort sections alphabetically
        file.sections.sort_by(|a, b| a.name.cmp(&b.name));

        file
    }
}
