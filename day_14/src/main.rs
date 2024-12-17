use std::fs;
use std::time::Instant;

struct Guard {
    start_position: (usize, usize),
    velocity: (i32, i32)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid = vec![vec![0; 101];103];
    
    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input, &grid), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input, &grid), start.elapsed());
}

fn part_one(input: &str, grid: &[Vec<u32>]) -> usize {
    let guards = parse_input(input);
    
    let final_positions: Vec<(usize, usize)> = guards.iter()
        .map(|guard| calculate_position_after_seconds(grid, guard, 100))
        .collect();
    
    calculate_safety_factor(grid, final_positions)
}

fn part_two(input: &str, grid: &[Vec<u32>]) -> usize {
    let guards = parse_input(input);
    let mut smallest_index = 0;
    let mut smallest_safety_factor = None;
    
    for i in 1..=(101 * 103){
        let positions = guards.iter()
            .map(|guard| calculate_position_after_seconds(grid, guard, i))
            .collect();

        let new_safety_factor = calculate_safety_factor(grid, positions);
        if smallest_safety_factor.is_none() || new_safety_factor < smallest_safety_factor.unwrap(){
            smallest_safety_factor = Some(new_safety_factor);
            smallest_index = i;
        } 
    }
    
    smallest_index
}

fn calculate_position_after_seconds(grid: &[Vec<u32>], guard: &Guard, seconds: usize) -> (usize, usize){
    let dx = seconds as i32 * guard.velocity.0;
    let dy = seconds as i32 * guard.velocity.1;
    
    let new_location = (guard.start_position.0 as i32 + dx, guard.start_position.1 as i32 + dy);
    
    let x_grids = new_location.0 / grid[0].len() as i32;
    let y_grids = new_location.1 / grid.len() as i32;
    
    let mut x = new_location.0 - (x_grids * grid[0].len() as i32);
    let mut y = new_location.1 - (y_grids * grid.len() as i32);
    
    if y < 0 {
        y += grid.len() as i32;
    }
    
    if x < 0 {
        x += grid[0].len() as i32;
    }
    
    (x as usize, y as usize)
}

fn calculate_safety_factor(grid: &[Vec<u32>], final_positions: Vec<(usize, usize)>) -> usize {
    let mut quadrant_counts = (0,0,0,0);

    for (x, y) in final_positions {
        if x > grid[0].len() / 2  && y > grid.len() / 2 {
            quadrant_counts.0 += 1;
        } else if x > grid[0].len() / 2  && y < grid.len() / 2 {
            quadrant_counts.1 += 1;
        } else if x < grid[0].len() / 2  && y < grid.len() / 2 {
            quadrant_counts.2 += 1;
        } else if x < grid[0].len() / 2  && y > grid.len() / 2 {
            quadrant_counts.3 += 1;
        }
    }

    quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3
}

fn parse_input(input: &str) -> Vec<Guard> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(['=',' ',',']).collect();
        Guard {
            start_position: (parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            velocity: (parts[4].parse().unwrap(), parts[5].parse().unwrap())
        }
    }).collect()
}

#[test]
fn small_input(){
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    
    let grid = vec![vec![0_u32; 11]; 7];

    assert_eq!((4,1), calculate_position_after_seconds(&grid, &Guard { start_position: (2, 4), velocity: (2,-3) }, 1));
    assert_eq!((6,5), calculate_position_after_seconds(&grid, &Guard { start_position: (2, 4), velocity: (2,-3) }, 2));
    assert_eq!((8,2), calculate_position_after_seconds(&grid, &Guard { start_position: (2, 4), velocity: (2,-3) }, 3));
    assert_eq!((10,6), calculate_position_after_seconds(&grid, &Guard { start_position: (2, 4), velocity: (2,-3) }, 4));
    assert_eq!((1,3), calculate_position_after_seconds(&grid, &Guard { start_position: (2, 4), velocity: (2,-3) }, 5));
    assert_eq!(12, part_one(input, &grid));
}
