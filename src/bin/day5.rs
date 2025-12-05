use std::{io::BufRead as _, ops::RangeInclusive};

use itertools::Itertools;

fn main() {
    let (ranges, ids): (Vec<_>, Vec<_>) = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|entry| !entry.is_empty())
        .partition(|entry| entry.contains('-'));

    let ranges = ranges
        .iter()
        .map(|range| {
            let (begin, end) = range.split_once('-').unwrap();
            RangeInclusive::new(
                begin.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect_vec();

    let ids = ids
        .iter()
        .map(|id| id.parse::<usize>().unwrap())
        .collect_vec();

    println!("{}", part1(&ranges, &ids));
    println!("{}", part2(&ranges));
}

fn part1(ranges: &[RangeInclusive<usize>], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}
fn part2(ranges: &[RangeInclusive<usize>]) -> usize {
    let ranges = ranges
        .iter()
        .sorted_by_key(|range| range.start())
        .cloned()
        .collect_vec();

    let mut merged: Vec<RangeInclusive<usize>> = Vec::new();

    for range in ranges.into_iter() {
        let (start, end) = range.into_inner();
        if merged.is_empty() {
            merged.push(RangeInclusive::new(start, end));
        } else {
            let last = merged.pop().unwrap();
            // there's an overlap
            if start <= last.end() + 1 {
                let new_end = std::cmp::max(last.end(), &end);
                merged.push(RangeInclusive::new(*last.start(), *new_end));
            // no overlap, push the new separate range
            } else {
                merged.push(last);
                merged.push(RangeInclusive::new(start, end));
            }
        }
    }

    merged
        .iter()
        .fold(0, |sum, range| sum + range.end() - range.start() + 1)
}
