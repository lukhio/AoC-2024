use std::fs::File;
use std::io::{ BufRead, BufReader };

static STATES: [u8; 256] = {
    let mut array = [0u8; 256];
    array[40]  = 4;     // (
    array[41]  = 7;     // )
    array[44]  = 6;     // ,
    array[48]  = 5;     // 0
    array[49]  = 5;     // 1
    array[50]  = 5;     // 2
    array[51]  = 5;     // 3
    array[52]  = 5;     // 4
    array[53]  = 5;     // 5
    array[54]  = 5;     // 6
    array[55]  = 5;     // 7
    array[56]  = 5;     // 8
    array[57]  = 5;     // 9
    array[109] = 1;     // m
    array[117] = 2;     // u
    array[108] = 3;     // l
    array
};

static TRANSITIONS: [[u8; 8]; 8] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 2, 0, 0, 0, 0, 0],
    [0, 0, 0, 3, 0, 0, 0, 0],
    [0, 0, 0, 0, 4, 0, 0, 0],
    [0, 0, 0, 0, 0, 5, 0, 0],
    [0, 0, 0, 0, 0, 5, 6, 7],
    [0, 0, 0, 0, 0, 5, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0],
];

#[derive(Debug, Default)]
struct Multiplier {
    state: u8,
    first_operand: u64,
    second_operand: u64,
    seen_comma: bool,
}

impl Multiplier {
    fn update_operand(&mut self, raw: char) {
        if let Some(digit) = raw.to_digit(10) {
            if ! self.seen_comma {
                self.first_operand = self.first_operand * 10 + digit as u64;
            } else {
                self.second_operand = self.second_operand * 10 + digit as u64;
            }
        }
    }

    fn compute(&self) -> u64 {
        self.first_operand * self.second_operand
    }

    fn clear(&mut self) {
        self.state = 0;
        self.first_operand = 0;
        self.second_operand = 0;
        self.seen_comma = false;
    }
}

fn parse(line: &str) -> u64 {
    let mut machine = Multiplier::default();
    let mut result = 0;

    for byte in line.as_bytes().iter() {
        let transition_index = STATES[*byte as usize] as usize;

        machine.state = TRANSITIONS[machine.state as usize][transition_index];

        match machine.state {
            0 => { machine.clear(); },
            5 => { machine.update_operand(*byte as char); },
            6 => { machine.seen_comma = true; },
            7 => { 
                result += machine.compute();
                machine.clear();
            },
            _ => { },
        }
    }

    result
}

fn parse2(line: &str) -> u64 {
    0
}

fn main() {
    let res = part_one().unwrap();
    println!("Part one: {res}");
}

fn part_one() -> Result<u64, &'static str> {
    let input = File::open("./input")
                           .expect("cannot open input file");
    let reader = BufReader::new(input);

    let mut result = 0;
    for line in reader.lines() {
        let line = line.expect("Error: cannot read line from input");

        result += parse(&line);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse(&test), 161);
    }

    #[test]
    fn test_part2() {
        let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse2(&test), 48);
    }
}
