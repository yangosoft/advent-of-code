use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;

fn is_symbol(cb: char) -> bool {
    if cb.is_digit(10) || cb == '.' {
        return false;
    }

    return true;
}

fn parse_line(line: &str, line2: &str, line3: &str) -> u64 {
    println!("Parsing\n{}\n{}", line, line2);

    let mut str_line2 = line2.to_string();
    if str_line2.is_empty() {
        //fill
        str_line2 = (0..line.len()).map(|_| ".").collect::<String>();
    }

    let mut str_up = line3.to_string();
    if str_up.is_empty() {
        //fill
        str_up = (0..line.len()).map(|_| ".").collect::<String>();
    }

    let mut number = 0;
    let mut total: u64 = 0;
    let mut idx_start = 0;
    let mut idx_end = 0;

    let chars_below: Vec<char> = str_line2.chars().collect();

    for (pos, c) in line.chars().enumerate() {
        if c == '.' {
            if number != 0 {
                // check below and diagonal
                let cb = chars_below[pos];
                let mut is_valid = true;
                if !is_symbol(cb) {
                    is_valid = false;
                }

                if pos + 1 < str_line2.len() && !is_valid {
                    //check if diagonal is valid
                    let cb = chars_below[pos + 1];
                    if !is_symbol(cb) {
                        is_valid = false;
                    } else {
                        is_valid = true;
                    }
                }

                if is_valid {
                    println!(" -> number: {}", number);
                    total += number;
                    println!(" -> is valid number: {}", number);
                    number = 0;
                } else {
                    //check up
                }
            }
        } else if !c.is_digit(10) {
            total += number;
            println!(" -> is valid number: {}", number);
            number = 0;
        } else if c.is_digit(10) {
            let d: u64 = c.to_digit(10).unwrap() as u64;
            number = number * 10 + d;
            println!(" -> still valid number: {}", number);
        }
    }

    return total;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let contents = fs::read_to_string(args[1].clone()).unwrap();

    let mut total_power: u64 = 0;

    let lines: Vec<&str> = contents.split("\n").collect();

    for i in 0..lines.len() - 1 {
        let line = lines[i];
        let line2 = lines[i + 1];

        let power = parse_line(line, line2);
        total_power += power;
    }

    println!("Total {}", total_power);
}
