use crate::Instruction::{Do, Dont, Multiply};
use regex::Regex;
use std::fs;
use std::time::Instant;

#[derive(PartialEq, Debug)]
enum Instruction {
    Do,
    Dont,
    Multiply(u32, u32),
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let start = Instant::now();
    println!("Part one: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(|multiplication| match multiplication {
            Multiply(x, y) => x * y,
            _ => 0,
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let instructions = parse_input(input);
    let mut index = 0;
    let mut total = 0;
    let mut calculating = true;
    
    while index < instructions.len() {
        let instruction = &instructions[index];
        match instruction {
            Multiply(x, y) => {
                if calculating { total += x * y; } 
            },
            Do => calculating = true,
            Dont => calculating = false,
        }
        
        index += 1;
    }
    
    total
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let expression = Regex::new(r"mul\([0-9,]+,[0-9)]+\)|do\(\)|don't\(\)").expect("Could not compile Regex");

    expression
        .find_iter(input)
        .map(|match_instance| {
            let match_string = match_instance.as_str();
            let split: Vec<&str> = match_string
                .split(['(', ')', ','])
                .filter(|item| !item.eq_ignore_ascii_case("mul") && !item.eq_ignore_ascii_case(""))
                .collect();
            
            if split.len() == 2 {
                Multiply(split[0].parse().unwrap(), split[1].parse().unwrap())
            } else if split[0] == "don't" {
                Dont
            } else if split[0] == "do" {
                Do
            } else {
                panic!("{:?} does not match the pattern", split);
            }
        })
        .collect()
}

#[test]
fn small_input() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(
        vec!(
            Multiply(2, 4),
            Multiply(5, 5),
            Multiply(11, 8),
            Multiply(8, 5)
        ),
        parse_input(input)
    );
    assert_eq!(161, part_one(input));
}

#[test]
fn with_controls() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(
        vec!(
            Multiply(2, 4),
            Dont,
            Multiply(5, 5),
            Multiply(11, 8),
            Do,
            Multiply(8, 5)
        ),
        parse_input(input)
    );
    assert_eq!(48, part_two(input));
}
