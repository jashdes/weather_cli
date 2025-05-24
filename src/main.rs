use clap::Parser;

#[derive(Parser)]
struct Cli{
    city: String,
}

fn main() {
    let args = Cli::parse();
    println!("Let's get the weather for: {}", args.city);
}
