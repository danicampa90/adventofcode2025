use libutils::map2d::{Map2D, map_from_lines};

use crate::parser::{parse_line, parse_line_part2};

mod parser;

#[derive(Debug, Copy, Clone)]
pub enum NumberOrOperator {
    Number(i64),
    Operator(char),
}

fn part1() {
    let mut map = None;

    libutils::read_file_foreach_line("input.txt", &mut |line: String| {
        let (_rest, line) = parse_line(&line).map_err(|e| e.to_string())?;
        if map.is_none() {
            map = Some(Map2D::new(NumberOrOperator::Number(0), line.len()));
        }
        map.as_mut().unwrap().add_row(line);
        Ok::<(), String>(())
    })
    .unwrap();

    println!("{:?}", map);
    let map = map.unwrap().transposed();
    println!("{:?}", map);
    let mut total = 0;
    for column in map.iter_rows() {
        if column.last().is_none() {
            continue;
        }
        let numbers: Vec<_> = column[0..column.len() - 1]
            .iter()
            .map(|num| match num {
                NumberOrOperator::Number(n) => n,
                other => panic!("Found {:?} when expecting number", other),
            })
            .collect();

        let result: i64 = match column.last().unwrap() {
            NumberOrOperator::Operator('*') => numbers.iter().cloned().product(),
            NumberOrOperator::Operator('+') => numbers.iter().cloned().sum(),
            other => panic!("Expecting +/*, found {:?} instead", other),
        };

        total += result;

        println!(
            "Result of {:?} for {:?} :{:?}",
            column.last().unwrap(),
            numbers,
            result
        );
    }
    println!("TOTAL: {}", total);
}

fn part2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "Lines sizes (need to match) {:?}",
        input.lines().map(|l| l.len()).collect::<Vec<_>>()
    );
    let input_lines = input.lines();
    let map = map_from_lines(' ', input_lines).unwrap();
    let map = map.rotateccw();
    let mut number_stack = vec![];
    let mut grand_total = 0;
    for row in map.iter_rows() {
        let line = String::from_iter(row);
        let line = line.trim();
        println!("{}", line);
        if line.len() > 0 {
            let ops = &mut parse_line_part2(&line).unwrap().1;
            for op in ops {
                match op {
                    NumberOrOperator::Number(n) => number_stack.push(*n),
                    NumberOrOperator::Operator('*') => {
                        grand_total += number_stack.into_iter().product::<i64>();
                        number_stack = vec![];
                    }
                    NumberOrOperator::Operator('+') => {
                        grand_total += number_stack.into_iter().sum::<i64>();
                        number_stack = vec![];
                    }
                    _ => panic!("Unknown operator"),
                }
            }
        }
    }
    println!("Part 2 grand total: {}", grand_total);
}

fn main() {
    part1();
    part2();
}
