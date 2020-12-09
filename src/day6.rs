pub fn day6() {
    use crate::common_io;
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

    let lines = common_io::read_a_file("./day6_customs_forms.txt").unwrap();
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
