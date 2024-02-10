use rand::Rng;
use crate::types::Output;

#[derive(Debug, Clone)]
pub struct Input {
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Child,
    Teen,
    Adult,
    Senior,
    All,
}

pub fn generate(input: Input) -> Option<Output> {
    let r#type = match input.r#type.unwrap_or("all".to_string()).as_str() {
        "child" => Types::Child,
        "teen" => Types::Teen,
        "adult" => Types::Adult,
        "senior" => Types::Senior,
        _ => Types::All,
    };

    let value = match r#type {
        Types::Child => from_range!(0..=12),
        Types::Teen => from_range!(13..=19),
        Types::Adult => from_range!(20..=65),
        Types::Senior => from_range!(66..=100),
        _ => from_range!(0..=100),
    }.to_string();

    Some(Output::new(value))
}
