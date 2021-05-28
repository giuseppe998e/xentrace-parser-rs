use std::collections::HashMap;

mod domain;
mod event;

#[derive(Clone, Debug)]
pub struct XTParser {
    // Host CPUs fiels
    cpu_current: u8,
    cpu_count: u8,
    // Virt. domain fields
    dom_hmap: HashMap<u16, domain::Domain>,
    // Events fiels
    events: Vec<event::Event>,
}

impl XTParser {
    pub fn from_file(path: &str) -> Self {
        Self {
            events: Vec::new(),
            cpu_count: 0,
        }
    }
}
