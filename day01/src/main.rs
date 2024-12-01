use std::fs::File;
use std::iter::zip;
use std::io::{ BufRead, BufReader };

fn main() {
    let distance = part_one().unwrap();
    println!("Part one distance: {distance}");
    let score = part_two().unwrap();
    println!("Part two similarity score: {score}");
}

fn part_one() -> Result<u32, &'static str> {
    let input = File::open("./input")
                           .expect("cannot open input file");
    let reader = BufReader::new(input);

    // firsts ints
    let mut firsts = Vec::new();
    // seconds ints
    let mut seconds = Vec::new();

    for line in reader.lines() {
        // split the line at the first space
        let (first, second) = line.as_ref()
            .expect("Cannot get line ref")
            .split_once(' ')
            .expect("Cannot split line");

        firsts.push(first.parse::<u32>()
                         .expect("Cannot convert {first} to u32"));
        seconds.push(second.trim().parse::<u32>()
                           .expect("Cannot convert {second} to u32"));
    }

    // sort lists
    firsts.sort();
    seconds.sort();

    // loop through both lists and compute distance
    let mut distance = 0;
    for (first, second) in zip(firsts, seconds) {
        distance += first.abs_diff(second);
    }

    Ok(distance)
}

fn part_two() -> Result<u32, &'static str> {
    let input = File::open("./input")
                           .expect("cannot open input file");
    let reader = BufReader::new(input);

    // firsts ints
    let mut firsts = Vec::new();
    // seconds ints
    let mut seconds = Vec::new();

    for line in reader.lines() {
        // split the line at the first space
        let (first, second) = line.as_ref()
            .expect("Cannot get line ref")
            .split_once(' ')
            .expect("Cannot split line");

        firsts.push(first.parse::<u32>()
                         .expect("Cannot convert {first} to u32"));
        seconds.push(second.trim().parse::<u32>()
                           .expect("Cannot convert {second} to u32"));
    }

    // sort lists
    firsts.sort();
    seconds.sort();

    // loop through both lists and compute distance
    let mut score = 0;
    for first in firsts.into_iter() {
        score += first * seconds.iter().filter(|&second| *second == first).count() as u32;
    }

    Ok(score)
}
