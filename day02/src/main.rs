use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let safe_count = part_one().unwrap();
    println!("Number of safe reports: {safe_count:?}");
}

fn part_one() -> Result<u32, String> {
    let input = File::open("input")
                           .expect("cannot open input file");
    let reader = BufReader::new(input);

    let mut safe_reports = 0;
    for line in reader.lines() {
        let line = line.expect("Error: cannot read line from input reader");

        let elements = line.split_whitespace()
            .map(|i| i.parse::<u32>().expect("Error: cannot convert level to u32"))
            .collect::<Vec<u32>>();

        // check all ascending and absolute difference between 1 and 3
        if elements.windows(2)
                   .all(|w| (w[0] < w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)) {
            safe_reports += 1;
            continue;
        }

        // check all descending and absolute difference between 1 and 3
        if elements.windows(2)
                   .all(|w| (w[0] > w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)) {
            safe_reports += 1;
        }
    }

    Ok(safe_reports)
}
