use crate::{
    parser::{Rotation, parse_full_list},
    wraparound::WraparoundCounter,
};

mod parser;
mod wraparound;

fn count_zeroes(list: &[Rotation]) -> (i32, i32) {
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
