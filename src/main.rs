use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: i8,
}


fn main() {
    let args = Cli::parse();

    match args.day {
        1 => day1::main(),
        _ => println!("Day not implemented")
    }
}


mod day1 {
    pub fn main() {
        let stdin = std::io::stdin();
        let mut total_calibration: i32 = 0;
        for line in stdin.lines() {
            let line = line.expect("Can't read line");
            // println!("{}", line)
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
}