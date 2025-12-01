use std::{io::BufRead as _, str::FromStr};

use anyhow::Context;
use itertools::Itertools;

enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).context("must be of size 1")? {
            'L' => Ok(Dir::Left),
            'R' => Ok(Dir::Right),
            _ => Err(anyhow::Error::msg("fiasco")),
        }
    }
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (dir, count) = l.split_at(1);
            (dir.parse::<Dir>().unwrap(), count.parse::<i32>().unwrap())
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[(Dir, i32)]) -> i32 {
    input
        .iter()
        .fold((50, 0), |(mut pos, mut total_zeroes), (dir, count)| {
            pos += match dir {
                Dir::Left => -count,
                Dir::Right => *count,
            };

            pos = pos.rem_euclid(100);
            if pos == 0 {
                total_zeroes += 1
            }
            (pos, total_zeroes)
        })
        .1
}

fn part2(input: &[(Dir, i32)]) -> i32 {
    input
        .iter()
        .fold((50, 0), |(mut pos, mut total_zeroes), (dir, count)| {
            let turn = match dir {
                Dir::Left => -count,
                Dir::Right => *count,
            };

            total_zeroes += if pos == 0 {
                turn.abs() / 100
            } else if pos + turn <= 0 {
                1 + (turn.abs() - pos) / 100
            } else {
                (pos + turn) / 100
            };

            pos = (pos + turn).rem_euclid(100);

            (pos, total_zeroes)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_fail() {
        assert!("X".parse::<Dir>().is_err());
    }
}
