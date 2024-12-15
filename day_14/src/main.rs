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
}

fn part_one(input: &str, grid: &Vec<Vec<u32>>) -> usize {
    let guards = parse_input(input);
    0
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
    
    let grid = vec![vec![0_u32, 11]; 7];

    assert_eq!(12, part_one(input, &grid));
}
