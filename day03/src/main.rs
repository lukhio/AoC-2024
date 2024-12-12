use std::fs::File;
use std::io::{ BufRead, BufReader };

/// Possible states for `Multiplier` state machine
#[derive(Debug, Clone, Copy, PartialEq)]
enum MultiplierState {
    Ignore,
    LeftBracket,
    Digit,
    Comma,
    RightBracket,
    Operator,
    Qualifier,
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
            1 => MultiplierState::Qualifier,
            2 => MultiplierState::Operator,
            3 => MultiplierState::LeftBracket,
            4 => MultiplierState::Digit,
            5 => MultiplierState::Comma,
            6 => MultiplierState::RightBracket,
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
    array[39]  = MultiplierState::Qualifier;    // '
    array[40]  = MultiplierState::LeftBracket;  // (
    array[41]  = MultiplierState::RightBracket; // )
    array[44]  = MultiplierState::Comma;        // ,
    array[48]  = MultiplierState::Digit;        // 0
    array[49]  = MultiplierState::Digit;        // 1
    array[50]  = MultiplierState::Digit;        // 2
    array[51]  = MultiplierState::Digit;        // 3
    array[52]  = MultiplierState::Digit;        // 4
    array[53]  = MultiplierState::Digit;        // 5
    array[54]  = MultiplierState::Digit;        // 6
    array[55]  = MultiplierState::Digit;        // 7
    array[56]  = MultiplierState::Digit;        // 8
    array[57]  = MultiplierState::Digit;        // 9
    array[100] = MultiplierState::Qualifier;    // d
    array[109] = MultiplierState::Operator;     // m
    array[110] = MultiplierState::Qualifier;    // n
    array[111] = MultiplierState::Qualifier;    // o
    array[116] = MultiplierState::Qualifier;    // t
    array[117] = MultiplierState::Operator;     // u
    array[108] = MultiplierState::Operator;     // l

    array
};

/// State transitions
///
/// When we encounter an element that could be
/// part of a valid expression, we need to update
/// the machine's internal state. This table holds
/// the transitions between two states.
const TRANSITIONS: [[MultiplierState; 8]; 8] = {
    let mut array = [
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
        [MultiplierState::Ignore; 8],
    ];

    array[MultiplierState::Ignore as usize]
         [MultiplierState::Operator as usize] = MultiplierState::Operator;
    array[MultiplierState::Ignore as usize]
         [MultiplierState::Qualifier as usize] = MultiplierState::Qualifier;

    array[MultiplierState::Operator as usize]
         [MultiplierState::Operator as usize] = MultiplierState::Operator;
    array[MultiplierState::Operator as usize]
         [MultiplierState::LeftBracket as usize] = MultiplierState::LeftBracket;

    array[MultiplierState::Qualifier as usize]
         [MultiplierState::Qualifier as usize] = MultiplierState::Qualifier;
    array[MultiplierState::Qualifier as usize]
         [MultiplierState::LeftBracket as usize] = MultiplierState::LeftBracket;

    array[MultiplierState::LeftBracket as usize]
         [MultiplierState::Digit as usize] = MultiplierState::Digit;
    array[MultiplierState::LeftBracket as usize]
         [MultiplierState::RightBracket as usize] = MultiplierState::RightBracket;

    array[MultiplierState::Digit as usize]
         [MultiplierState::Digit as usize] = MultiplierState::Digit;
    array[MultiplierState::Digit as usize]
         [MultiplierState::Comma as usize] = MultiplierState::Comma;
    array[MultiplierState::Digit as usize]
         [MultiplierState::RightBracket as usize] = MultiplierState::RightBracket;

    array[MultiplierState::Comma as usize]
         [MultiplierState::Digit as usize] = MultiplierState::Digit;

    array
};


/// State machine
///
/// Our multiplier needs to keep track of its
/// internal state, two operands that will be
/// multiplied together, and if there was a comma
/// in the current expression.
#[derive(Debug)]
struct Multiplier {
    state: MultiplierState,
    enabled: bool,
    operator: Vec<char>,
    valid_operator: bool,
    qualifier: Vec<char>,
    first_operand: u64,
    second_operand: u64,
    seen_comma: bool,
}

impl Default for Multiplier {
    fn default() -> Self {
        Self {
            state: MultiplierState::Ignore,
            first_operand: 0,
            operator: Vec::new(),
            valid_operator: false,
            qualifier: Vec::new(),
            second_operand: 0,
            seen_comma: false,
            enabled: true,
        }
    }
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

    /// Ensure the operator we have is valid
    /// For this problem we have only one operator but this
    /// functions makes it easier to add new operators.
    fn is_operator_valid(&mut self) -> bool {
        if self.operator == vec!['m', 'u', 'l'] {
            self.valid_operator = true;
        }

        self.valid_operator
    }

    /// Ensure the qualifier we have is valid
    fn is_qualifier_valid(&self) -> bool {
        match self.qualifier.iter().collect::<String>().as_str() {
            "do" | "dont't" => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }

    /// Multiply the two operands together
    ///
    /// This function is called after fully parsing a valid
    /// expression, including the closing parenthesis.
    fn compute(&mut self) -> Option<u64> {
        if self.enabled && self.seen_comma && self.is_operator_valid() {
            println!("---> compute (new result {:?})", Some(self.first_operand * self.second_operand));
            return Some(self.first_operand * self.second_operand)
        }

        println!("---> not computing");
        None
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
        self.operator.clear();
        self.valid_operator = false;
        self.qualifier.clear();
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

        // Save previous state
        let previous_state = machine.state;

        // Update the machine's internal state
        machine.state = TRANSITIONS[machine.state as usize][byte_state];

        println!("byte: {} {byte_state:?} ({})", *byte as char, *byte as usize);
        println!("new state: {:?}", machine.state);
        // println!("machine: enabled ({}) seen comma ({})", machine.enabled, machine.seen_comma);
        println!("{machine:#?}");
        println!("------------------------------");

        // If the machine is disabled, we don't bother checking the state

        // Depending on the new state, we might need to take some action
        match machine.state {
            // Initial state, we reset the machine
            MultiplierState::Ignore => { machine.clear(); },
            // Operator state, we save the bytes to ensure we have the full operator
            MultiplierState::Operator => {
                if previous_state == MultiplierState::Qualifier {
                    machine.clear();
                }
                machine.operator.push(*byte as char);
            },
            // Qualifier state, we save the bytes to ensure we have the full qualifier
            MultiplierState::Qualifier => {
                if previous_state == MultiplierState::Operator {
                    machine.clear();
                }
                machine.qualifier.push(*byte as char);
            },
            // When we have an opening bracket we need to check that
            // we have a valid operator or that we are parsing a qualifier.
            // If not we reset the whole machine.
            MultiplierState::LeftBracket => {
                match previous_state {
                    MultiplierState::Operator => {
                        if ! machine.is_operator_valid() {
                            machine.clear();
                        }
                    },
                    MultiplierState::Qualifier => {
                        if ! machine.is_qualifier_valid() {
                            machine.clear();
                        }
                    },
                    _ => { machine.clear(); },
                }
            },
            // We have a valid expression so far and `byte`
            // is a digit: we store it in the state machine
            MultiplierState::Digit => { machine.update_operand(*byte as char); },
            // We have a valid expression so far and `byte`
            // is a comma: we let the machine know
            MultiplierState::Comma => {
                println!("COMMA {}", machine.is_operator_valid());
                if machine.is_operator_valid() {
                    machine.seen_comma = true;
                } else {
                    machine.clear();
                }
            },
            // Comlete valid expression or operator, we can compute
            // the result and add it to the accumulator, or change
            // the machine `enabled` status.
            MultiplierState::RightBracket => {
                match previous_state {
                    MultiplierState::LeftBracket => {
                        match machine.qualifier.iter().collect::<String>().as_str() {
                            "do" => {
                                println!("---> enabling machine");
                                machine.enabled = true;
                            },
                            "don't" => {
                                println!("---> disabling machine");
                                machine.enabled = false;
                            },
                            _ => { }
                        }
                    },
                    MultiplierState::Digit => {
                        result += machine.compute().unwrap_or(0);
                    }
                    _ => { }
                }

                machine.clear();
            },
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
    fn test_part1_basic() {
        let test = "mul(2,4)";
        assert_eq!(parse(test), 8);
    }

    #[test]
    fn test_part1_no_valid() {
        let test = "xmul(2,4%&mul[3,7]!@^do_not_mul5,5)+mul(3264]then(ul(11,8)mul(85))";
        assert_eq!(parse(test), 0);
    }

    #[test]
    fn test_part2() {
        let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse(test), 48);
    }

    #[test]
    fn test_part2_all() {
        assert_eq!(part_one(), Ok(83595109));
    }
}
