use crate::Direction::{East, North, South, West};
use itertools::Itertools;
use std::collections::HashMap;
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

#[derive(Clone, Debug)]
struct Path {
    current_direction: Direction,
    current_location: (usize, usize),
    running_score: usize,
    path_tiles: Vec<(usize, usize)>,
}

impl Path {
    fn new(direction: Direction, current_location: (usize, usize)) -> Path {
        Path {
            current_direction: direction,
            current_location,
            running_score: 0,
            path_tiles: Vec::new(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let map = parse_input(input);

    *find_paths(&map)
        .iter()
        .map(|(value, _)| value)
        .min()
        .unwrap()
}

fn part_two(input: &str) -> usize {
    let mut map = parse_input(input);
    let valid_paths = find_paths(&map);
    let min = valid_paths.iter().map(|(value, _)| value).min().unwrap();

    let unique_tiles = valid_paths
        .iter()
        .filter(|(value, _)| value == min)
        .flat_map(|(_, set)| set.iter().collect::<Vec<&(usize, usize)>>())
        .unique()
        .collect::<Vec<&(usize, usize)>>();

    for unique_tile in &unique_tiles {
        map[unique_tile.1][unique_tile.0] = 'O';
    }

    unique_tiles.len()
}

fn find_paths(map: &Vec<Vec<char>>) -> Vec<(usize, Vec<(usize, usize)>)> {
    let mut start_path = Path::new(East, find_start(map));
    let mut solutions = vec![];
    let mut visited_locations = HashMap::new();

    find_shortest_path_to_finish(map, &mut visited_locations, &mut start_path, &mut solutions);

    solutions
}

fn find_shortest_path_to_finish(
    map: &Vec<Vec<char>>,
    visited_locations: &mut HashMap<(usize, usize), usize>,
    path: &mut Path,
    solutions: &mut Vec<(usize, Vec<(usize, usize)>)>,
) {
    if square_already_visited(path) || path_longer_than_existing_solution(path, solutions) {
        return;
    }

    if reached_end(map, path) {
        path.path_tiles.push(path.current_location);
        solutions.push((path.running_score, path.path_tiles.clone()));
        return;
    } else {
        visited_locations.insert(path.current_location, path.running_score);
        path.path_tiles.push(path.current_location);
    }

    make_turns_if_available(map, visited_locations, path, solutions);

    if square_visited_cheaper(path, visited_locations) {
        return;
    }

    if clear_space_ahead(map, path) {
        path.current_location = path
            .current_direction
            .get_location_in_direction(path.current_location);

        path.running_score += 1;
        find_shortest_path_to_finish(map, visited_locations, path, solutions);
    }
}

fn path_longer_than_existing_solution(path: &mut Path, solutions: &mut [(usize, Vec<(usize, usize)>)]) -> bool {
    !solutions.is_empty() && solutions.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0 < path.running_score
}

fn square_already_visited(path: &Path) -> bool {
    path.path_tiles.contains(&path.current_location)
}

fn square_visited_cheaper(
    path: &Path,
    visited_locations: &mut HashMap<(usize, usize), usize>,
) -> bool {
    visited_locations.contains_key(&path.current_location)
        && visited_locations.get(&path.current_location).unwrap() < &path.running_score
}

fn make_turns_if_available(
    map: &Vec<Vec<char>>,
    visited_locations: &mut HashMap<(usize, usize), usize>,
    path: &mut Path,
    solutions: &mut Vec<(usize, Vec<(usize, usize)>)>,
) {    
    for direction in path.current_direction.get_turn_directions().iter() {
        if location_worth_visiting(direction, map, &visited_locations, &path) {
            let mut new_path = Path {
                current_direction: direction.clone(),
                running_score: path.running_score + 1001,
                current_location: direction.get_location_in_direction(path.current_location),
                path_tiles: path.path_tiles.clone(),
            };

            find_shortest_path_to_finish(map, visited_locations, &mut new_path, solutions);
        }
    }
}

fn location_worth_visiting(
    direction: &Direction,
    map: &[Vec<char>],
    visited_locations: &&mut HashMap<(usize, usize), usize>,
    path: &&mut Path,
) -> bool {
    let location_to_check = direction.get_location_in_direction(path.current_location);

    map[location_to_check.1][location_to_check.0] == '.'
        && (!visited_locations.contains_key(&location_to_check)
            || visited_locations.get(&location_to_check).unwrap() >= &(path.running_score + 1001))
}

fn clear_space_ahead(map: &[Vec<char>], path: &Path) -> bool {
    let new_position = path
        .current_direction
        .get_location_in_direction(path.current_location);
    map[new_position.1][new_position.0] != '#'
}

fn reached_end(map: &[Vec<char>], path: &Path) -> bool {
    map[path.current_location.1][path.current_location.0] == 'E'
}

fn find_start(map: &[Vec<char>]) -> (usize, usize) {
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
    assert_eq!(45, part_two(input));
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
    assert_eq!(64, part_two(input));
}
