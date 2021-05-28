mod event;

#[derive(Clone, Debug)]
pub struct XTParser {
    events: Vec<event::Event>,
    cpu_count: u8,
}

impl XTParser {
    pub fn from_file(path: &str) -> Self {
        Self {
            events: Vec::new(),
            cpu_count: 0,
        }
    }
}