fn read_a_file(filename: &str) -> std::io::Result<Vec<String>> {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    let file = File::open(filename)?;

    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn day1() {
    let lines = read_a_file("./day1_expenses.txt")
        .unwrap()
        .iter()
        .map(move |expense| expense.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for expense1 in &lines {
        for expense2 in &lines {
            if expense1 != expense2 && expense1 + expense2 == 2020 {
                println!(
                    "Day 1 Part 1: Found these matches, {} + {} == 2020, multiplied they are {}\n",
                    expense1,
                    expense2,
                    expense1 * expense2
                );
            }
            for expense3 in &lines {
                if expense1 != expense2
                    && expense2 != expense3
                    && expense3 != expense1
                    && expense1 + expense2 + expense3 == 2020
                {
                    println!("Day 1 Part 2: Found these matches, {} + {} + {} == 2020, multiplied they are {}\n", expense1, expense2, expense3, expense1 * expense2 * expense3);
                }
            }
        }
    }
}

fn day2() {
    use std::convert::TryInto;
    use std::str::FromStr;

    struct PolicyAndPassword {
        min_occurrences: u32,
        max_occurrences: u32,
        letter: char,
        password: String,
    }

    impl PolicyAndPassword {
        fn is_valid(&self) -> bool {
            let num_occurrences = self.password.chars().filter(|c| *c == self.letter).count();
            num_occurrences >= self.min_occurrences.try_into().unwrap()
                && num_occurrences <= self.max_occurrences.try_into().unwrap()
        }

        fn is_valid_part2(&self) -> bool {
            let valid_position1 = self.min_occurrences - 1;
            let valid_position2 = self.max_occurrences - 1;
            (self
                .password
                .chars()
                .nth(valid_position1.try_into().unwrap())
                .unwrap()
                == self.letter)
                != (self
                    .password
                    .chars()
                    .nth(
                        valid_position2
                            .try_into()
                            .expect("Couldn't convert valid_position2"),
                    )
                    .unwrap()
                    == self.letter)
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
                password: caps.get(4).unwrap().as_str().to_string(),
            })
        }
    }

    let num_valid = read_a_file("./day2_passwords.txt")
        .unwrap()
        .iter()
        .map(move |line| PolicyAndPassword::from_str(line).unwrap())
        .filter(move |policy_and_password| policy_and_password.is_valid())
        .count();
    print!("Day 2 part 1: There are {} valid passwords.\n", num_valid);

    let num_valid = read_a_file("./day2_passwords.txt")
        .unwrap()
        .iter()
        .map(move |line| PolicyAndPassword::from_str(line).unwrap())
        .filter(move |policy_and_password| policy_and_password.is_valid_part2())
        .count();
    print!("Day 2 part 2: There are {} valid passwords.\n", num_valid);
}

fn day3() {
    let lines = read_a_file("./day3_trees.txt").unwrap();

    fn count_trees(lines: &Vec<String>, horizontal_speed: usize, vertical_speed: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while y < lines.len() {
            let line = &lines[y];
            if line.chars().nth(x % line.chars().count()).unwrap() == '#' {
                trees += 1;
            }
            x += horizontal_speed;
            y += vertical_speed;
        }
        trees
    }

    print!(
        "Day 3, part 1: We encounter {} trees\n",
        count_trees(&lines, 3, 1)
    );
    print!(
        "Day 3, part 2: Multiplied trees: {}\n",
        count_trees(&lines, 1, 1)
            * count_trees(&lines, 3, 1)
            * count_trees(&lines, 5, 1)
            * count_trees(&lines, 7, 1)
            * count_trees(&lines, 1, 2)
    );
}

fn day4() {
    use std::collections::HashMap;

  struct Passport {
      birth_year: String,
      issue_year: String,
      expiration_year: String,
      height: String,
      hair_color: String,
      eye_color: String,
      passport_id: String,
      country_id: String
  }

  impl Passport {
      fn is_valid(&self) -> bool {
          !self.birth_year.is_empty() &&
              !self.issue_year.is_empty() &&
              !self.expiration_year.is_empty() &&
              !self.height.is_empty() &&
              !self.hair_color.is_empty() &&
              !self.eye_color.is_empty() &&
              !self.passport_id.is_empty()
      }

      fn parse(lines: &[String]) -> Passport {
        let attributes : HashMap<&str, &str> = lines.iter().flat_map(|line| line.split(" "))
            .map(|attr| attr.split(":").collect::<Vec<&str>>())
            .map(|parts| (parts[0], parts[1]))
            .collect();
        
        Passport {
            birth_year: attributes.get("byr").unwrap_or(&"").to_string(),
            issue_year: attributes.get("iyr").unwrap_or(&"").to_string(),
            expiration_year: attributes.get("eyr").unwrap_or(&"").to_string(),
            height: attributes.get("hgt").unwrap_or(&"").to_string(),
            hair_color: attributes.get("hcl").unwrap_or(&"").to_string(),
            eye_color: attributes.get("ecl").unwrap_or(&"").to_string(),
            passport_id: attributes.get("pid").unwrap_or(&"").to_string(),
            country_id: attributes.get("cid").unwrap_or(&"").to_string()
        }
      }
  }

  let lines = read_a_file("./day4_passports.txt").unwrap(); // I manually added a blank line to this.
  let mut current_window_start = 0; // where the current passport begins
  let mut num_valid = 0;
  for (index, line) in lines.iter().enumerate() {
      if line.is_empty() {
          let passport = Passport::parse(&lines[current_window_start..index]);
          if passport.is_valid() {
            num_valid += 1;
          }
          current_window_start = index + 1;
      }
  }
  print!("There are {} valid passports\n", num_valid);
}

fn main() {
    if false {
        day1();
        day2();
        day3();
    }

    day4();
}
