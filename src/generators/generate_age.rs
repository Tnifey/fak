use rand::Rng;

#[derive(Debug, Clone)]
pub struct AgeInput {
    pub r#type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AgeResult {
    pub age: u8,
}

pub enum Ages {
    Child,
    Teen,
    Adult,
    Senior,
    All,
}

pub fn generate_age(input: Option<AgeInput>) -> Option<AgeResult> {
    let input = input.unwrap_or(AgeInput {
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

    let age = match r#type {
        Ages::Child => rand::thread_rng().gen_range(0..=12),
        Ages::Teen => rand::thread_rng().gen_range(13..=19),
        Ages::Adult => rand::thread_rng().gen_range(20..=65),
        Ages::Senior => rand::thread_rng().gen_range(66..=100),
        Ages::All => rand::thread_rng().gen_range(0..=100),
    };

    Some(AgeResult { age })
}
