use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = "0 37551 469 63 1 791606 2065 9983586";

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(input), start.elapsed());
}

fn part_one(input: &str) -> u64 { 
    parse_input(input)
        .iter()
        .map(|stone|  blink_n_times(&mut vec!(*stone), 25))
        .sum()
}

fn part_two(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(|stone|  blink_n_times(&mut vec!(*stone), 75))
        .sum()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split_ascii_whitespace()
        .map(|item| (item.parse().unwrap(), 1))
        .collect()
}

fn blink_n_times(stones: &mut Vec<(u64, u64)>, blinks: usize) -> u64 {
    let known_conversions = HashMap::new();
    
    let mut new_stones = stones.clone();
    for _ in 0..blinks {
        new_stones = blink(&mut new_stones, known_conversions.clone());
    }
    
    new_stones.iter().map(|(_, count)| *count).sum()
}

fn blink(stones: &mut Vec<(u64, u64)>, mut known_conversions: HashMap<u64, Vec<u64>>) -> Vec<(u64, u64)>{
    let mut map = HashMap::new();
    
    for (stone, count) in stones.iter() {
        known_conversions.entry(*stone).or_insert_with(|| get_new_value(*stone));
        let converts_to = known_conversions.get(stone).unwrap();
        
        for new_stone in converts_to {
            if map.contains_key(new_stone) {
                let entry: &(u64,u64) = map.get(new_stone).unwrap();
                map.insert(entry.0, (entry.0, entry.1 + count));
            } else {
                map.insert(*new_stone, (*new_stone, *count));
            }
        }
    }
    
    map.values().cloned().collect()
}

fn get_new_value(stone: u64) -> Vec<u64>{
    if stone == 0 {
        vec!(1)
    } else if stone.to_string().chars().collect::<Vec<char>>().len() % 2 == 0 {
        let stone_as_chars = stone.to_string().chars().collect::<Vec<char>>();
        let middle = stone_as_chars.len() / 2;

         vec!(
            stone_as_chars[0..middle]
                .iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap(),
            stone_as_chars[middle..stone_as_chars.len()]
                .iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        )
        
    } else {
        vec!(stone * 2024)
    }
}

#[test]
fn small_input() {
    let result = parse_input("125 17")
        .iter()
        .map(|stone|  blink_n_times(&mut vec!(*stone), 6))
        .sum::<u64>();
    
    assert_eq!(22, result);
}
