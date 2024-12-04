use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let safe_count = part_one().unwrap();
    println!("Number of safe reports: {safe_count:?}");
    let safe_count = part_two().unwrap();
    println!("Number of safe reports: {safe_count:?}");
}

fn is_increasing(array: &[u32]) -> bool {
    array.windows(2).all(|w| w[0] < w[1])
}

fn is_decreasing(array: &[u32]) -> bool {
    array.windows(2).all(|w| w[0] > w[1])
}

fn check_diff(array: &[u32]) -> bool {
    array.windows(2).all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
}

fn is_safe(array: &[u32]) -> bool {
    check_diff(array) && (is_increasing(array) || is_decreasing(array))
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

        if is_safe(&elements) {
            safe_reports += 1;
        }
    }

    Ok(safe_reports)
}

fn part_two() -> Result<u32, String> {
    let input = File::open("input")
                           .expect("cannot open input file");
    let reader = BufReader::new(input);

    let mut safe_reports = 0;
    'outer: for line in reader.lines() {
        let line = line.expect("Error: cannot read line from input reader");

        let elements = line.split_whitespace()
            .map(|i| i.parse::<u32>().expect("Error: cannot convert level to u32"))
            .collect::<Vec<u32>>();

        if is_safe(&elements) {
            safe_reports += 1;
            continue;
        }

        for i in 0..elements.len() {
            let mut tmp = elements.clone();
            tmp.remove(i);

            if is_safe(&tmp) {
                safe_reports += 1;
                continue 'outer;
            }
        }
    }

    Ok(safe_reports)
}
