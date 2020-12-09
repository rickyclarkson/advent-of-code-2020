pub fn day2() {
    use crate::common_io;
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

    let num_valid = common_io::read_a_file("./day2_passwords.txt")
        .unwrap()
        .iter()
        .map(move |line| PolicyAndPassword::from_str(line).unwrap())
        .filter(move |policy_and_password| policy_and_password.is_valid())
        .count();
    print!("Day 2 part 1: There are {} valid passwords.\n", num_valid);

    let num_valid = common_io::read_a_file("./day2_passwords.txt")
        .unwrap()
        .iter()
        .map(move |line| PolicyAndPassword::from_str(line).unwrap())
        .filter(move |policy_and_password| policy_and_password.is_valid_part2())
        .count();
    print!("Day 2 part 2: There are {} valid passwords.\n", num_valid);
}
