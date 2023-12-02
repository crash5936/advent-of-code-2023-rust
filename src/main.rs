use clap::Parser;

mod days;

#[derive(Parser)]
struct Cli {
    day: String,
}

fn main() {
    let args = Cli::parse();

    match args.day.as_str() {
        "1" => days::day1::part1(),
        "1-1" => days::day1::part1(),
        "1-2" => days::day1::part2(),
        _ => println!("Day not implemented"),
    }
}