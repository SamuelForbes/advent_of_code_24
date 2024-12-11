use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    
    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u32 {
    let map = build_map(input);
    find_trails(map).iter().sum()
}

fn part_two(input: &str) -> u32 {
    let map = build_map(input);
    find_trail_ratings(map).iter().sum()
}

fn build_map(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_trails(map: Vec<Vec<u32>>) -> Vec<u32> {
    let mut trail_scores = Vec::new();

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, item) in row.iter().enumerate() {
            if *item == 0 {
                trail_scores.push(calculate_trail_score(&map, (column_index, row_index)));
            }
        }
    }

    trail_scores
}

fn calculate_trail_score(map: &Vec<Vec<u32>>, starting_position: (usize, usize)) -> u32 {
    let mut score = 0_u32;
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    look_for_trail_top(&mut score, &mut visited_positions, map, starting_position);

    score
}

fn look_for_trail_top(score: &mut u32, visited_positions: &mut HashSet<(i32, i32)>, map: &Vec<Vec<u32>>,position: (usize, usize)) {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|vector| (position.0 as i32 + vector.0, position.1 as i32 + vector.1))
        .filter(|new_position| is_within_map_bounds(map, *new_position))
        .filter(|new_position| { map[new_position.1 as usize][new_position.0 as usize] == map[position.1][position.0] + 1})
        .for_each(|(x, y)| {
            if map[y as usize][x as usize] == 9 && !visited_positions.contains(&(x, y)){
                visited_positions.insert((x, y));
                *score += 1;
            } else if !visited_positions.contains(&(x, y)) {
                visited_positions.insert((x, y));
                look_for_trail_top(score, visited_positions, map, (x as usize, y as usize))
            }
        });
}

fn find_trail_ratings(map: Vec<Vec<u32>>) -> Vec<u32> {
    let mut trail_ratings = Vec::new();

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, item) in row.iter().enumerate() {
            if *item == 0 {
                trail_ratings.push(calculate_trail_rating(&map, (column_index, row_index)));
            }
        }
    }

    trail_ratings
}

fn calculate_trail_rating(map: &Vec<Vec<u32>>, starting_position: (usize, usize)) -> u32 {
    let mut rating = 0_u32;
    find_trail_rating(&mut rating, map, starting_position);

    rating
}

fn find_trail_rating(rating: &mut u32, map: &Vec<Vec<u32>>, position: (usize, usize)) {
    if map[position.1][position.0] == 9{
        *rating += 1;
    }
    
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|vector| (position.0 as i32 + vector.0, position.1 as i32 + vector.1))
        .filter(|new_position| is_within_map_bounds(map, *new_position))
        .filter(|new_position| { map[new_position.1 as usize][new_position.0 as usize] == map[position.1][position.0] + 1})
        .for_each(|(x, y)| find_trail_rating(rating, map, (x as usize, y as usize)));
}

fn is_within_map_bounds(map: &Vec<Vec<u32>>, starting_position: (i32, i32)) -> bool {
    starting_position.0 >= 0
        && starting_position.1 >= 0
        && starting_position.0 < map[0].len() as i32
        && starting_position.1 < map.len() as i32
}

#[test]
fn small_input() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    assert_eq!(vec!(5, 6, 5, 3, 1, 3, 5, 3, 5), find_trails(build_map(input)));
    assert_eq!(36, part_one(input));

    assert_eq!(vec!(20, 24, 10, 4, 1, 4, 5, 8, 5), find_trail_ratings(build_map(input)));
    assert_eq!(81, part_two(input));
}
