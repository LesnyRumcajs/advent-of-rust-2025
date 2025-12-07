use std::{collections::HashSet, io::BufRead as _};

use anyhow::bail;
use itertools::Itertools;

enum Object {
    Space,
    Splitter,
    Start,
}

impl TryFrom<char> for Object {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Object::Space,
            'S' => Object::Start,
            '^' => Object::Splitter,
            _ => bail!("unsupported obj"),
        })
    }
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|ch| Object::try_from(ch).unwrap())
                .collect_vec()
        })
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2());
}

fn part1(input: &[Vec<Object>]) -> usize {
    let mut split_count = 0;
    let mut beams: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in input.iter().enumerate() {
        for (x, obj) in row.iter().enumerate() {
            match obj {
                Object::Space => {
                    if y > 0 && beams.contains(&(y - 1, x)) {
                        beams.insert((y, x));
                    }
                }
                Object::Start => {
                    beams.insert((y, x));
                }
                Object::Splitter => {
                    if beams.contains(&(y - 1, x)) {
                        split_count += 1;
                        beams.insert((y, x - 1));
                        beams.insert((y, x + 1));
                    }
                }
            }
        }
    }
    split_count
}
fn part2() -> i32 {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_from_obj() {
        assert!(matches!(Object::try_from('.').unwrap(), Object::Space));
        assert!(matches!(Object::try_from('S').unwrap(), Object::Start));
        assert!(matches!(Object::try_from('^').unwrap(), Object::Splitter));
        assert!(Object::try_from('X').is_err());
    }
}
