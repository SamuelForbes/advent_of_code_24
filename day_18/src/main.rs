use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {:?} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let mut map = vec![vec!['.'; 71]; 71];
    let ram_sequence = parse_input(input);
    make_n_ram_fall(&mut map, 1024, &ram_sequence);

    find_shortest_path(&map)
}

fn part_two(input: &str) -> (usize, usize) {
    let mut map = vec![vec!['.'; 71]; 71];
    let ram_sequence = parse_input(input);

    find_first_blocker(&mut map, &ram_sequence)
}

fn find_first_blocker(map: &mut [Vec<char>], ram_sequence: &[(usize, usize)]) -> (usize, usize) {
    for falling_ram in ram_sequence {
        map[falling_ram.1][falling_ram.0] = '#';
    }

    let reversed_ram_sequence: Vec<&(usize, usize)> = ram_sequence.iter().rev().collect();

    for (index, raising_ram) in reversed_ram_sequence.iter().enumerate() {
        map[raising_ram.1][raising_ram.0] = '.';

        let shortest_path = find_shortest_path(map);
        if shortest_path != 100_000 {
            return *reversed_ram_sequence[index];
        }
    }

    (0, 0)
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn make_n_ram_fall(map: &mut [Vec<char>], n: usize, ram_sequence: &[(usize, usize)]) {
    for index in 0..n {
        let falling_ram = ram_sequence[index];
        map[falling_ram.1][falling_ram.0] = '#';
    }
}

fn find_shortest_path(map: &[Vec<char>]) -> usize {
    let mut graph = vec![vec![100_000; map[0].len()]; map.len()];
    make_moves(map, &mut graph, (0, 0), 0);

    graph[graph.len() - 1][graph[0].len() - 1]
}

fn make_moves(
    map: &[Vec<char>],
    graph: &mut [Vec<usize>],
    position: (usize, usize),
    running_score: usize,
) {
    let new_spaces: Vec<(usize, usize)> = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter(|(y, x)| {
            let new_y = position.0 as i32 + *y;
            let new_x = position.1 as i32 + *x;
            new_y >= 0 && new_x >= 0 && new_y < map.len() as i32 && new_x < map[0].len() as i32
        })
        .map(|&(y, x)| {
            (
                (position.0 as i32 + y) as usize,
                (position.1 as i32 + x) as usize,
            )
        })
        .filter(|&(y, x)| map[y][x] != '#')
        .filter(|&(y, x)| graph[y][x] > (running_score + 1))
        .collect();

    for (y, x) in new_spaces {
        graph[y][x] = running_score + 1;
        make_moves(map, graph, (y, x), running_score + 1)
    }
}

#[test]
fn small_input() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    let mut map = vec![vec!['.'; 7]; 7];
    let ram_sequence = parse_input(input);
    make_n_ram_fall(&mut map, 12, &ram_sequence);

    assert_eq!(22, find_shortest_path(&map));
    assert_eq!(
        (6, 1),
        find_first_blocker(&mut vec![vec!['.'; 7]; 7], &ram_sequence)
    );
}
