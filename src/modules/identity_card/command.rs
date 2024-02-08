#[derive(Debug, Clone, Parse)]
pub struct Arguments {
    #[arg(short, long)]
    pub count: Option<u32>,
}

pub fn handle(args: Arguments) {
   let Arguments { count } = args;

   for _ in 0..count.unwrap_or(1) {
       let result = super::generator::generate(super::generator::Input {});
       if let Some(result) = result {
           if pretty.unwrap_or(false) {
               println!("{}", super::generator::format_pretty(result));
           } else {
               println!("{}", result.value);
           }
       }
   }
}