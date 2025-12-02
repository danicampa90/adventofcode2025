use std::{fs::File, thread::current};

use crate::{
    parser::{Rotation, parse_full_list},
    wraparound::WraparoundCounter,
};

mod parser;
mod wraparound;

fn count_times_stopped_at_0(list: &[Rotation], count_pass_throughs: bool) -> usize {
    let mut result = 0;
    let mut current_position = 50;
    for rotation in list {
        let mut original_position = current_position;

        current_position += match rotation {
            Rotation::Left(num) => {
                //print!("L{}", num);
                -*num
            }
            Rotation::Right(num) => {
                //print!("R{}", num);
                *num
            }
        };

        while current_position < 0 {
            current_position += 100;
            if count_pass_throughs && original_position != 0 {
                result += 1;
            }
            original_position = 1; // :puke:
        }
        while current_position >= 100 {
            current_position -= 100;
            if count_pass_throughs && original_position != 0 {
                result += 1;
            }
            original_position = 1; // :puke:
        }
        //print!("=> {}", current_position);
        if current_position == 0 {
            //println!("<ZERO!>");
            result += 1;
        }
        //println!();
    }
    result
}

fn count_zeroes(list: &[Rotation]) -> (i32, i32) {
    let mut result = 0;
    let mut current_position = WraparoundCounter::new(50, 0, 0, 99);
    for rotation in list {
        current_position.add(match rotation {
            Rotation::Left(num) => {
                //print!("L{}", num);
                -*num
            }
            Rotation::Right(num) => {
                //print!("R{}", num);
                *num
            }
        });
    }
    (current_position.stopped_at, current_position.passed_through)
}

fn main() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let (rest, parsed) = parse_full_list(&input).unwrap();

    assert!(rest.chars().all(char::is_whitespace));
    println!("{:?}", parsed);
    let (stopped_at, passed_through) = count_zeroes(&parsed);
    println!(
        "Stopped at zero {} times and passed through {} times",
        stopped_at, passed_through
    );
}
