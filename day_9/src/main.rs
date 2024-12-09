use std::fs;
use std::ptr::replace;
use std::time::Instant;

#[derive(Debug)]
#[derive(Clone)]
struct DataBlock {
    item: Option<u64>,
    start_index: usize,
    quantity: usize,
    moved: bool
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &str) -> u64 {
    let instructions = input.trim().chars().collect::<Vec<char>>();
    let disk = build_disk(instructions);

    compress_disk(disk)
        .iter()
        .enumerate()
        .map(|(index, item)| index as u64 * item)
        .sum()
}

fn part_two(input: &str) -> u64 {
    let instructions = input.trim().chars().collect::<Vec<char>>();
    let disk = build_disk(instructions);
    0

 //  compress_disk_by_file(disk)
 //        .iter()
 //        .enumerate()
 //        .map(|(index,item)| {
 //            if item.is_none() { 0 } else { index * item.unwrap() }
 //        })
 //        .sum()
}

fn build_disk(instructions: Vec<char>) -> Vec<Option<u64>> {
    let mut disk: Vec<Option<u64>> = Vec::new();
    let mut index = 0;

    while index < instructions.len() {
        let instruction = instructions[index].to_digit(10).unwrap();
        let item = if index % 2 == 0 { Some(index as u64 / 2) } else { None };

        for _ in 0..instruction {
            disk.push(item);
        }

        index += 1;
    }

    disk
}

fn compress_disk(disk: Vec<Option<u64>>) -> Vec<u64> {
    let mut compressed_disk: Vec<u64> = Vec::new();
    let mut forward_index = 0;
    let mut backward_index = disk.len() - 1;

    loop {
        if disk[forward_index].is_none() {
            if disk[backward_index].is_some() {
                compressed_disk.push(disk[backward_index].unwrap());
                forward_index += 1;
            }
            backward_index -= 1;
        } else {
            compressed_disk.push(disk[forward_index].unwrap());
            forward_index += 1;
        }

        if backward_index < forward_index {
            break;
        }
    }

    compressed_disk
}

fn build_disk_into_blocks(instructions: Vec<char>) -> Vec<DataBlock> {
    let mut disk: Vec<DataBlock> = Vec::new();
    let mut index = 0;
    let mut block_index = 0;
    let mut iter = instructions.iter();
    let mut is_some = true;

    loop {
        let instruction = iter.next();
        if instruction.is_none() {
            break;
        }
        
        let item = if is_some { Some(block_index as u64 / 2) } else { None };
        let quantity = instruction.unwrap().to_digit(10).unwrap() as usize;
        
        if quantity != 0 {
            disk.push(DataBlock {
                item,
                start_index: index,
                quantity,
                moved: false
            });
        }
        
        index += quantity;
        block_index += 1;
        is_some = !is_some;
    }

    disk
}

fn compress_blocks(disk: Vec<DataBlock>) -> Vec<DataBlock> {
    let mut compressed_disk = disk.clone();
    let mut backward_index = disk.len() - 1;

    while backward_index > 0 {
        println!("{backward_index}");
        let item_to_move = &compressed_disk[backward_index];
        println!("Item to move {:?}", item_to_move);
        if item_to_move.item.is_some() {
            let spot_to_move = find_spot_for_block(&compressed_disk, backward_index);
            if let Some(spot) = spot_to_move {
                let replaced_item = &compressed_disk[spot];
                let mut replacements = vec!(DataBlock {
                    item: item_to_move.item,
                    start_index: replaced_item.start_index,
                    quantity: item_to_move.quantity,
                    moved: true
                });
                
                if item_to_move.quantity < replaced_item.quantity {
                    replacements.push(DataBlock{
                     item: None,
                        start_index: replaced_item.start_index + item_to_move.quantity,
                        quantity: replaced_item.quantity - item_to_move.quantity,
                        moved: false
                    })
                }
                
                compressed_disk.splice(spot..spot + 1, replacements);
                compressed_disk[backward_index] = DataBlock{
                    item: None,
                    start_index: item_to_move.clone().start_index,
                    quantity: item_to_move.quantity,
                    moved: false
                }
            }
        }
        backward_index -= 1;
    }
    
    compressed_disk
}

fn find_spot_for_block(disk: &Vec<DataBlock>, item_index: usize) -> Option<usize> {
    let mut forward_index = 0;
    println!("Item to move index {item_index}");
    let item_to_move = &disk[item_index];
    
    while forward_index < item_index {
        let potential_spot = &disk[forward_index];
        println!("{:?}, {}, {}, {}", item_to_move.item, forward_index, potential_spot.quantity, item_to_move.quantity);
        if potential_spot.item.is_none() && potential_spot.quantity >= item_to_move.quantity{
            return Some(forward_index);
        }
        forward_index += 1;
    }
    
    None
}

#[test]
fn small_input() {
    let input = "2333133121414131402";
    assert_eq!(1928, part_one(input));
    println!("{:?}", compress_blocks(build_disk_into_blocks(input.chars().collect())));
    assert_eq!(2858, part_two(input));
}
