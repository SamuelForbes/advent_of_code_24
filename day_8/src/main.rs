use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

#[derive(Debug, PartialEq)]
struct Antenna {
    x: usize,
    y: usize,
    frequency: char,
}

impl Antenna {
    fn new(location: (usize, usize), character: char) -> Antenna {
        Antenna {
            x: location.0,
            y: location.1,
            frequency: character,
        }
    }

    fn get_location(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

fn part_one(input: &str) -> usize {
    let antennas = get_antennas(input);
    count_unique_antinodes_in_grid(input, &antennas)
}

fn part_two(input: &str) -> usize {
    let antennas = get_antennas(input);
    find_antinodes_with_resonance(input, &antennas)
}

fn count_unique_antinodes_in_grid(input: &str, antennas: &[Antenna]) -> usize {
    let antenna_frequencies = get_frequencies(antennas);
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid_size = (grid[0].len(), grid.len());

    antenna_frequencies
        .iter()
        .flat_map(|frequency| {
            let same_frequency_antennas = antennas
                .iter()
                .filter(|antenna| antenna.frequency == *frequency)
                .collect();

            find_antinodes_for_antennas(&same_frequency_antennas)
        })
        .filter(|position| contained_to_grid(grid_size, *position))
        .unique()
        .count()
}

fn find_antinodes_for_antennas(antennas: &Vec<&Antenna>) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    for source_index in 0..antennas.len() {
        let source_antenna = antennas[source_index];

        for target_antenna in antennas {
            if source_antenna == *target_antenna {
                continue;
            }

            antinodes.push(calculate_antinode(
                source_antenna.get_location(),
                target_antenna.get_location(),
            ));
            antinodes.push(calculate_antinode(
                target_antenna.get_location(),
                source_antenna.get_location(),
            ));
        }
    }

    antinodes
}

fn find_antinodes_with_resonance(input: &str, antennas: &Vec<Antenna>) -> usize {
    let antenna_frequencies = get_frequencies(antennas);
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid_size = (grid[0].len(), grid.len());

    let mut antinodes = antenna_frequencies
        .iter()
        .flat_map(|frequency| {
            let same_frequency_antennas = antennas
                .iter()
                .filter(|antenna| antenna.frequency == *frequency)
                .collect();

            find_antinodes_for_antennas_with_resonance(grid_size, &same_frequency_antennas)
        })
        .unique()
        .map(|position| (position.0 as usize, position.1 as usize))
        .collect::<Vec<(usize, usize)>>();
    
    for antenna in antennas {
        if !antinodes.contains(&antenna.get_location()) {
            antinodes.push(antenna.get_location());
        }
    }

    antinodes.len()
}

fn find_antinodes_for_antennas_with_resonance(
    grid_size: (usize, usize),
    antennas: &Vec<&Antenna>,
) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    for source_index in 0..antennas.len() {
        let source_antenna = antennas[source_index];

        for target_antenna in antennas {
            if source_antenna == *target_antenna {
                continue;
            }

            antinodes.append(&mut get_resonant_antinodes_in_grid(grid_size, source_antenna, target_antenna));
            antinodes.append(&mut get_resonant_antinodes_in_grid(grid_size, target_antenna, source_antenna));
        }
    }

    antinodes
}

fn get_resonant_antinodes_in_grid(grid_size: (usize, usize), source_antenna: &Antenna, target_antenna: &Antenna) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let mut last_node = source_antenna.get_location();
    let mut current_node = (
        target_antenna.get_location().0 as i32,
        target_antenna.get_location().1 as i32,
    );

    loop  {
        let new_antinode = calculate_antinode(
            last_node,
            (current_node.0 as usize, current_node.1 as usize),
        );
        
        if !contained_to_grid(grid_size, new_antinode){
            break;
        }
        last_node = (current_node.0 as usize, current_node.1 as usize);
        current_node = new_antinode;

        antinodes.push(current_node);
    }
    
    antinodes
}

fn contained_to_grid(grid_size: (usize, usize), position: (i32, i32)) -> bool {
    position.0 >= 0
        && position.1 >= 0
        && position.0 < grid_size.0 as i32
        && position.1 < grid_size.1 as i32
}

fn calculate_antinode(source_node: (usize, usize), target_node: (usize, usize)) -> (i32, i32) {
    let vector = (
        target_node.0 as i32 - source_node.0 as i32,
        target_node.1 as i32 - source_node.1 as i32,
    );
    (
        target_node.0 as i32 + vector.0,
        target_node.1 as i32 + vector.1,
    )
}

fn get_frequencies(antennas: &[Antenna]) -> Vec<char> {
    antennas
        .iter()
        .map(|antenna| antenna.frequency)
        .unique()
        .collect()
}

fn get_antennas(input: &str) -> Vec<Antenna> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| Antenna::new((x, y), character))
                .collect::<Vec<Antenna>>()
        })
        .filter(|antenna: &Antenna| antenna.frequency != '.')
        .collect::<Vec<Antenna>>()
}

#[test]
fn small_input() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let antennas = get_antennas(input);
    assert_eq!(7, antennas.len());
    assert_eq!(vec!('0', 'A'), get_frequencies(&antennas));
    assert_eq!((2, 3), calculate_antinode(antennas[0].get_location(), antennas[1].get_location()));
    assert_eq!((11, 0), calculate_antinode(antennas[1].get_location(), antennas[0].get_location()));
    assert_eq!(14, part_one(input));
    
    assert_eq!(3, get_resonant_antinodes_in_grid((12,12), &antennas[1], &antennas[3]).len());
    assert_eq!(8, get_resonant_antinodes_in_grid((12,12), &antennas[6], &antennas[5]).len());
    assert_eq!(2, get_resonant_antinodes_in_grid((12,12), &antennas[5], &antennas[6]).len());
    assert_eq!(34, part_two(input));
}
