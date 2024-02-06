use rand::Rng;
use crate::types::Output;

#[derive(Debug, Clone)]
pub struct Input {
    pub r#type: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Ages {
    Child,
    Teen,
    Adult,
    Senior,
    All,
}

pub fn generate(input: Option<Input>) -> Option<Output> {
    let input = input.unwrap_or(Input {
        r#type: None,
    });

    let r#type = match input.r#type {
        Some(r#type) => match r#type.to_lowercase().as_str() {
            "child" => Ages::Child,
            "teen" => Ages::Teen,
            "adult" => Ages::Adult,
            "senior" => Ages::Senior,
            _ => Ages::All,
        },
        _ => Ages::All,
    };

    let value = match r#type {
        Ages::Child => from_range!(0..=12),
        Ages::Teen => from_range!(13..=19),
        Ages::Adult => from_range!(20..=65),
        Ages::Senior => from_range!(66..=100),
        _ => from_range!(0..=100),
    }.to_string();

    Some(Output::value_only(value))
}
