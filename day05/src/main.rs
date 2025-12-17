use crate::parser::parse_range;

mod parser;
fn main() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let input_lines = input.lines();
    let mut ranges = vec![];
    let mut availables: Vec<i64> = vec![];
    let mut found_empty_line = false;

    for line in input_lines {
        if !found_empty_line {
            if line.is_empty() {
                found_empty_line = true;
                continue;
            }
            ranges.push(parse_range(line).unwrap().1)
        } else {
            availables.push(line.parse().unwrap())
        }
    }

    ranges.sort();

    println!("Ranges: {:?}", ranges);
    println!("Availables: {:?}", availables);

    let mut count = 0;
    'outer: for number in availables {
        for range in ranges.iter() {
            if range.contains(number) {
                println!("Ingredient {} is fresh", number);
                count += 1;
                continue 'outer;
            }
        }
    }
    println!("There are {} fresh ingredients", count);
    ////////// Part 2
    let mut count = 0;
    let mut current_idx = 0;
    while let Some(current_range) = ranges.get(current_idx) {
        let mut current_start = current_range.start;
        let mut current_end = current_range.end;
        // continue consuming overlapping ranges, updading the end.
        // the start is always the lowes because the ranges array was sorted beforehand.
        while ranges
            .get(current_idx)
            .is_some_and(|rng| rng.start <= current_end)
        {
            let overlapping_range = ranges.get(current_idx).unwrap();
            current_end = std::cmp::max(overlapping_range.end, current_end);
            current_idx += 1;
        }
        count += current_end - current_start + 1;
        println!(
            "Got range {}-{} at index {}. Current total: {}",
            current_start, current_end, current_idx, count
        );
    }
    println!("Total ids: {}", count)
}
