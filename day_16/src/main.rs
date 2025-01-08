use crate::Direction::{East, North, South, West};
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_turn_directions(&self) -> Vec<Direction> {
        match self {
            North | South => vec![West, East],
            East | West => vec![North, South],
        }
    }

    fn get_location_in_direction(&self, current_location: (usize, usize)) -> (usize, usize) {
        match self {
            North => (current_location.0, current_location.1 - 1),
            East => (current_location.0 + 1, current_location.1),
            South => (current_location.0, current_location.1 + 1),
            West => (current_location.0 - 1, current_location.1),
        }
    }
}

#[derive(Clone)]
struct Path {
    current_direction: Direction,
    current_location: (usize, usize),
    visited_locations: HashSet<(usize, usize)>,
    running_score: usize,
}

impl Path {
    fn new(direction: Direction, current_location: (usize, usize)) -> Path {
        Path {
            current_direction: direction,
            current_location,
            visited_locations: HashSet::new(),
            running_score: 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let map = parse_input(input);

    find_paths(&map)
}

fn find_paths(map: &Vec<Vec<char>>) -> usize {
    let mut start_path = Path::new(East, find_start(map));
    let mut solutions = vec!();

    find_shortest_path_to_finish(map, &mut start_path, &mut solutions);
    
    *solutions.iter().min().unwrap()
}

fn find_shortest_path_to_finish(map: &Vec<Vec<char>>, path: &mut Path, solutions: &mut Vec<usize>) {
    if path.visited_locations.contains(&path.current_location) {
        return;
    }
    
    if reached_end(map, path) {
        solutions.push(path.running_score);
        return;
    } else {
        path.visited_locations.insert(path.current_location);
    }
    
    if available_turns(map, path) {
        path.current_direction
            .get_turn_directions()
            .iter()
            .filter(|direction| {
                let location_to_check = direction.get_location_in_direction(path.current_location);
                map[location_to_check.1][location_to_check.0] == '.'
            })
            .for_each(|direction| {
                let mut new_path = Path {
                    current_direction: direction.clone(),
                    running_score: path.running_score + 1001,
                    visited_locations: path.visited_locations.clone(),
                    current_location: direction.get_location_in_direction(path.current_location)
                };
                
                find_shortest_path_to_finish(map, &mut new_path, solutions);
            });
    }

    if clear_space_ahead(map, path) {
        path.current_location = path.current_direction.get_location_in_direction(path.current_location);
        path.running_score += 1;
        find_shortest_path_to_finish(map, path, solutions);
    }
}

fn clear_space_ahead(map: &Vec<Vec<char>>, path: &Path) -> bool {
    let new_position = path.current_direction.get_location_in_direction(path.current_location);
    map[new_position.1][new_position.0] != '#'
}

fn reached_end(map: &Vec<Vec<char>>, path: &Path) -> bool {
    map[path.current_location.1][path.current_location.0] == 'E'
}

fn available_turns(map: &Vec<Vec<char>>, path: &Path) -> bool {
    let available_turns = path.current_direction
        .get_turn_directions()
        .iter()
        .filter(|direction| {
            let location_to_check = direction.get_location_in_direction(path.current_location);
            map[location_to_check.1][location_to_check.0] == '.'
        })
        .count();
    
    available_turns > 0
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut start = (0_usize, 0_usize);

    map.iter().enumerate().for_each(|(i, row)| {
        if row.contains(&'S') {
            start = (row.iter().position(|char| char == &'S').unwrap(), i);
        }
    });

    start
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[test]
fn small_input() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    assert_eq!(7036, part_one(input));
}

#[test]
fn bigger_input() {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    assert_eq!(11048, part_one(input));
}
