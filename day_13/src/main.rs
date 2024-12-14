use std::fs;
use std::time::Instant;
use rayon::prelude::*;

struct Game {
    prize_location: (i64, i64),
    button_a: (i64, i64),
    button_b: (i64, i64),
}

impl Game {
    fn new_corrected_game(game: &Game) -> Game {
        Game {
            button_a: game.button_a,
            button_b: game.button_b,
            prize_location: (
                game.prize_location.0 + 10000000000000,
                game.prize_location.1 + 10000000000000,
            ),
        }
    }
}

#[derive(Debug)]
struct Solution {
    a: i64,
    b: i64,
}

impl Solution {
    fn new(lead_button: char, a: i64, b: i64) -> Solution {
        if lead_button == 'a' {
            Solution { a, b }
        } else {
            Solution { a: b, b: a }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> i64 {
    let games = parse_input(input);

    games.iter()
        .filter_map(solve) 
        .filter(|solution| solution.0 < 100 && solution.1 < 100)
        .map(|solution| solution.0 * 3 + solution.1)
        .sum()
}

fn part_two(input: &str) -> i64 {
    let games = parse_input(input);

    games
        .par_iter()
        .map(|game| {
            let new_game = Game::new_corrected_game(game);
            solve(&new_game)
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|solution| solution.0 * 3 + solution.1)
        .sum()
}

fn solve(game: &Game) -> Option<(i64, i64)> {
    let b = ((game.prize_location.1 * game.button_a.0) - (game.button_a.1 * game.prize_location.0)) /
        ((game.button_b.1 * game.button_a.0) - (game.button_a.1 * game.button_b.0));
    
    let a = (game.prize_location.0 - game.button_b.0 * b) / game.button_a.0;
    
    if(a * game.button_a.0) + (b * game.button_b.0) == game.prize_location.0 &&
        (a * game.button_a.1) + (b * game.button_b.1) == game.prize_location.1{
        return Some((a, b))
    }
    
    None
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut splitter = "\n\n";
    let mut locations = [12, 14, 2, 4, 7, 9];

    if input.contains("\r\n\r\n") {
        splitter = "\r\n\r\n";
        locations = [14, 16, 2, 4, 8, 10];
    };

    input
        .split(splitter)
        .map(|game| {
            let parts: Vec<&str> = game.split([',', '\n', ':', '+', '=', '\r']).collect();

            Game {
                prize_location: (
                    parts[locations[0]].parse().unwrap(),
                    parts[locations[1]].parse().unwrap(),
                ),
                button_a: (
                    parts[locations[2]].parse().unwrap(),
                    parts[locations[3]].parse().unwrap(),
                ),
                button_b: (
                    parts[locations[4]].parse().unwrap(),
                    parts[locations[5]].parse().unwrap(),
                ),
            }
        })
        .collect()
}

#[test]
fn small_input() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    assert_eq!(Some((80, 40)), solve(&Game {
        prize_location: (8400,5400),
        button_a: (94, 34),
        button_b: (22, 67)
    }));
    assert_eq!(480, part_one(input));
}
