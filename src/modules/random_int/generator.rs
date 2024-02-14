use rand::Rng;
use crate::types::Output;

#[derive(Debug, Clone)]
pub struct Input {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

pub fn generate(input: Input) -> Output {
    let min = input.min.unwrap_or(0);
    let max = input.max.unwrap_or(1);
    Output::new(format!("{}", from_range!(min..=max)))
}
