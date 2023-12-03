use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;

struct Game {
    blue: u64,
    red: u64,
    green: u64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

fn parse_line(line: &str) -> Game {
    println!("Parsing {}", line);
    let parts = line.split(":").nth(1).unwrap();
    let mut g = Game::new();

    let shows = parts.split(";");
    for s in shows {
        for color in s.split(",") {
            if color.contains("red") {
                let c = color.replace(" red", "").trim().to_string();
                let num = c.parse::<u64>().unwrap();
                g.red = std::cmp::max(g.red, num);
            } else if color.contains("blue") {
                let c = color.replace(" blue", "").trim().to_string();
                let num = c.parse::<u64>().unwrap();
                g.blue = std::cmp::max(g.blue, num);
            } else if color.contains("green") {
                let c = color.replace(" green", "").trim().to_string();
                let num = c.parse::<u64>().unwrap();
                g.green = std::cmp::max(g.green, num);
            }
        }
        //parse 4 red, 1 green, 15 blue
    }
    println!(" |-> r: {} b: {} g: {}", g.red, g.blue, g.green);

    return g;
}

fn filter_by_colors(games: &Vec<Game>, red: u64, blue: u64, green: u64) -> u64 {
    let mut total: u64 = 0;

    for i in 0..games.len() {
        let g = &games[i];
        if g.red <= red && g.blue <= blue && g.green <= green {
            total += (i + 1) as u64
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
    let mut games: Vec<Game> = Vec::with_capacity(10);
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let g = parse_line(line);
        let power = g.red * g.blue * g.green;
        total_power += power;
        games.push(g);
    }

    //12 red cubes, 13 green cubes, and 14 blue cubes
    let total = filter_by_colors(&games, 12, 14, 13);
    println!("Total {} Power {}", total, total_power);
}
