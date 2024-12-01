use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read file");
    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> i32 {
    let (mut left_hand_side, mut right_hand_side) = parse_input(input);
    left_hand_side.sort();
    right_hand_side.sort();

    left_hand_side.iter().zip(right_hand_side)
        .map(|(l, r)| (l-r).abs())
        .sum()
}

fn part_two(input: &str) -> i32 {
    let (left_hand_side, right_hand_side) = parse_input(input);
    let mut right_hand_occurrences = get_right_hand_occurrences(right_hand_side);

    left_hand_side.into_iter()
        .map(|left| left * *right_hand_occurrences.entry(left).or_insert(0) as i32)
        .sum()
}

fn get_right_hand_occurrences(right_hand_side: Vec<i32>) -> HashMap<i32, usize> {
    let mut occurrence_counts = HashMap::new();

    right_hand_side.into_iter()
        .for_each(|right|{
            *occurrence_counts.entry(right).or_insert(0) += 1;
        });

    occurrence_counts
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_hand_side = vec!();
    let mut right_hand_side = vec!();

    input.lines().for_each(|line| {
        let line_parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left_hand_side.push(line_parts[0].parse::<i32>().unwrap());
        right_hand_side.push(line_parts[1].parse::<i32>().unwrap());
    });

    (left_hand_side, right_hand_side)
}

#[test]
fn small_input() {
    let small_input = String::from("3   4
    4   3
    2   5
    1   3
    3   9
    3   3");

    assert_eq!((vec!(3,4,2,1,3,3), vec!(4,3,5,3,9,3)), parse_input(&small_input));
    assert_eq!(11, part_one(&small_input));
    assert_eq!(31, part_two(&small_input));
}
