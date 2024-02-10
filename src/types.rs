#[derive(Debug, Clone)]
pub struct Output {
    pub value: String,
    pub meta: Option<Vec<(String, String)>>,
}

impl Output {
    pub fn new(value: String) -> Self {
        Self {
            value,
            meta: None,
        }
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
