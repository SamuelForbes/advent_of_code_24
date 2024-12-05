use std::fs;
use std::time::Instant;

type Instruction = (u32, u32);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading the file");

    let start = Instant::now();
    println!("Part one: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u32 {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();

    let (instructions, pages) = parse_input(&parts);
    apply_rules(&instructions, &pages)
}

fn part_two(input: &str) -> u32 {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();

    let (instructions, pages) = parse_input(&parts);
    fix_broken(&instructions, &pages)
}

fn apply_rules(instructions: &[Instruction], pages: &[Vec<u32>]) -> u32 {
    pages.iter()
        .filter(|page| page_is_valid(page, instructions))
        .map(|page| get_middle_number(page))
        .sum()
}

fn page_is_valid(page: &[u32], instructions: &[Instruction]) -> bool {
    for (left, right) in instructions {
        for index in 0..page.len() {
            if page[index] == *left {
                for first_half_index in index..0 {
                    if page[first_half_index] == *right {
                        return false;
                    }
                }
            }
            if page[index] == *right {
                for second_half_index in index..page.len() {
                    if page[second_half_index] == *left {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn fix_broken(instructions: &[Instruction], pages: &[Vec<u32>]) -> u32 {
    pages.iter()
        .filter(|page| !page_is_valid(page, instructions))
        .map(|page| fix_page(page, instructions))
        .map(|page| get_middle_number(&page))
        .sum()
}

fn fix_page(page: &Vec<u32>, instructions: &[Instruction]) -> Vec<u32> {
    let mut fixed_page = page.clone();

    while !page_is_valid(&fixed_page, instructions) {
        for (left, right) in instructions {
            for index in 0..page.len() {
                if fixed_page[index] == *left {
                    for first_half_index in index..0 {
                        if fixed_page[first_half_index] == *right && index != page.len() - 1 {
                            fixed_page.remove(first_half_index);
                            fixed_page.insert(index + 1, *right);
                        }
                    }
                }
                if fixed_page[index] == *right {
                    for second_half_index in index..page.len() {
                        if fixed_page[second_half_index] == *left {
                            fixed_page.remove(second_half_index);
                            fixed_page.insert(index, *left);
                        }
                    }
                }
            }
        }
    }

    fixed_page
}

fn get_middle_number(page: &[u32]) -> u32 {
    page[page.len() / 2]
}

fn parse_input(parts: &[&str]) -> (Vec<Instruction>, Vec<Vec<u32>>) {
    let instructions = parts[0]
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numbers: Vec<&str> = line.split('|').collect();
            (numbers[0].parse().unwrap(), numbers[1].parse().unwrap())
        })
        .collect::<Vec<Instruction>>();

    let pages = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    (instructions, pages)
}

#[test]
fn small_input() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let parts: Vec<&str> = input.split("\n\n").collect();

    let (instructions, pages) = parse_input(&parts);

    assert_eq!(143, apply_rules(&instructions, &pages));

    assert_eq!(vec!(97, 75, 47, 61, 53), fix_page(&pages[3], &instructions));
    assert_eq!(vec!(61, 29, 13), fix_page(&pages[4], &instructions));
    assert_eq!(vec!(97, 75, 47, 29, 13), fix_page(&pages[5], &instructions));

    assert_eq!(123, fix_broken(&instructions, &pages));
}
