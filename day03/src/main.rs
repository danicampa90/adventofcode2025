fn main() {
    println!("hi");
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut maximums = vec![];
    for line in input.lines() {
        maximums.push(calc_max(line));
    }
}

fn calc_max(line: &str) -> i32 {
    let mut max = 0;
    for char1_idx in 0..line.len() {
        for char2_idx in char1_idx..line.len() {
            let ch1 = line.as_bytes()[char1_idx] - ('0' as u8);
            let ch2 = line.as_bytes()[char2_idx] - ('0' as u8);
            if ch1 >= 0 && ch1 <= 9 && ch2 >= 0 && ch2 <= 9 {
                max = std::cmp::max(max, ch1 as i32 * 10 + ch2 as i32);
            }
        }
    }
    max
}
