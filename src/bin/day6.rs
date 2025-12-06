use std::{io::BufRead as _, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

#[derive(Debug)]
enum Ops {
    Addition,
    Multiplication,
}

impl FromStr for Ops {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Ops::Addition),
            "*" => Ok(Ops::Multiplication),
            _ => bail!("unsupported op"),
        }
    }
}

fn main() {
    let (nums, ops): (Vec<_>, Vec<_>) = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.split_whitespace().map(|s| s.to_string()).collect_vec()
        })
        .partition(|entry| {
            entry
                .first()
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .is_ascii_digit()
        });

    let nums = nums
        .iter()
        .map(|l| {
            l.iter()
                .map(|entry| entry.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let ops = ops
        .first()
        .iter()
        .map(|ops| {
            ops.iter()
                .map(|op| Ops::from_str(op).unwrap())
                .collect_vec()
        })
        .next()
        .unwrap();

    println!("{}", part1(&nums, &ops));
    println!("{}", part2(&nums, &ops));
}

fn part1(nums: &[Vec<u64>], ops: &[Ops]) -> u64 {
    let nums = transpose(nums.to_owned());
    nums.iter().enumerate().fold(0, |sum, (i, nums_single)| {
        sum + nums_single.iter().fold(0, |sum, num| match ops[i] {
            Ops::Addition => sum + num,
            Ops::Multiplication => sum.max(1) * num,
        })
    })
}
fn part2(_nums: &[Vec<u64>], _ops: &[Ops]) -> u64 {
    // need to handle it completely differently than part 1...
    0
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_fail() {
        assert!("X".parse::<Ops>().is_err());
    }
}
