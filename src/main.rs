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
        birth_year: u32,
        issue_year: u32,
        expiration_year: u32,
        height: String,
        hair_color: String,
        eye_color: String,
        passport_id: String,
        _country_id: String,
    }

    impl Passport {
        fn is_valid(&self) -> bool {
            let is_valid_height = if self.height.ends_with("cm") {
                let cm: u32 = self.height[0..self.height.chars().count() - 2]
                    .parse()
                    .unwrap();
                cm >= 150 && cm <= 193
            } else if self.height.ends_with("in") {
                let inch: u32 = self.height[0..self.height.chars().count() - 2]
                    .parse()
                    .unwrap();
                inch >= 59 && inch <= 76
            } else {
                false
            };

            let is_valid_hair_color = {
                use regex::Regex;
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                re.is_match(&self.hair_color)
            };

            let is_valid_eye_color = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(&self.eye_color.as_str());

            let is_valid_passport_id = {
                use regex::Regex;
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                re.is_match(&self.passport_id)
            };

            self.birth_year >= 1920
                && self.birth_year <= 2002
                && self.issue_year >= 2010
                && self.issue_year <= 2020
                && self.expiration_year >= 2020
                && self.expiration_year <= 2030
                && is_valid_height
                && is_valid_hair_color
                && is_valid_eye_color
                && is_valid_passport_id
        }

        fn parse(lines: &[String]) -> Passport {
            let attributes: HashMap<&str, &str> = lines
                .iter()
                .flat_map(|line| line.split(" "))
                .map(|attr| attr.split(":").collect::<Vec<&str>>())
                .map(|parts| (parts[0], parts[1]))
                .collect();

            Passport {
                birth_year: attributes.get("byr").unwrap_or(&"").parse().unwrap_or(0),
                issue_year: attributes.get("iyr").unwrap_or(&"").parse().unwrap_or(0),
                expiration_year: attributes.get("eyr").unwrap_or(&"").parse().unwrap_or(0),
                height: attributes.get("hgt").unwrap_or(&"").to_string(),
                hair_color: attributes.get("hcl").unwrap_or(&"").to_string(),
                eye_color: attributes.get("ecl").unwrap_or(&"").to_string(),
                passport_id: attributes.get("pid").unwrap_or(&"").to_string(),
                _country_id: attributes.get("cid").unwrap_or(&"").to_string(),
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

fn day5() {
    let lines = read_a_file("./day5_boarding_passes.txt").unwrap();
    let mut seat_numbers: Vec<u32> = lines
        .iter()
        .map(|line| {
            line.replace('F', "0")
                .replace('B', "1")
                .replace('L', "0")
                .replace('R', "1")
        })
        .map(|bin| u32::from_str_radix(&bin, 2).unwrap())
        .collect();

    seat_numbers.sort();

    for (index, number) in seat_numbers.iter().enumerate() {
        if seat_numbers[(index + 1) as usize] != number + 1 {
            print!("Your seat number is {}\n", number + 1);
            return;
        }
    }
}

fn day6() {
    use std::collections::HashSet;

    struct Group {
        lines: Vec<String>,
    }

    impl Group {
        fn _count_yeses(&self) -> usize {
            self.lines
                .iter()
                .flat_map(|line| line.chars())
                .collect::<HashSet<char>>()
                .len()
        }

        fn count_all_yeses(&self) -> usize {
            let mut all_said_yes: HashSet<char> = self.lines[0].chars().collect();
            for line in &self.lines[1..] {
                let mut to_remove: Vec<char> = vec![];
                for c in &all_said_yes {
                    if !line.contains(|x| x == *c) {
                        to_remove.push(*c);
                    }
                }
                for r in &to_remove {
                    all_said_yes.remove(&r);
                }
            }
            all_said_yes.len()
        }
    }

    let lines = read_a_file("./day6_customs_forms.txt").unwrap();
    let mut groups: Vec<Group> = vec![];
    groups.push(Group { lines: vec![] });
    for line in lines {
        if line.is_empty() {
            groups.push(Group { lines: vec![] });
        } else {
            groups.last_mut().unwrap().lines.push(line);
        }
    }

    print!(
        "Total number of yeses, {}\n",
        groups
            .iter()
            .map(|group| group.count_all_yeses())
            .sum::<usize>()
    );
}

fn day7() {
    let lines: Vec<String> = read_a_file("./day7_bag_rules.txt")
        .unwrap()
        .iter()
        .map(|line| line[0..line.chars().count() - 1].to_string())
        .collect(); // remove the trailing period from each line.

    #[derive(Debug)]
    struct BagInfo {
        color: String,
        contents: Vec<(u32, String)>,
    }

    impl BagInfo {
        fn contains(&self, color: &str) -> bool {
            for (_, contained_color) in &self.contents {
                if contained_color == color {
                    return true;
                }
            }
            false
        }
    }

    let mut bags: Vec<BagInfo> = vec![];
    for line in lines {
        // line ~= <color> bags contain (<n> <color> bag(s)[, <n> <color> bag(s)..] | no other
        // bags.)
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let color = parts[0]; // e.g., light red
        let contents: Vec<(u32, String)> = parts[1]
            .split(", ")
            .flat_map(
                |part| // ~= (<n> <color> bag(s) | no other bags) {
                        if part == "no other bags" {
                            vec!()
                        } else {
                            // part ~= <n> <color> bag(s)
                            let parts : Vec<&str>= part.split(" ").collect();
                            let number : u32 = parts[0].parse().expect(part);
                            vec!((number, format!("{} {}", parts[1], parts[2])))
                        }
            )
            .collect();
        bags.push(BagInfo {
            color: color.to_string(),
            contents: contents,
        });
    }

    use std::collections::HashMap;

    /// Finds if the supplied bag contains the specified color.
    /// If it's not immediately present, recurses to see if one of the bag's contents contains
    /// the specified color.
    fn recursive_contains(
        color: &str,
        bag_info: &BagInfo,
        bag_infos: &Vec<BagInfo>,
        seen: &mut HashMap<String, bool>,
    ) -> bool {
        if *seen.get(&bag_info.color).unwrap_or(&false) {
            return true;
        }

        if bag_info.contains(color) {
            seen.insert(bag_info.color.to_string(), true);
            return true;
        } else {
            for other in bag_infos {
                if other.color == bag_info.color {
                    continue;
                }
                if bag_info
                    .contents
                    .iter()
                    .map(|(number, sub_color)| sub_color)
                    .any(|c| *c == other.color)
                    && (*seen.get(&other.color).unwrap_or(&false)
                        || recursive_contains(color, other, bag_infos, seen))
                {
                    seen.insert(bag_info.color.to_string(), true);
                    return true;
                }
            }
            seen.insert(bag_info.color.to_string(), false);
            return false;
        }
    }

    let mut count = 0;
    let mut seen = HashMap::new();
    for bag in &bags {
        if recursive_contains("shiny gold", &bag, &bags, &mut seen) {
            count += 1;
        }
    }

    println!(
        "There are {} bag colors that can contain a shiny gold bag",
        count
    );
}

fn main() {
    if false {
        day1();
        day2();
        day3();
        day4();
        day5();
        day6();
    }

    day7();
}
