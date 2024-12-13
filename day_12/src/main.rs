use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Region = (char, HashSet<(usize, usize)>);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let grid = parse_input(input);

    get_regions(grid)
        .into_iter()
        .map(|(_, locations)| locations.len() * get_perimeter(locations))
        .sum()
}

fn part_two(input: &str) -> usize {
    let grid = parse_input(input);

    get_regions(grid)
        .into_iter()
        .map(|(_, locations)| locations.len() * get_sides(&locations))
        .sum()
}

fn get_regions(grid: Vec<Vec<char>>) -> Vec<Region> {
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = Vec::new();

    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            if visited_positions.contains(&(col_index, row_index)) {
                continue;
            }

            let region_locations = get_region_locations(&grid, (col_index, row_index));
            visited_positions.extend(&region_locations);
            regions.push((grid[row_index][col_index], region_locations));
        }
    }

    regions
}

fn get_region_locations(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut locations = HashSet::new();
    locations.insert(position);
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    look_around_for_region_extent(grid, &mut visited_positions, position, &mut locations);

    locations
}

fn look_around_for_region_extent(
    grid: &Vec<Vec<char>>,
    visited_positions: &mut HashSet<(usize, usize)>,
    position: (usize, usize),
    locations: &mut HashSet<(usize, usize)>,
) {
    let viable_positions: Vec<(usize, usize)> = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(|vector| (vector.0 + position.0 as i32, vector.1 + position.1 as i32))
        .filter(|new_position| position_is_in_grid(grid, *new_position))
        .map(|new_position| {
            let new_position = (new_position.0 as usize, new_position.1 as usize);
            visited_positions.insert(position);
            new_position
        })
        .collect();

    let new_positions_to_check: Vec<&(usize, usize)> = viable_positions
        .iter()
        .filter(|new_position| !visited_positions.contains(new_position))
        .filter(|new_position| !locations.contains(new_position))
        .filter(|new_position| grid[new_position.1][new_position.0] == grid[position.1][position.0])
        .collect();

    new_positions_to_check.iter().for_each(|new_position| {
        locations.insert(**new_position);
        look_around_for_region_extent(grid, visited_positions, **new_position, locations);
    })
}

fn get_perimeter(locations: HashSet<(usize, usize)>) -> usize {
    let known_locations: Vec<(i32, i32)> = locations
        .iter()
        .map(|location| (location.0 as i32, location.1 as i32))
        .collect();
    let vectors: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    known_locations
        .iter()
        .map(|location| {
            vectors
                .iter()
                .map(|vector| (vector.0 + location.0, vector.1 + location.1))
                .filter(|adjoining_location| !known_locations.contains(adjoining_location))
                .count()
        })
        .sum()
}

fn get_sides(locations: &HashSet<(usize, usize)>) -> usize {
    let mut internal_vertexes = 0_usize;
    let mut external_vertexes = 0_usize;
    let locations_i32 =
        HashSet::from_iter(locations.iter().cloned().map(|(x, y)| (x as i32, y as i32)));

    for location in locations {
        internal_vertexes += count_internal_vertexes(location, &locations_i32);
        external_vertexes += count_external_vertexes(location, &locations_i32);
    }

    external_vertexes + internal_vertexes
}

fn count_internal_vertexes(location: &(usize, usize), locations: &HashSet<(i32, i32)>) -> usize {
    [
        [(1, -1), (1, 0)],
        [(1, 1), (1, 0)],
        [(-1, 1), (-1, 0)],
        [(-1, -1), (-1, 0)],
    ]
    .iter()
    .map(|vectors| convert_to_locations(Vec::from(vectors), location))
    .filter(|location_to_check: &Vec<(i32, i32)>| {
        locations.contains(&location_to_check[0]) && !locations.contains(&location_to_check[1])
    })
    .count()
}

fn count_external_vertexes(location: &(usize, usize), locations: &HashSet<(i32, i32)>) -> usize {
    [
        [(1, 0), (0, 1), (1, 1)],
        [(1, 0), (0, -1), (1, -1)],
        [(-1, 0), (0, -1), (-1, -1)],
        [(-1, 0), (0, 1), (-1, 1)],
    ]
    .iter()
    .map(|vectors| convert_to_locations(Vec::from(vectors), location))
    .filter(|locations_to_check: &Vec<(i32, i32)>| {
        !locations.contains(&locations_to_check[0])
            && !locations.contains(&locations_to_check[1])
            && !locations.contains(&locations_to_check[2])
    })
    .count()
}

fn convert_to_locations(vectors: Vec<(i32, i32)>, location: &(usize, usize)) -> Vec<(i32, i32)> {
    vectors
        .iter()
        .map(|vector| (vector.0 + location.0 as i32, vector.1 + location.1 as i32))
        .collect()
}

fn position_is_in_grid(grid: &[Vec<char>], position: (i32, i32)) -> bool {
    position.0 >= 0
        && position.1 >= 0
        && position.0 < grid[0].len() as i32
        && position.1 < grid.len() as i32
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[test]
fn small_input() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    assert_eq!(1930, part_one(input));
    assert_eq!(1206, part_two(input));
}

#[test]
fn smallest_input() {
    let input = "AAAA
BBCD
BBCC
EEEC";

    assert_eq!(140, part_one(input));
}

#[test]
fn has_internal_holes() {
    let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    assert_eq!(772, part_one(input));
}

#[test]
fn e_shape() {
    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    assert_eq!(236, part_two(input));
}

#[test]
fn ab_shape() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    assert_eq!(368, part_two(input));
}
