use std::fs;
use std::time::Instant;

const XMAS_SEARCH_SEQUENCE: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS_OPTIONS: [[char; 3]; 2] = [['M','A','S'],['S','A','M']];

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let start = Instant::now();
    println!("Part one: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    count_instances(&parse_input(input))
}

fn count_instances(grid: &Vec<Vec<char>>) -> usize {
    let mut total: usize = 0;

    for column_index in 0..grid.len() {
        for row_index in 0..grid[column_index].len() {
            if grid[column_index][row_index] == XMAS_SEARCH_SEQUENCE[0] {
                total += count_instances_at_position(grid, (column_index, row_index));
            }
        }
    }

    total
}

fn count_instances_at_position(grid: &Vec<Vec<char>>, location: (usize, usize)) -> usize {
    [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (-1, 1),
    ]
        .map(|vector| check_string_in_direction(grid, (location.0 as i32, location.1 as i32), vector, 1))
        .iter()
        .filter(|result| **result)
        .count()
}

fn check_string_in_direction(grid: &Vec<Vec<char>>, location: (i32, i32), vector: (i32, i32), index: usize) -> bool {
    if index > XMAS_SEARCH_SEQUENCE.len() - 1 {
        return true
    }
    
    let location_to_check;
    
    if new_location_is_in_grid(grid, location, vector) {
        location_to_check = ((location.0 + vector.0) as usize, (location.1 + vector.1) as usize);
    } else {
        return false
    }
    
    if grid[location_to_check.0][location_to_check.1] != XMAS_SEARCH_SEQUENCE[index] {
       false
    } else {
       check_string_in_direction(grid, (location_to_check.0 as i32, location_to_check.1 as i32), vector, index + 1) 
    }
}

fn new_location_is_in_grid(grid: &[Vec<char>], location: (i32, i32), vector: (i32, i32)) -> bool {
    location.0 + vector.0 >= 0 &&
        location.0 + vector.0 < grid.len() as i32 &&
        location.1 + vector.1 >= 0 &&
        location.1 + vector.1 < grid[0].len() as i32
}

fn part_two(input: &str) -> usize {
    count_crossed_mas_instances(&parse_input(input))
}

fn count_crossed_mas_instances(grid: &[Vec<char>]) -> usize {
    let mut total: usize = 0;

    for column_index in 0..grid.len() {
        for row_index in 0..grid[column_index].len() {
            if grid[column_index][row_index] == 'A' && is_crossed_mas(grid, (column_index, row_index)) {
                total += 1;
            }
        }
    }

    total
}

fn is_crossed_mas(grid: &[Vec<char>], location: (usize, usize)) -> bool {
    
    if mas_contained_to_grid_bounds(grid, location) {
        let forward = [grid[location.0 - 1][location.1 - 1], 'A', grid[location.0 + 1][location.1 + 1]];
        let backwards = [grid[location.0 - 1][location.1 + 1], 'A', grid[location.0 + 1][location.1 - 1]];
        
        return MAS_OPTIONS.contains(&forward) && MAS_OPTIONS.contains(&backwards);
    } 
    
    false
}

fn mas_contained_to_grid_bounds(grid: &[Vec<char>], location: (usize, usize)) -> bool{
    [(0,1), (1,0), (0,-1), (-1, 0)]
        .iter()
        .map(|vector| new_location_is_in_grid(grid, (location.0 as i32, location.1 as i32), *vector))
        .all(|value| value)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[test]
fn small_input() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(18, part_one(input));
    assert_eq!(9, part_two(input));
}


