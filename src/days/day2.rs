use std::collections::HashMap;

pub fn part1() {
    let stdin = std::io::stdin();

    let maximums: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut game_sum = 0;

    for line in stdin.lines() {
        let line = line.expect("Can't read line");
        let mut game_possible = true;
        let mut game_and_data = line.split(": ");
        let game = game_and_data.nth(0).unwrap();
        let data = game_and_data.nth(0).unwrap();
        let game_nr_str = game.split(" ").nth(1).unwrap();

        let game_nr = game_nr_str.parse::<i32>().unwrap();

        let lots = data.split("; ");

        'outer: for lot in lots {
            for draw in lot.split(", ") {
                let mut nr_and_color = draw.split(" ");
                let nr = nr_and_color.nth(0).unwrap().parse::<i32>().unwrap();
                let color = nr_and_color.nth(0).unwrap();

                if maximums.get(color).unwrap() < &nr {
                    game_possible = false;
                    break 'outer
                }
            }
        }

        if game_possible {
            game_sum += game_nr;
        }
    }

    println!("Possible game nrs sum: {}", game_sum);
}


pub fn part2() {
    let stdin = std::io::stdin();

    let mut total_power = 0;

    for line in stdin.lines() {
        let mut minimums = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let line = line.expect("Can't read line");
        let mut game_and_data = line.split(": ");
        let data = game_and_data.nth(1).unwrap();

        let lots = data.split("; ");

        for lot in lots {
            for draw in lot.split(", ") {
                let mut nr_and_color = draw.split(" ");
                let nr = nr_and_color.nth(0).unwrap().parse::<i32>().unwrap();
                let color = nr_and_color.nth(0).unwrap();

                if minimums.get(color).unwrap() < &nr {
                    minimums.insert(color, nr);
                }
            }
        }

        let power = minimums.get("red").unwrap() * minimums.get("blue").unwrap() * minimums.get("green").unwrap();

        total_power += power;
    }
    println!("Total power: {}", total_power);
}