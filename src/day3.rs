use crate::common_io;

pub fn day3() {
    let lines = common_io::read_a_file("./day3_trees.txt").unwrap();

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

