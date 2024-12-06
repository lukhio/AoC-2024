use std::fs::File;
use std::io::{ BufRead, BufReader };

/// Possible states for `Multiplier` state machine
#[derive(Debug, Clone, Copy)]
enum MultiplierState {
    Ignore,
    CharM,
    CharU,
    CharL,
    LeftBracket,
    Digit,
    Comma,
    RightBracket
}

/// Provide the default value for `MultiplierState`
impl Default for MultiplierState {
    fn default() -> Self {
        Self::Ignore
    }
}

/// Allow conversion from `u8` to `MultiplierState`
impl From<u8> for MultiplierState {
    fn from(val: u8) -> Self {
        match val {
            1 => MultiplierState::CharM,
            2 => MultiplierState::CharU,
            3 => MultiplierState::CharL,
            4 => MultiplierState::LeftBracket,
            5 => MultiplierState::Digit,
            6 => MultiplierState::Comma,
            7 => MultiplierState::RightBracket,
            _ => MultiplierState::Ignore
        }
    }
}

/// States of each element of the ASCII table
///
/// By default the state is `0`. Only the elements
/// that make a valid expression have an actual state.
const STATES: [MultiplierState; 256] = {
    let mut array = [MultiplierState::Ignore; 256];
    array[40]  = MultiplierState::LeftBracket;     // (
    array[41]  = MultiplierState::RightBracket;     // )
    array[44]  = MultiplierState::Comma;     // ,
    array[48]  = MultiplierState::Digit;     // 0
    array[49]  = MultiplierState::Digit;     // 1
    array[50]  = MultiplierState::Digit;     // 2
    array[51]  = MultiplierState::Digit;     // 3
    array[52]  = MultiplierState::Digit;     // 4
    array[53]  = MultiplierState::Digit;     // 5
    array[54]  = MultiplierState::Digit;     // 6
    array[55]  = MultiplierState::Digit;     // 7
    array[56]  = MultiplierState::Digit;     // 8
    array[57]  = MultiplierState::Digit;     // 9
    array[109] = MultiplierState::CharM;     // m
    array[117] = MultiplierState::CharU;     // u
    array[108] = MultiplierState::CharL;     // l
    array
};

/// State transitions
///
/// When we encounter an element that could be
/// part of a valid expression, we need to update
/// the machine's internal state. This table holds
/// the transitions between two states.
const TRANSITIONS: [[u8; 8]; 8] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 2, 0, 0, 0, 0, 0],
    [0, 0, 0, 3, 0, 0, 0, 0],
    [0, 0, 0, 0, 4, 0, 0, 0],
    [0, 0, 0, 0, 0, 5, 0, 0],
    [0, 0, 0, 0, 0, 5, 6, 7],
    [0, 0, 0, 0, 0, 5, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0],
];

/// State machine
///
/// Our multiplier needs to keep track of its
/// internal state, two operands that will be
/// multiplied together, and if there was a comma
/// in the current expression.
#[derive(Debug, Default)]
struct Multiplier {
    state: MultiplierState,
    first_operand: u64,
    second_operand: u64,
    seen_comma: bool,
}

impl Multiplier {
    /// Updates one of the operands
    ///
    /// If we encounter an integer after seeing `mul(`
    /// we need to store it as one of the operands.
    /// If we have already seen a comma, the second operand
    /// is updated, otherwise the first.
    /// We convert each digit as a `u64`. We are going to
    /// multiply and sum together a lot of numbers so better
    /// to use big integers and avoid a potential overflow.
    fn update_operand(&mut self, raw: char) {
        // Parse the number into an integer
        if let Some(digit) = raw.to_digit(10) {
            if ! self.seen_comma {
                self.first_operand = self.first_operand * 10 + digit as u64;
            } else {
                self.second_operand = self.second_operand * 10 + digit as u64;
            }
        }
    }

    /// Multiply the two operands together
    ///
    /// This function is called after fully parsing a valid
    /// expression, including the closing parenthesis.
    fn compute(&self) -> u64 {
        self.first_operand * self.second_operand
    }

    /// Clear the internal state
    ///
    /// If we find ourselves back to the initial state
    /// (either after fully parsing an expression or when
    /// handling an invalid expression) we need to reset
    /// both the internal state and the operands back to
    /// the default valiues.
    fn clear(&mut self) {
        self.state = MultiplierState::Ignore;
        self.first_operand = 0;
        self.second_operand = 0;
        self.seen_comma = false;
    }
}

/// Parse a line from the input
///
/// We parse the line character by character and use the
/// state machine to parse the valid expressions and compute
/// the final result (i.e., the sum of the results of the
/// multiplications) for this line.
fn parse(line: &str) -> u64 {
    // The final result for this line
    let mut result = 0;

    // Create the state machine with default values
    let mut machine = Multiplier::default();

    // Loop across the bytes of the line
    for byte in line.as_bytes().iter() {
        // Get the state for the burrent byte
        let byte_state = STATES[*byte as usize] as usize;

        // Update the machine's internal state
        machine.state = TRANSITIONS[machine.state as usize][byte_state].into();

        // Depending on the new state, we might need to take some action
        match machine.state {
            // Initial state, we reset the machine
            MultiplierState::Ignore => { machine.clear(); },
            // We have a valid expression so far and `byte`
            // is a digit: we store it in the state machine
            MultiplierState::Digit => { machine.update_operand(*byte as char); },
            // We have a valid expression so far and `byte`
            // is a comma: we let the machine know
            MultiplierState::Comma => { machine.seen_comma = true; },
            // Comlete valid expression, we can compute
            // the result and add it to the accumulator
            MultiplierState::RightBracket => {
                result += machine.compute();
                machine.clear();
            },
            // For all the other states we do nothing
            _ => { },
        }
    }

    result
}

/// Part one of the problem
///
/// This is basically a wrapper to read
/// the input and parse it.
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

fn main() {
    let res = part_one().unwrap();
    println!("Part one: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse(test), 161);
    }

    #[test]
    fn test_part1_no_invalid() {
        let test = "xmul(2,4%&mul[3,7]!@^do_not_mul5,5)+mul(3264]then(ul(11,8)mul(85))";
        assert_eq!(parse(test), 0);
    }
}
