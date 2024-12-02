use std::fs;
use std::time::Instant;
use crate::Direction::{Ascending, Descending};
use crate::State::{Safe, Unsafe};

#[derive(PartialEq, Debug)]
enum State {
    Safe,
    Unsafe,
}

#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let start = Instant::now();
    println!("Part one: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> i32{
    let reports = parse_input(input);
    reports.into_iter()
        .map(|report| to_state(&report))
        .filter(|state| *state == Safe)
        .count() as i32
}

fn to_state(report: &[i32]) -> State{
    let start_direction = if report[0] > report[1] {Descending} else {Ascending};

    for index in 0..report.len() - 1 {
        let delta = (report[index] - report[index+1]).abs();
        let delta_direction = if report[index] > report[index + 1] {Descending} else {Ascending};

        if !(1..=3).contains(&delta) || delta_direction != start_direction {
            return Unsafe
        }
    }

    Safe
}

fn part_two(input: &str) -> i32{
    let reports = parse_input(input);
    reports.into_iter()
        .map(to_state_with_dampener)
        .filter(|state| *state == Safe)
        .count() as i32
}

fn to_state_with_dampener(report: Vec<i32>) -> State{
    match to_state(&report) {
        Safe => Safe,
        Unsafe => {
            for index in 0..report.len() {
                let mut edited_report = report.clone();
                edited_report.remove(index);
                let state_without = to_state(&edited_report);
                if state_without == Safe {
                    return Safe;
                }
            }
            
            Unsafe
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|value| value.parse().unwrap())
            .collect()
    })
        .collect()
}

#[test]
fn small_input(){
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    assert_eq!(vec!(
        vec!(7, 6, 4, 2, 1),
        vec!(1, 2, 7, 8, 9),
        vec!(9, 7, 6, 2, 1),
        vec!(1, 3, 2, 4, 5),
        vec!(8, 6, 4, 4, 1),
        vec!(1, 3, 6, 7, 9)
    ), parse_input(input));
    assert_eq!(2, part_one(input));
    assert_eq!(4, part_two(input));
}
