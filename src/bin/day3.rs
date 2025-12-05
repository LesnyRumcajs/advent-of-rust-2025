use std::io::BufRead as _;

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(|ch| ch as u8 - b'0').collect_vec())
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[Vec<u8>]) -> i64 {
    input.iter().fold(0, |sum, bank| {
        let max_pos = bank.len() - bank.iter().rev().skip(1).position_max().unwrap() - 2;
        let max2 = bank[max_pos + 1..].iter().max().unwrap();
        sum + 10 * bank[max_pos] as i64 + *max2 as i64
    })
}

fn part2(input: &[Vec<u8>]) -> i64 {
    input.iter().fold(0, |sum, bank| {
        let mut max_pos_all = vec![];
        (0..12).rev().for_each(|rem| {
            let max_pos = bank.len()
                - bank
                    .iter()
                    .skip(max_pos_all.last().map(|x| x + 1).unwrap_or(0))
                    .rev()
                    .skip(rem)
                    .position_max()
                    .unwrap()
                - rem
                - 1;
            max_pos_all.push(max_pos);
        });

        sum + max_pos_all
            .iter()
            .map(|pos| char::from_digit(bank[*pos] as u32, 10).unwrap())
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    })
}
