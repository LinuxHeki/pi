use clap::Parser;
use pi::Pi;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// How many digits will be calculated, including first digit "3"
    digits: u32,
    /// Precision used for calulating pi. If too small it won't output right digits. If not set it will use 3.4 times the number of digits
    precision: Option<u32>,
}

fn main()  {
    let args = Args::parse();
    let mut pi = Pi::new(args.precision.unwrap_or_else(|| auto_precision(args.digits)));
    
    print!("{}.", pi.next_digit());
    for _ in 1..args.digits {
        print!("{}", pi.next_digit());
    }
}

fn auto_precision(digits: u32) -> u32 {
    (digits as f32 * 3.4) as u32
}