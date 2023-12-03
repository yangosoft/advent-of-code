use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Index;

fn replace_numbers2(txt: &str) -> String {
    println!("PROCESSING LINE: \n{}", txt);

    let numbers = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "9e"),
    ]);

    let mut out = txt.to_string();
    loop {
        for n in numbers.clone() {
            out = out.replace(n.0, n.1);
        }

        let mut done = true;
        for n in numbers.clone() {
            if out.contains(n.0) {
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }
    out = out + "\n";
    return out;
}

fn replace_numbers3(txt: &str) -> String {
    println!("PROCESSING LINE: \n{}", txt);

    let numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
}

fn replace_numbers(txt: &str) -> String {
    println!("PROCESSING LINE: \n{}", txt);

    let numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut indexes = Vec::<usize>::with_capacity(9);

    let mut idx = txt.len() * 2;
    let mut max_idx = 0;

    let mut sel = 0;
    let mut sel2 = 0;

    let mut out: String = txt.replace("o", "oo");
    let mut out = out.replace("t", "tt");
    let mut out = out.replace("e", "ee");
    let mut out = out.replace("n", "nn");

    for i in 0..numbers.len() {
        let k = txt.find(numbers[i]);
        if k.is_some() {
            if k.unwrap() <= idx {
                idx = k.unwrap();
                sel = i;
            }
            if k.unwrap() >= max_idx {
                max_idx = k.unwrap();
                sel2 = i;
            }
        }
    }

    out = out.replace(numbers[sel], &(sel + 1).to_string());
    out = out.replace(numbers[sel2], &(sel2 + 1).to_string());

    /*for i in 0..numbers.len() {
        out = out.replace(numbers[i], &(i + 1).to_string());
    }*/
    out = out + "\n";
    println!("PROCESSED LINE: \n{}", out);

    return out.to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let contents = fs::read_to_string(args[1].clone()).unwrap();

    let mut first: u64 = 0;
    let mut second: u64 = 0;
    let mut total = 0;
    let mut found = false;

    let vlines = contents.split("\n");

    for line in vlines {
        let l = replace_numbers2(line);
        println!("Line {} -> {}", line, l);

        found = false;
        first = 0;
        second = 0;
        for c in l.chars() {
            if c == '\n' {
                total = total + (first * 10) + second;
                println!("To add {} total {}", (first * 10) + second, total);
            }

            if c.to_digit(10).is_none() {
                continue;
            }

            if found == false {
                first = c.to_digit(10).unwrap() as u64;
                second = first;
                println!("First {}", first);
                found = true;
            } else {
                second = c.to_digit(10).unwrap() as u64;
                println!("Second {}", second);
            }
        }
    }


    println!("Total {}", total);
}
