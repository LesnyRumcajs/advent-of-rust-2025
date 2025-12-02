use std::{io::BufRead as _, ops::RangeInclusive};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|s| {
                    let (start, end) = s.split_once('-').unwrap();
                    let (start, end) = (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap());
                    RangeInclusive::new(start, end)
                })
                .collect_vec()
        })
        .next()
        .unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[RangeInclusive<i64>]) -> i64 {
    input.iter().fold(0, |mut sum, range| {
        for i in range.clone() {
            let s = i.to_string();
            if s.len() % 2 != 0 {
                continue;
            }

            let (begin, end) = s.split_at(s.len() / 2);
            if begin == end {
                sum += i;
            }
        }
        sum
    })
}

fn part2(input: &[RangeInclusive<i64>]) -> i64 {
    input.iter().fold(0, |mut sum, range| {
        for i in range.clone() {
            let s = i.to_string();

            for j in 0..(s.len() / 2) {
                let pattern = &s[0..=j];
                if (s.len() - pattern.len()) % pattern.len() == 0
                    && pattern.repeat((s.len() - pattern.len()) / pattern.len() + 1) == s
                {
                    sum += i;
                    break;
                }
            }
        }
        sum
    })
}
