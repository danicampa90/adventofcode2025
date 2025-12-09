use std::collections::{HashMap, LinkedList, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let mut maximums = vec![];
    for line in input.lines() {
        maximums.push(calc_max(line));
    }
    println!("{:?}", maximums);
    println!("Sum of maximums: {}", maximums.iter().cloned().sum::<i32>());

    for line in input.lines() {
        let line: Vec<u8> = line
            .as_bytes()
            .iter()
            .filter(|c| **c >= '0' as u8 && **c <= '9' as u8)
            .cloned()
            .collect();
        let t = DigitsCache::new(2, &line).solve();
        println!("Same calculation, but with new method: {}", t)
    }
    let mut maximums = vec![];
    for line in input.lines() {
        let line: Vec<u8> = line
            .as_bytes()
            .iter()
            .filter(|c| **c >= '0' as u8 && **c <= '9' as u8)
            .cloned()
            .collect();

        let t = DigitsCache::new(12, &line).solve();
        maximums.push(t);
        println!("Same calculation, but with 12 digits: {}", t)
    }
    println!(
        "Sum of maximums with 12 digits: {}",
        maximums.iter().cloned().sum::<u128>()
    );
}

fn calc_max(line: &str) -> i32 {
    let mut max = 0;
    for char1_idx in 0..line.len() {
        for char2_idx in char1_idx + 1..line.len() {
            let ch1 = line.as_bytes()[char1_idx] - ('0' as u8);
            let ch2 = line.as_bytes()[char2_idx] - ('0' as u8);
            if ch1 <= 9 && ch2 <= 9 {
                max = std::cmp::max(max, ch1 as i32 * 10 + ch2 as i32);
            }
        }
    }
    max
}

struct DigitsCache {
    biggest_number_bytes: Vec<u8>,
    remaining: VecDeque<u8>,
}

impl DigitsCache {
    pub fn new(max_digits_to_pick: usize, input_string_ascii: &[u8]) -> Self {
        Self {
            biggest_number_bytes: input_string_ascii[0..max_digits_to_pick].to_vec(),
            remaining: input_string_ascii[max_digits_to_pick..].to_vec().into(),
        }
    }
    pub fn solve(mut self) -> u128 {
        //println!("To process: {:?}", self.remaining);
        while let Some(digit) = self.remaining.pop_front() {
            self.explore_add(digit)
        }
        return ascii_to_number(&self.biggest_number_bytes);
    }
    pub fn explore_add(&mut self, digit: u8) {
        let mut current_biggest_number_buffer = self.biggest_number_bytes.to_vec();
        let mut current_biggest_number = ascii_to_number(&self.biggest_number_bytes);
        let mut buffer = Vec::with_capacity(self.biggest_number_bytes.len());
        for i in 0..self.biggest_number_bytes.len() {
            buffer.clear();
            buffer.extend_from_slice(&self.biggest_number_bytes);
            buffer.remove(i);
            buffer.push(digit);
            let value = ascii_to_number(&buffer);
            if value > current_biggest_number {
                /*println!(
                    "Number {} is bigger than {}, adding {}",
                    value, current_biggest_number, digit
                );*/
                current_biggest_number = value;
                std::mem::swap(&mut current_biggest_number_buffer, &mut buffer);
            }
        }
        std::mem::swap(
            &mut current_biggest_number_buffer,
            &mut self.biggest_number_bytes,
        );
    }
}

pub fn ascii_to_number(input: &[u8]) -> u128 {
    let mut num = 0;
    for char in input {
        let ch_val = char - ('0' as u8);
        if ch_val <= 9 {
            num = num * 10 + (ch_val as u128);
        }
    }
    num
}

#[cfg(test)]
#[test]
pub fn test_number_parsing() {
    assert_eq!(123, ascii_to_number(&['1' as u8, '2' as u8, '3' as u8]));
    assert_eq!(
        123,
        ascii_to_number(&['0' as u8, '1' as u8, '2' as u8, '3' as u8])
    );
    assert_eq!(
        9870,
        ascii_to_number(&['9' as u8, '8' as u8, '7' as u8, '0' as u8])
    )
}
