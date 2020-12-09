pub fn day7() {
    let lines: Vec<String> = common_io::read_a_file("./day7_bag_rules.txt")
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
            contents,
        });
    }

    use std::collections::HashMap;
    use crate::common_io;

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
                    .map(|(_number, sub_color)| sub_color)
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

    fn recursive_count(
        bag: &BagInfo,
        bags: &Vec<BagInfo>,
        mut seen_count: &mut HashMap<String, u32>,
    ) -> u32 {
        if seen_count.contains_key(&bag.color) {
            return *seen_count.get(&bag.color).unwrap();
        }
        let mut count = 1; // self
        for (number, sub_color) in &bag.contents {
            count += number
                * recursive_count(
                    bags.iter().find(|b| b.color == *sub_color).unwrap(),
                    &bags,
                    &mut seen_count,
                );
        }
        seen_count.insert(bag.color.to_string(), count);
        count
    }

    if false {
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

    let mut seen_count = HashMap::new();
    println!(
        "A shiny gold bag has {} bags inside it",
        recursive_count(
            bags.iter().find(|b| b.color == "shiny gold").unwrap(),
            &bags,
            &mut seen_count
        ) - 1 // self
    );
}

