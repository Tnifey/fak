use rand::Rng;
use crate::types::Output;

#[derive(Debug, Clone)]
pub struct Input {}

pub fn generate(input: Option<Input>) -> Option<Output> {
    let input = input.unwrap_or(Input {});
    Some(Output { })
}
