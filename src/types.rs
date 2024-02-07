use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Output {
    pub value: String,
    pub meta: Option<HashMap<String, String>>,
}

impl Output {
    pub fn new(value: String) -> Self {
        Self {
            value,
            meta: None,
        }
    }
}
