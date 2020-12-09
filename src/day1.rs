use crate::common_io;

pub(crate) fn day1() {
    let lines = common_io::read_a_file("./day1_expenses.txt")
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
