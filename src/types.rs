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
        if pretty {
            self.pretty();
            println!(" ");
        } else {
            println!("{}", self.value);
        }
    }

    pub fn pretty(&self) {
        match self.meta {
            None => println!("{}", self.value),
            Some(ref meta) => {
                meta.iter().for_each(|(k, v)| {
                    println!("{k}: {v}");
                });
            }
        }
        if self.meta.is_none() {
            return println!("{}", self.value);
        }

        let meta = self.meta.as_ref().unwrap();
        meta.iter().for_each(|(k, v)| {
            println!("{k}: {v}");
        });
    }
}
