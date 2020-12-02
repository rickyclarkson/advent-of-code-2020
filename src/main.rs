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
                println!("Day 1: Found these matches, {} + {} == 2020, multiplied they are {}\n", expense1, expense2, expense1 * expense2);
            }
        }
    }
}

fn main() {
    day1();
}
