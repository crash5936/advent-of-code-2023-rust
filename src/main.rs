use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: String,
}

fn main() {
    let args = Cli::parse();

    match args.day.as_str() {
        "1" => day1::part1(),
        "1-1" => day1::part1(),
        "1-2" => day1::part2(),
        _ => println!("Day not implemented"),
    }
}

mod day1 {
    use regex::Regex;

    pub fn part1() {
        let stdin = std::io::stdin();
        let mut total_calibration: i32 = 0;
        for line in stdin.lines() {
            let line = line.expect("Can't read line");
            let mut calibration_string = String::from("");

            for char in line.chars() {
                if char.is_numeric() {
                    calibration_string.push(char);
                    break;
                }
            }

            for char in line.chars().rev() {
                if char.is_numeric() {
                    calibration_string.push(char);
                    break;
                }
            }

            let calibration = calibration_string.parse::<i32>().unwrap();

            total_calibration += calibration;
        }

        println!("{}", total_calibration)
    }

    pub fn part2() {
        let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();

        let mut total_calibration = 0;

        let stdin = std::io::stdin();
        for line in stdin.lines() {
            let line = line.expect("Can't read line");

            let caps = re.captures(&line).unwrap();

            println!("{:#?}", caps);

            let first_digit: &str = caps.get(1).unwrap().as_str();
            let second_digit: &str = caps.get(caps.len() - 1).unwrap().as_str();

            let first_char = digit_str_to_char(first_digit);
            let second_char = digit_str_to_char(second_digit);

            println!("line: {}", line);
            println!("{} {}", first_digit, second_digit);

            let mut calibration_string = String::from("");
            calibration_string.push(first_char);
            calibration_string.push(second_char);

            let calibration = calibration_string.parse::<i32>().unwrap();

            println!("calibration: {}", calibration);

            total_calibration += calibration;
        }

        println!("{}", total_calibration)
    }

    fn digit_str_to_char(digit: &str) -> char {
        let digit_str = match digit {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => digit,
        };

        let chars: Vec<char> = String::from(digit_str).chars().collect();

        *chars.get(0).unwrap()
    }
}
