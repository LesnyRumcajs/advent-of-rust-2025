use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

type Position = (i64, i64, i64);

fn part1(junctions: &[(i64, i64, i64)]) -> i64 {
    let mut circuits = Vec::new();

    for junction in junctions {
        let with_distances = junctions
            .iter()
            .filter(|x| *x != junction)
            .map(|x| {
                let min = std::cmp::min(x, junction);
                let max = std::cmp::max(x, junction);
                (min, max, squared_euclidean_distance(x, junction))
            })
            .sorted()
            .dedup()
            .collect_vec();

        circuits.extend(with_distances);
    }

    let mut separated: Vec<HashSet<Position>> = Vec::new();
    let mut connections = 0;
    for circuit in circuits.iter().sorted_by_key(|v| v.2).dedup() {
        let mut already_inserted = false;
        let mut need_merge = false;
        let mut insertion = (0, 0);
        for (i, separate) in separated.iter_mut().enumerate() {
            if separate.contains(circuit.0) || separate.contains(circuit.1) {
                let added1 = separate.insert(*circuit.0);
                let added2 = separate.insert(*circuit.1);

                // sanity check
                assert!(!(added1 && added2));

                if added1 || added2 {
                    // a box was already already_inserted to the circuit - need to merge
                    if already_inserted {
                        need_merge = true;
                        insertion = (insertion.0, i);
                        break;
                    // One of the boxes was connected to this circuit - adding it and we might need
                    // to merge
                    } else {
                        already_inserted = true;
                        connections += 1;
                        insertion = (i, 0);
                    }
                // All boxes already in this circuit, move to the next pair
                } else {
                    connections += 1;
                    already_inserted = true;
                    break;
                }
            }
        }

        if need_merge {
            let to_merge = separated.remove(insertion.1);
            separated.get_mut(insertion.0).unwrap().extend(to_merge);
        }

        if !already_inserted {
            connections += 1;
            separated.push(HashSet::from_iter([*circuit.0, *circuit.1]));
        }
        if connections >= 1000 {
            break;
        }
    }

    separated
        .iter()
        .map(|v| v.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>() as i64
}

fn part2(junctions: &[(i64, i64, i64)]) -> i64 {
    let mut circuits = Vec::new();
    let mut distinct_boxes = HashSet::new();

    for junction in junctions {
        distinct_boxes.insert(junction);
        let with_distances = junctions
            .iter()
            .filter(|x| *x != junction)
            .map(|x| {
                let min = std::cmp::min(x, junction);
                let max = std::cmp::max(x, junction);
                (min, max, squared_euclidean_distance(x, junction))
            })
            .sorted()
            .dedup()
            .collect_vec();

        circuits.extend(with_distances);
    }

    let mut separated: Vec<HashSet<Position>> = Vec::new();
    let mut result = 0;
    for circuit in circuits.iter().sorted_by_key(|v| v.2).dedup() {
        let mut already_inserted = false;
        let mut need_merge = false;
        let mut insertion = (0, 0);
        for (i, separate) in separated.iter_mut().enumerate() {
            if separate.contains(circuit.0) || separate.contains(circuit.1) {
                let added1 = separate.insert(*circuit.0);
                let added2 = separate.insert(*circuit.1);

                // sanity check
                assert!(!(added1 && added2));

                if added1 || added2 {
                    // a box was already inserted to the circuit - need to merge
                    if already_inserted {
                        need_merge = true;
                        insertion = (insertion.0, i);
                        break;
                    // One of the boxes was connected to this circuit - adding it and we might need
                    // to merge
                    } else {
                        already_inserted = true;
                        insertion = (i, 0);
                    }
                // All boxes already in this circuit, move to the next pair
                } else {
                    already_inserted = true;
                    break;
                }
            }
        }

        if need_merge {
            let to_merge = separated.remove(insertion.1);
            separated.get_mut(insertion.0).unwrap().extend(to_merge);
        }

        if !already_inserted {
            separated.push(HashSet::from_iter([*circuit.0, *circuit.1]));
        }

        if separated.len() == 1 && separated.first().unwrap().len() == distinct_boxes.len() {
            result = circuit.0.0 * circuit.1.0;
            break;
        }
    }
    result
}

fn squared_euclidean_distance(p: &Position, q: &Position) -> i64 {
    (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2)
}
