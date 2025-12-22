use std::{collections::HashSet, mem};

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
    let mut frontier = HashSet::new();
    frontier.insert(*start_coordinate);

    let mut number_of_splits = 0;

    for _steps in 0..map.size_y() {
        let mut old_frontier = HashSet::new();
        mem::swap(&mut frontier, &mut old_frontier);
        for (f_x, f_y) in old_frontier {
            if map.get_value(f_x, f_y + 1) == '^' {
                frontier.insert((f_x - 1, f_y + 1));
                frontier.insert((f_x + 1, f_y + 1));
                number_of_splits += 1;
            } else {
                frontier.insert((f_x, f_y + 1));
            }
        }
    }
    println!("number_of_splits {}", number_of_splits);
}
