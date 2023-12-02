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

    let mut total_calibration = 0;

    let stdin = std::io::stdin();
    for line in stdin.lines() {
        let line = line.expect("Can't read line");
        let digits = find_digits(line);
        total_calibration += digits;
    }

    println!("Total Calibration: {}", total_calibration);
}

fn find_digits(record: String) -> i32 {
    // Get a line (called record) and return a combination of the first and the
    // last digit as an _i32_

    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let mut calibration_string = String::from("");        
    let mut first_digit = "";
    let mut last_digit = "";
    let mut min: usize = usize::MAX;
    let mut max: usize = usize::MIN;

    for digit in digits {
        let first = record.find(digit);
        let last = record.rfind(digit);


        match first {
            Some(val) => if val <= min { min = val; first_digit = digit }, 
            None => (),
        }

        match last {
            Some(val) => if val >= max { max = val; last_digit = digit }
            None => (),
        }

    }

    calibration_string.push(digit_str_to_char(first_digit));
    calibration_string.push(digit_str_to_char(last_digit));

    calibration_string.parse::<i32>().unwrap()
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