use std::{
    collections::{HashMap, HashSet},
    mem,
};

use libutils::map2d::{Map2D, map_from_lines};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let map = map_from_lines(' ', input_lines).unwrap();
    let start_coordinates = map.filter_values(|v| *v == 'S');
    assert_eq!(
        start_coordinates.len(),
        1,
        "Start coordinates should be only present once on the map"
    );
    let start_coordinate = start_coordinates.first().unwrap();
    println!("Start coordinate: {:?}", start_coordinate);
    ////////////////////
    let mut frontier = HashMap::new();
    frontier.insert(*start_coordinate, 1);

    let mut number_of_splits = 0;

    for _steps in 0..map.size_y() {
        let mut old_frontier = HashMap::new();
        mem::swap(&mut frontier, &mut old_frontier);
        for ((f_x, f_y), timelines) in old_frontier {
            if map.get_value(f_x, f_y + 1) == '^' {
                frontier
                    .entry((f_x - 1, f_y + 1))
                    .and_modify(|t| *t += timelines)
                    .or_insert(timelines);
                frontier
                    .entry((f_x + 1, f_y + 1))
                    .and_modify(|t| *t += timelines)
                    .or_insert(timelines);
                number_of_splits += 1;
            } else {
                frontier
                    .entry((f_x, f_y + 1))
                    .and_modify(|t| *t += timelines)
                    .or_insert(timelines);
            }
        }
    }
    let number_of_timelines: i64 = frontier.into_iter().map(|(k, v)| v).sum();
    println!("number of splits {}", number_of_splits);
    println!("number of timelines {}", number_of_timelines);
}
