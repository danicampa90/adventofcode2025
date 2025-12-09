use libutils::map2d::{Map2D, map_from_lines};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_lines = input.lines();
    let mut map = map_from_lines(' ', input_lines).unwrap();
    println!("Loaded map {}x{}", map.size_x(), map.size_y());

    let reachable_coords = map.filter_coordinates(|map, (x, y)| {
        if map.get_value(x, y) != '@' {
            return false;
        }

        let mut count = 0;
        for dy in -1..2 {
            for dx in -1..2 {
                if map.get_value(x + dx, y + dy) == '@' {
                    count += 1
                }
            }
        }
        return count <= 4; // surrounded by 3, but we count also ourselves, so 4
    });

    println!(
        "There are {} rolls that are accessible",
        reachable_coords.len()
    );
    reachable_coords
        .into_iter()
        .for_each(|(x, y)| map.set_value(x, y, 'x'));

    map.debug_print()
}

fn part2() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let input_lines = input.lines();
    let mut map = map_from_lines(' ', input_lines).unwrap();
    println!("Loaded map {}x{}", map.size_x(), map.size_y());

    loop {
        let reachable_coords = map.filter_coordinates(|map, (x, y)| {
            if map.get_value(x, y) != '@' {
                return false;
            }

            let mut count = 0;
            for dy in -1..2 {
                for dx in -1..2 {
                    if map.get_value(x + dx, y + dy) == '@' {
                        count += 1
                    }
                }
            }
            return count <= 4; // surrounded by 3, but we count also ourselves, so 4
        });

        println!(
            "There are {} rolls that are accessible",
            reachable_coords.len()
        );
        if reachable_coords.len() == 0 {
            break;
        }
        reachable_coords
            .into_iter()
            .for_each(|(x, y)| map.set_value(x, y, 'x'));
    }
    map.debug_print();
    let removed = map.filter_values(|v| *v == 'x');
    println!("A total of {} rolls got removed", removed.len())
}
