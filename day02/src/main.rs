use std::collections::BTreeSet;

use crate::parser::parse_range_list;

mod parser;

pub fn is_invalid_part1(number: &i64) -> bool {
    let str = number.to_string();
    if str.len() % 2 != 0 {
        return false; // 2 strings concatenated are always having a length divisible by 2
    }
    let half_len = str.len() / 2;

    if str[0..half_len] != str[half_len..half_len * 2] {
        return false;
    }
    return true;
}

pub fn is_invalid_part2(number: &i64) -> bool {
    let str = number.to_string();
    'outer: for number_of_parts in 2..str.len() + 1 {
        //println!("num {}, check if it's made of {} parts", number, number_of_parts);
        if str.len() % number_of_parts != 0 {
            continue; // N strings concatenated are always having a length divisible by N
        }

        // check that each string is composed of N repeated parts, each part_len characters long.
        let part_len = str.len() / number_of_parts;
        let mut to_check_start_idx = part_len;
        while to_check_start_idx != str.len() {
            if str[0..part_len] != str[to_check_start_idx..to_check_start_idx + part_len] {
                continue 'outer;
            }
            to_check_start_idx += part_len;
        }
        return true;
    }
    return false;
}

fn main() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let (_, result) = parse_range_list(&input).unwrap();
    let invalid_numbers_part1: BTreeSet<_> = result
        .iter()
        .flat_map(|range| range.flatten())
        .filter(is_invalid_part1)
        .collect();
    println!("part 1: {:?}", invalid_numbers_part1.iter().sum::<i64>());

    let invalid_numbers_part2: BTreeSet<_> = result
        .iter()
        .flat_map(|range| range.flatten())
        .filter(is_invalid_part2)
        .collect();
    println!("part 2: {:?}", invalid_numbers_part2.iter().sum::<i64>());
}
