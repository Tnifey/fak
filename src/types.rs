#[derive(Debug, Clone)]
pub struct Output {
    pub value: String,
    pub meta: Option<Vec<(String, String)>>,
}

impl Output {
    pub fn new(value: String) -> Self {
        Self { value, meta: None }
    }

    pub fn meta(value: &str, meta: Vec<(&str, &str)>) -> Self {
        Self {
            value: value.to_string(),
            meta: Some(
                meta.iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            ),
        }
    }

    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn none(self) -> Option<Self> {
        None
    }

    pub fn print(&self, pretty: bool) {
        match (pretty, self.meta.is_some()) {
            (true, true) => self.pretty(),
            (_, _) => println!("{}", self.value),
        }
    }

    fn pretty(&self) {
        let meta = self.meta.as_ref().unwrap();
        meta.iter().for_each(|(k, v)| println!("{k}: {v}"));
    }
}
