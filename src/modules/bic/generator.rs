use crate::{
    modules::iban::utils::rand_alpha,
    types::Output,
    utils::{rand_alphanumeric, rand_iso3166},
};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {
    pub branch: Option<bool>,
}

pub fn generate(input: Input) -> Output {
    let include_branch_code = input.branch.unwrap_or(probability!());

    let bank_identifier = (0..4)
        .map(|_| rand_alpha().to_uppercase())
        .collect::<String>();
    let country_code = rand_iso3166();
    let location_code = rand_alphanumeric(2).to_uppercase();
    let branch_code = match include_branch_code {
        true => match probability!(0.5) {
            true => rand_alphanumeric(3).to_uppercase(),
            false => "XXX".into(),
        },
        false => "".into(),
    };

    let value = format!("{bank_identifier}{country_code}{location_code}{branch_code}");

    Output::new(value)
}

pub fn format_pretty(output: &Output) -> String {
    let bic = output.value.as_str();
    let iter = output.value.chars();

    let bank_identifier = iter.clone().take(4).collect::<String>();
    let country_code = iter.clone().skip(4).take(2).collect::<String>();
    let location_code = iter.clone().skip(6).take(2).collect::<String>();
    let branch_code = iter.clone().skip(8).collect::<String>();

    [
        format!("BIC:                {bic}"),
        format!("Bank identifier:    {bank_identifier}"),
        format!("Country code:       {country_code}"),
        format!("Location:           {location_code}"),
        format!("Branch code:        {branch_code}"),
    ]
    .join("\n")
}
