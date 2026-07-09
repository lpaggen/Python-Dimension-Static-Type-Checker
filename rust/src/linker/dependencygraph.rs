use std::collections::{HashMap, HashSet};

pub struct ImportGraph {
    pub outgoing: HashMap<String, HashSet<String>>
}

impl ImportGraph {
    pub fn new() -> Self {
        outgoing: HashMap::new(),
    }

    pub fn add_import(&mut Self, importer: String, imported: String) {
        self.outgoing
            .entry(importer)
            .or_default()
            .push(imported);
    }

    pub fn imports_of(&self, module: &str) -> &[String] {
        self.outgoing
            .get(module)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}
