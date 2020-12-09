pub fn day4() {
    use std::collections::HashMap;
    use crate::common_io;

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

    let lines = common_io::read_a_file("./day4_passports.txt").unwrap(); // I manually added a blank line to this.
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

