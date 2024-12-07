use std::fs;
use std::time::Instant;
use crate::Operation::{Add, Multiply, Concat};

#[derive(Clone, Debug)]
enum Operation {
    Multiply,
    Add,
    Concat
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u64 {
    parse_input(input).iter()
        .filter(test_can_be_made_from_components)
        .map(|(test, _)| test)
        .sum()
}

fn test_can_be_made_from_components((test, components): &&(u64, Vec<u64>)) -> bool {
    *test == calculate_add_or_multiply(*test, components, Add, 0, 0)
}

fn calculate_add_or_multiply(search_value: u64, components: &Vec<u64>, operation: Operation, mut running_total: u64, index: usize) -> u64 {
    if index == components.len() {
        return running_total;
    }
    
    match operation {
        Multiply => {
            running_total *= components[index];
        },
        Add => {
            running_total += components[index];
        },
        _ => panic!("Unexpected operation")
    }

    let total_after_multiply = calculate_add_or_multiply(search_value, components, Multiply, running_total, index + 1);
    let total_after_adding = calculate_add_or_multiply(search_value, components, Add, running_total, index + 1);
    
    if search_value == total_after_multiply{
        total_after_multiply
    } else if search_value == total_after_adding{
        total_after_adding
    } else {
        0
    }
}

fn part_two(input: &str) -> u64 {
    parse_input(input).iter()
        .filter(test_can_be_made_with_three_operators)
        .map(|(test, _)| test)
        .sum()
}

fn test_can_be_made_with_three_operators((test, components): &&(u64, Vec<u64>)) -> bool {
    *test == add_multiply_concat(*test, components, Add, 0, 0)
}

fn add_multiply_concat(search_value: u64, components: &Vec<u64>, operation: Operation, mut running_total: u64, index: usize) -> u64 {
    if index == components.len() {
        return running_total;
    }
    
    if running_total > search_value {
        return 0;
    }

    match operation {
        Multiply => {
            running_total *= components[index];
        },
        Add => {
            running_total += components[index];
        },
        Concat => {
            running_total = format!("{}{}", running_total, components[index]).parse().unwrap();
        }
    }

    let total_after_multiply = add_multiply_concat(search_value, components, Multiply, running_total, index + 1);
    let total_after_adding = add_multiply_concat(search_value, components, Add, running_total, index + 1);
    let total_after_concat = add_multiply_concat(search_value, components, Concat, running_total, index + 1);

    if search_value == total_after_multiply{
        total_after_multiply
    } else if search_value == total_after_adding{
        total_after_adding
    } else if search_value == total_after_concat {
        total_after_concat
    } else {
        0
    }
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(':').collect::<Vec<&str>>();
            let test_value = parts[0].parse::<u64>().unwrap();
            let component_values = parts[1]
                .split_ascii_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
            (test_value, component_values)
        })
        .collect()
}

#[test]
fn small_input() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(3749, part_one(input));
    assert_eq!(11387, part_two(input));
}
