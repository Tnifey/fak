use std::collections::HashMap;

pub trait Pretty {
    fn pretty(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Output {
    pub value: String,
    pub meta: Option<HashMap<String, String>>,
}

impl Output {
    pub fn pretty(&self) -> String {
        format!("{} -> {:?}", self.value, self.meta)
    }

    pub fn value_only(value: String) -> Self {
        Self {
            value,
            meta: None,
        }
    }
}
