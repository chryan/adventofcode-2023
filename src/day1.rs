use std::fs::File;
use std::io;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;

const DAY1_INPUT_FILE: &str = "assets/day1_input.txt";

struct Digit {
    value: u32,
    found_idx: usize,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_numeric_digits(digits_string: &String) -> Option<(Digit, Digit)> {

    let first_digit_idx = digits_string.find(|c: char| c.is_numeric());
    let first_digit = first_digit_idx
        .map(|idx| (digits_string.as_bytes()[idx] as char))
        .map(|c| c.to_digit(10).unwrap_or_default());

    let second_digit_idx = digits_string.rfind(|c: char| c.is_numeric());
    let second_digit = second_digit_idx
        .map(|idx| (digits_string.as_bytes()[idx] as char))
        .map(|c| c.to_digit(10).unwrap_or_default());

    if let (Some(first), Some(second)) = (first_digit, second_digit)  {
        Some((
            Digit {
                value: first,
                found_idx: first_digit_idx.unwrap(),
            },
            Digit {
                value: second,
                found_idx: second_digit_idx.unwrap(),
            },
        ))
    }
    else {
        None
    }
}

fn parse_string_digits(digits_string: &String) -> Option<(Digit, Digit)> {
    const DIGIT_STRINGS: &'static [&str] = &[
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut first_digit: Option<u32> = None;
    let mut first_digit_idx = usize::MAX;
    let mut second_digit: Option<u32> = None;
    let mut second_digit_idx = usize::MIN;

    for (digit, digit_string) in DIGIT_STRINGS.iter().enumerate() {
        if let Some(found_idx) = digits_string.find(digit_string) {
            if found_idx < first_digit_idx {
                first_digit = Some(digit as u32);
                first_digit_idx = found_idx;
            }
        }
        if let Some(found_idx) = digits_string.rfind(digit_string) {
            if found_idx >= second_digit_idx {
                second_digit = Some(digit as u32);
                second_digit_idx = found_idx;
            }
        }
    }

    if let (Some(first), Some(second)) = (first_digit, second_digit)  {
        Some((
            Digit {
                value: first,
                found_idx: first_digit_idx,
            },
            Digit {
                value: second,
                found_idx: second_digit_idx,
            },
        ))
    }
    else {
        None
    }
}

pub fn run_part1() {
    let lines = read_lines(DAY1_INPUT_FILE).unwrap();
    let mut final_result: u32 = 0;

    for line in lines {
        let parse_line = line.unwrap();

        match parse_numeric_digits(&parse_line) {
            None => { println!("{} has no digits!", parse_line); },
            Some((first, second)) => {
                let parsed_number = first.value * 10 + second.value;
                println!("{}: {}", parsed_number, parse_line);
                final_result += parsed_number;
            },
        }
    }

    println!("Final result: {}", final_result);
}

pub fn run_part2() {
    let lines = read_lines(DAY1_INPUT_FILE).unwrap();
    let mut final_result: u32 = 0;

    for line in lines {
        let parse_line = line.unwrap();

        let numeric_digits = parse_numeric_digits(&parse_line);
        let string_digits = parse_string_digits(&parse_line);

        let first_digit = if let Some((ref numeric_first, _)) = numeric_digits {
            if let Some((ref string_first, _)) = string_digits {
                if string_first.found_idx < numeric_first.found_idx {
                    Some(string_first)
                } else {
                    Some(numeric_first)
                }
            } else {
                Some(numeric_first)
            }
        } else {
            None
        };

        let second_digit = if let Some((_, ref numeric_second)) = numeric_digits {
            if let Some((_, ref string_second)) = string_digits {
                if string_second.found_idx > numeric_second.found_idx {
                    Some(string_second)
                } else {
                    Some(numeric_second)
                }
            } else {
                Some(numeric_second)
            }
        } else {
            None
        };

        if let (Some(first), Some(second)) = (first_digit, second_digit) {
            let parsed_number = first.value * 10 + second.value;
            println!("{}: {}", parsed_number, parse_line);
            final_result += parsed_number;
        } else {
            println!("No number found for {}!", parse_line);
        }
    }

    println!("Final result: {}", final_result);
}