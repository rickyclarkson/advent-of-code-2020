use crate::common_io;

pub fn day5() {
    let lines = common_io::read_a_file("./day5_boarding_passes.txt").unwrap();
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
