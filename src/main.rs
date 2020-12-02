fn read_a_file(filename: &str) -> std::io::Result<Vec<String>> {
  use std::fs::File;
  use std::io::{self, BufRead, BufReader};
  let file = File::open(filename)?;

  let file_reader = BufReader::new(file);
  Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn day1() {
    let lines = read_a_file("./day1_expenses.txt").unwrap().iter()
        .map(move |expense| expense.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for expense1 in &lines {
        for expense2 in &lines {
            if expense1 != expense2 && expense1 + expense2 == 2020 {
                println!("Day 1 Part 1: Found these matches, {} + {} == 2020, multiplied they are {}\n", expense1, expense2, expense1 * expense2);
            }
            for expense3 in &lines {
                if expense1 != expense2 && expense2 != expense3 && expense3 != expense1 && expense1 + expense2 + expense3 == 2020 {
                    println!("Day 1 Part 2: Found these matches, {} + {} + {} == 2020, multiplied they are {}\n", expense1, expense2, expense3, expense1 * expense2 * expense3);
                }
            }
        }
    }
}

fn day2() {
    use std::str::FromStr;
    use std::convert::TryInto;

    struct PolicyAndPassword {
        min_occurrences: u32,
        max_occurrences: u32,
        letter: char,
        password: String
    }

    impl PolicyAndPassword {
        fn is_valid(&self) -> bool {
            let num_occurrences = self.password.chars().filter(|c| *c == self.letter).count();
            num_occurrences >= self.min_occurrences.try_into().unwrap() && num_occurrences <= self.max_occurrences.try_into().unwrap()
        }
    }
    impl FromStr for PolicyAndPassword {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use regex::Regex;
            let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
            let caps = re.captures(s).unwrap();

            Ok(PolicyAndPassword {
                min_occurrences: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                max_occurrences: caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                password: caps.get(4).unwrap().as_str().to_string()
            })
        }
    }

    let num_valid = read_a_file("./day2_passwords.txt").unwrap().iter()
        .map(move |line| PolicyAndPassword::from_str(line).unwrap())
        .filter(move |policy_and_password| policy_and_password.is_valid())
        .count();
    print!("Day 2, there are {} valid passwords.\n", num_valid);
}

fn main() {
    day2();
}
