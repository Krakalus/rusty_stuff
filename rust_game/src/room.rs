use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<String>,
    pub exits: HashMap<String, String>,
    pub locked_exits: Vec<String>,
    pub generated: bool,        // ← NEW: has this room been created yet?
}

impl Room {
    pub fn new(name: &str, desc: &str) -> Self {
        Room {
            name: name.to_string(),
            description: desc.to_string(),
            items: Vec::new(),
            exits: HashMap::new(),
            locked_exits: Vec::new(),
            generated: false,
        }
    }
}