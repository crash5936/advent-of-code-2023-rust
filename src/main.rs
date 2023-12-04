use clap::Parser;

mod days;

#[derive(Parser)]
struct Cli {
    day: String,
}

fn main() {
    let args = Cli::parse();

    match args.day.as_str() {
        "1" | "1-1" => days::day1::part1(),
        "1-2" => days::day1::part2(),
        "2" | "2-1" => days::day2::part1(),
        "2-2" => days::day2::part2(),
        "3" | "3-1" => days::day3::part1(),
        _ => println!("Day not implemented"),
    }
}