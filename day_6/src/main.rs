use crate::Direction::{East, North, South, West};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_vector(&self) -> (i32, i32) {
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }

    fn next(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
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

fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let starting_point = find_start(&grid);
    let track = get_track(&grid, &starting_point);

    HashSet::<(i32, i32)>::from_iter(track.iter().map(|value| value.0)).len()
}

fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let starting_point = find_start(&grid);
    let track = get_track(&grid, &starting_point);

    count_potential_obstructions(&grid, track)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == '^' {
                return (x, y);
            }
        }
    }
    panic!("Could not find start point");
}

fn get_track(grid: &[Vec<char>], starting_point: &(usize, usize)) -> Vec<((i32, i32), Direction)> {
    let mut visited_locations = vec![];
    let mut current_location = (starting_point.0 as i32, starting_point.1 as i32);
    let mut current_direction = North;

    while location_is_in_grid(&current_location, grid) {
        visited_locations.push((current_location, current_direction));
        (current_location, current_direction) =
            get_new_location(grid, current_location, current_direction)
    }

    visited_locations
}

fn location_is_in_grid(location: &(i32, i32), grid: &[Vec<char>]) -> bool {
    location.1 >= 0
        && location.0 >= 0
        && location.1 < grid.len() as i32
        && location.0 < grid[0].len() as i32
}

fn get_new_location(
    grid: &[Vec<char>],
    current_location: (i32, i32),
    mut current_direction: Direction,
) -> ((i32, i32), Direction) {
    let mut transformation = current_direction.get_vector();
    let mut new_location = (
        current_location.0 + transformation.0,
        current_location.1 + transformation.1,
    );

    if location_is_in_grid(&new_location, grid)
        && grid[new_location.1 as usize][new_location.0 as usize] == '#'
    {
        current_direction = current_direction.next();
        transformation = current_direction.get_vector();
        new_location = (
            current_location.0 + transformation.0,
            current_location.1 + transformation.1,
        );
    }

    (new_location, current_direction)
}

fn count_potential_obstructions(grid: &[Vec<char>], track: Vec<((i32, i32), Direction)>) -> usize {
    let mut total = 0;
    for index in 0..track.len() {
        if blocker_would_create_loop(grid, &track, index) {
            total += 1;
        }
    }

    total
}

fn blocker_would_create_loop(
    grid: &[Vec<char>],
    track: &[((i32, i32), Direction)],
    index: usize,
) -> bool {
    let mut direction = track[index].1.next();
    let mut transformation = direction.get_vector();
    let mut turns = vec!(track[index].0);
    let mut location = track[index].0.clone();

    while location_is_in_grid(&location, grid) {
        let next_location = (location.0 + transformation.0, location.1 + transformation.1);

        if location_is_in_grid(&next_location, grid) {
            let next_grid_square = grid[next_location.1 as usize][next_location.0 as usize];

            if next_grid_square == '#' {
                turns.push(location);
                direction = direction.next();
                transformation = direction.get_vector();
                location = (location.0 + transformation.0, location.1 + transformation.1);
            } else {
                location = next_location;
            }

            if turns.contains(&location) {
                return true;
            }

        } else {
            return false;
        }
    }
    false
}

#[test]
fn small_input() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    assert_eq!(41, part_one(input));
    assert_eq!(6, part_two(input));
}
