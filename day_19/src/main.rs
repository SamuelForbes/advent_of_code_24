use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    // let start = Instant::now();
    // println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let (components, targets) = parse_input(input);
    let mut valid_patterns = HashSet::new();
    let mut invalid_patterns = HashSet::new();

    let mut counter = 0;
    for target in targets {
        if pattern_can_be_made(&components, target, &mut valid_patterns, &mut invalid_patterns) {
            counter += 1;
        }
    }

    counter
}

fn pattern_can_be_made<'a>(components: &[&str], target: &'a str, valid_patterns: &mut HashSet<&'a str>, invalid_patterns: &mut HashSet<&'a str>) -> bool {
    if target.is_empty() {
        return true;
    }

    if valid_patterns.contains(target) {
        return true;
    } else if invalid_patterns.contains(target) {
        return false;
    }

    for component in components {
        if target.starts_with(component) {

            let result = pattern_can_be_made(components, target.strip_prefix(component).unwrap(), valid_patterns, invalid_patterns);

            if result {
                valid_patterns.insert(target);
                return result;
            }
        }
    }

    invalid_patterns.insert(target);
    false
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parts = input.split(['\r','\n']).collect::<Vec<&str>>();
    (
        parts[0].split(',').map(|a| a.trim()).collect(),
        parts[2..].iter().filter(|a| !a.is_empty()).copied().collect(),
    )
}

#[test]
fn small_input() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    assert_eq!(6, part_one(input));
}
