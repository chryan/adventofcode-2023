use std::cmp::max;
use std::fmt;
use std::fmt::{Formatter};
use regex::Regex;
use crate::common;

fn extract_count_from_regex(re: &Regex, parse_str: &str) -> u32 {
    if let Some(c) = re.captures(&parse_str) {
        if let Some(result) = c.get(1) {
            if let Ok(parsed_result) = result.as_str().parse::<u32>() {
                return parsed_result;
            }
        }
    }
    0
}

struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    fn new(game_str: &str, re_game: &Regex, re_red: &Regex, re_green: &Regex, re_blue: &Regex) -> Game {
        let mut id: u32 = 0;
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        if let Some(c) = re_game.captures(&game_str) {
            let (game_num_str, sets_str) = (&c[1], &c[2]);
            if let Ok(game_num) = game_num_str.parse::<u32>() {
                id = game_num;
                for set_str in sets_str.split(';') {
                    max_red = max(max_red, extract_count_from_regex(&re_red, &set_str));
                    max_green = max(max_green, extract_count_from_regex(&re_green, &set_str));
                    max_blue = max(max_blue, extract_count_from_regex(&re_blue, &set_str));
                }
            }
        }

        Game {
            id,
            max_red,
            max_green,
            max_blue,
        }
    }

    pub fn check_valid(&self, r: u32, g: u32, b: u32) -> bool {
        self.max_red <= r &&
            self.max_green <= g &&
            self.max_blue <= b
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: ({}, {}, {})", self.id, self.max_red, self.max_green, self.max_blue)
    }
}

fn get_games() -> Vec<Game> {
    let lines = common::read_lines("assets/day2_input.txt").unwrap();

    let re_game = Regex::new(r"Game (\d+): (.+)").unwrap();
    let re_red_extract = Regex::new(r"(\d+)\sred").unwrap();
    let re_green_extract = Regex::new(r"(\d+)\sgreen").unwrap();
    let re_blue_extract = Regex::new(r"(\d+)\sblue").unwrap();

    let mut games = Vec::new();

    for line in lines {
        let parse_line = line.unwrap();
        let game = Game::new(parse_line.as_str(), &re_game, &re_red_extract, &re_green_extract, &re_blue_extract);
        games.push(game);
    }
    games
}

pub fn run_part1() {
    let games = get_games();

    let mut accum_id: u32 = 0;
    for ref game in games {
        if game.check_valid(12, 13, 14) {
            accum_id += game.id;
        }
    }

    println!("day2-run_part1: {}", accum_id);
}

pub fn run_part2() {
    let games = get_games();

    let mut accum_power: u32 = 0;
    for ref game in games {
        accum_power += game.max_red * game.max_green * game.max_blue;
    }

    println!("day2-run_part2: {}", accum_power);
}