use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(char: char) -> Direction {
        match char {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let (mut map, directions) = parse_input(input);
    let mut robot_location = find_robot_location(&map);
    
    for direction in directions {
        let path_ahead = get_squares_in_direction(&map, &robot_location, &direction);

        if robot_can_move(&path_ahead) {
            move_robot(&mut map, &mut robot_location, &direction, &path_ahead);
        }
    }

    calculate_coordinates(map)
}

fn find_robot_location(map: &[Vec<char>]) -> (usize, usize) {
    for (y, line) in map.iter().enumerate() {
        if line.contains(&'@') {
            return (line.iter().position(|c| *c == '@').unwrap(), y);
        }
    }
    unreachable!()
}

fn get_squares_in_direction(
    map: &[Vec<char>],
    robot_location: &(usize, usize),
    direction: &Direction,
) -> Vec<char> {
    match direction {
        Direction::Up => (0..robot_location.1)
            .rev()
            .map(|index| map[index][robot_location.0])
            .collect(),
        Direction::Down => ((robot_location.1 + 1)..map.len())
                .map(|index| map[index][robot_location.0])
                .collect(),
        Direction::Left => map[robot_location.1][0..robot_location.0]
            .iter()
            .rev()
            .copied()
            .collect(),
        Direction::Right => map[robot_location.1][robot_location.0 + 1..].to_vec(),
    }
}

fn robot_can_move(path_ahead: &[char]) -> bool {
    let first_block = path_ahead.iter().position(|block| *block == '#').unwrap();
    path_ahead[0..first_block].contains(&'.')
}

fn move_robot(
    map: &mut Vec<Vec<char>>,
    location: &mut (usize, usize),
    direction: &Direction,
    path_ahead: &Vec<char>,
) {
    let new_path = get_moved_path(path_ahead);
    map[location.1][location.0] = '.';

    match direction {
        Direction::Up => {
            for (path_index, index) in (location.1 - new_path.len()..location.1).rev().enumerate() {
                map[index][location.0] = new_path[path_index];
            }
            location.1 -= 1;
        }
        Direction::Down => {
            for (path_index, index) in (location.1 + 1..location.1 + 1 + new_path.len()).enumerate() {
                map[index][location.0] = new_path[path_index];
            }
            location.1 += 1;
        }
        Direction::Left => {
            for (path_index, index) in (location.0 - new_path.len()..location.0).rev().enumerate() {
                map[location.1][index] = new_path[path_index];
            }
            
            location.0 -= 1;
        }
        Direction::Right => {
            for (path_index, index) in ((location.0 + 1)..(location.0 + 1 + new_path.len())).enumerate() {
                map[location.1][index] = new_path[path_index];
            }
            
            location.0 += 1;
        }
    }
}

fn get_moved_path(path_ahead: &Vec<char>) -> Vec<char> {
    let first_blocker = path_ahead.iter().position(|block| *block == '#').unwrap();
    let mut blocks_to_change = path_ahead[0..first_blocker].to_vec();
    if blocks_to_change[0] == 'O' {
        let first_space = blocks_to_change.iter().position(|block| *block == '.').unwrap();
        blocks_to_change.remove(first_space);
        blocks_to_change.insert(0, '.');
    }
    
    blocks_to_change[0] = '@';

    blocks_to_change
}

fn calculate_coordinates(map: Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| if *c == 'O' { y * 100 + x } else { 0 })
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    let splitter = if input.contains('\r') {
        "\r\n\r\n"
    } else {
        "\n\n"
    };
    let parts: Vec<&str> = input.split(splitter).collect();
    assert_eq!(2, parts.len());

    let map = parts[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let instructions = parts[1]
        .lines()
        .flat_map(|line| line.chars().map(Direction::from))
        .collect();

    (map, instructions)
}

#[test]
fn part_test() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    let (mut map, _) = parse_input(input);
    let mut robot_location = find_robot_location(&map);
    assert_eq!((2, 2), robot_location);
    assert_eq!(
        vec!('#', '#'),
        get_squares_in_direction(&map, &robot_location, &Direction::Left)
    );
    assert_eq!(
        vec!('.', 'O', '.', '.', '#'),
        get_squares_in_direction(&map, &robot_location, &Direction::Right)
    );
    assert_eq!(
        vec!('.', '#'),
        get_squares_in_direction(&map, &robot_location, &Direction::Up)
    );
    assert_eq!(
        vec!('.', '#', '.', '.', '#'),
        get_squares_in_direction(&map, &robot_location, &Direction::Down)
    );

    assert!(!robot_can_move(&['#', '#']));
    assert!(robot_can_move(&['.', '#']));
    assert!(!robot_can_move(&['O', '#']));
    assert!(robot_can_move(&['.', 'O', '.', '.', '#']));
}

#[test]
fn small_test(){
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    
    part_one(input);
}

#[test]
fn full_test() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    assert_eq!(10092, part_one(input));
}
