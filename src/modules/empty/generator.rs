use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {}

#[derive(Debug, Clone)]
pub struct Output {}

pub fn generate(input: Option<Input>) -> Option<Output> {
    let input = input.unwrap_or(Input {});
    Some(Output { })
}
