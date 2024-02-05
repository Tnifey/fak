use chrono::Datelike;
use clap::{ Parser, Subcommand };
use rand::Rng;

#[derive(Debug, Parser)]
#[command(version, about)]
struct App {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug, Subcommand)]
#[command(about)]
enum Commands {
    Pesel {
        #[arg(short, long)]
        count: Option<u8>,

        #[arg(short, long)]
        age: Option<u8>
    },
    Email {
        #[arg(short, long)]
        count: Option<u8>,

        #[arg(short, long)]
        pre: Option<String>
    }
}

fn main() {
    let app = App::parse();

    // println!("App: {:?}", app);

    if let Some(name) = app.command {
        match name {
            Commands::Pesel { count, age } => {
                let count = count.unwrap_or(10);
                let _age: u8 = age.unwrap_or(random_age());
                for _ in 0..count {
                    generate_pesel();
                    println!()
                }
            },
            Commands::Email { count, pre } => {
                let count = count.unwrap_or(10);
                println!();
                if let Some(pre) = pre {
                    for _ in 0..count {
                        println!("{}", generate_pre_email(&pre));
                    }
                } else {
                    for _ in 0..count {
                        println!("{}", generate_email());
                    }
                }
                println!();
            }
        }
    }
}

fn generate_pre_email(email: &str) -> String {
    let [first, last] = match email.split_once('@') {
        Some((first, last)) => [first, last],
        None => panic!("Invalid email")
    };

    let int = rand::random::<u32>();
    format!("{}+{}@{}", first, int, last)
}

fn generate_email() -> String {
    let mut rng = rand::thread_rng();
    let email: String = (0..10).map(|_| rng.gen_range(0..10).to_string()).collect();

    format!("xx@xxxxxxx.xxx {}", email)
}

fn generate_pesel() {
    let mut rng = rand::thread_rng();

    let date = chrono::NaiveDate::from_ymd_opt(rand::thread_rng().gen_range(1900..2024), rand::thread_rng().gen_range(1..12), rand::thread_rng().gen_range(1..28));
    let date = match date {
        Some(date) => date,
        None => panic!("Invalid date")
    };

    let y = date.year();
    let m = date.month0() + 1;
    let d = date.day();

    let is_genz = y >= 2000;

    let year = if is_genz { y - 2000 } else { y - 1900 };
    let year = match year {
        0 => "00".into(),
        1..=9 => format!("0{}", year),
        _ => year.to_string()
    };

    let month = if is_genz { m + 20 } else { m };
    let month = match month {
        0..=9 => format!("0{}", month),
        _ => month.to_string()
    };

    let day = if d < 10 { format!("0{}", d) } else { d.to_string() };

    let ran = rng.gen_range(100..999);
    let sex = rng.gen_range(0..9);
    let parts = format!("{}{}{}{}{}", year, month, day, ran, sex);
    let controllist = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];
    let mut sum = 0;
    for (i, c) in parts.chars().enumerate() {
        let c = c.to_digit(10).unwrap();
        sum += c * controllist[i];
    }

    let control = match sum % 10 {
        0 => 0,
        sum => 10 - (sum % 10)
    };

    println!();
    println!("pesel {}{}", parts, control);
    println!("date  {}", date.format("%Y-%m-%d"));
    println!("sex   {}", if sex % 2 == 0 { "F" } else { "M" });
}

fn random_age() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(14..48)
}
