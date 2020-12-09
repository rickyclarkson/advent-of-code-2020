pub fn read_a_file(filename: &str) -> std::io::Result<Vec<String>> {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    let file = File::open(filename)?;

    let file_reader = BufReader::new(file);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

