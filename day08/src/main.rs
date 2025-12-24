use libutils::connected_graph::ConnectionGraph;
use std::collections::{BinaryHeap, HashMap};

use crate::parser::parse_connection;

mod parser;

fn main() {
    let number_of_boxes_to_take = 1000;
    //////
    let mut input = vec![];
    libutils::read_file_foreach_line("input.txt", &mut |line| {
        input.push(parse_connection(&line).unwrap().1);
        Ok::<(), ()>(())
    })
    .unwrap();
    ////////// initialize

    let mut conn_graph = ConnectionGraph::new();
    let mut distances = HashMap::new();
    let mut sorted_distances = BinaryHeap::new();
    for box1 in 0..input.len() {
        conn_graph.connect(box1, box1); // make it appear in the list
        for box2 in box1 + 1..input.len() {
            let distance = input[box1].distance_to(&input[box2]);
            distances.insert((box1, box2), distance);
            sorted_distances.push((-distance, box1, box2));
        }
    }

    //////// part 1
    //println!("{:?}", sorted_distances);
    for _ in 0..number_of_boxes_to_take {
        let (_dist, box1, box2) = sorted_distances.pop().unwrap();
        conn_graph.connect(box1, box2);
        /* println!(
            "Connect (d={}) {} {} = ({:?}, {:?})",
            (-_dist as f64).sqrt(),
            box1,
            box2,
            input[box1],
            input[box2]
        ); */
        //let dist = (-dist as f64).sqrt();
    }
    let networks = conn_graph.all_networks();

    let mut net_sizes = vec![];
    for (netid, components) in networks.iter() {
        net_sizes.push(components.len());
    }
    net_sizes.sort();
    net_sizes.reverse();
    println!(
        "Biggest sizes: {} * {} * {} = {}",
        net_sizes[0],
        net_sizes[1],
        net_sizes[2],
        net_sizes[0] * net_sizes[1] * net_sizes[2]
    );

    let mut last_connection_box1 = usize::MAX;
    let mut last_connection_box2 = usize::MAX;

    while let Some((_dist, box1, box2)) = sorted_distances.pop() {
        if conn_graph.get_network_id(box1) == conn_graph.get_network_id(box2) {
            continue;
        }
        last_connection_box1 = box1;
        last_connection_box2 = box2;
        conn_graph.connect(box1, box2);
        /*println!(
            "Connect (d={}) {} {} = ({:?}, {:?})",
            (-_dist as f64).sqrt(),
            box1,
            box2,
            input[box1],
            input[box2]
        ); */
    }
    println!(
        "Last boxes {} {} = ({:?}, {:?})",
        last_connection_box1,
        last_connection_box2,
        input[last_connection_box1],
        input[last_connection_box2]
    );
    println!(
        "X coordinates multiplication: {}",
        input[last_connection_box1].x * input[last_connection_box2].x
    )
}
