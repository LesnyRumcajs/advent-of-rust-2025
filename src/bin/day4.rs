use std::io::BufRead as _;

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(|ch| ch == '@').collect_vec())
        .collect_vec();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &[Vec<bool>]) -> usize {
    let grid = Grid::from_input(input.to_vec());
    grid.count_with_few_adjacent(4)
}
fn part2(input: &[Vec<bool>]) -> usize {
    let mut grid = Grid::from_input(input.to_vec());
    grid.mutate_count_with_few_adjacent(4)
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn from_input(grid: Vec<Vec<bool>>) -> Self {
        Grid { grid }
    }

    fn count_with_few_adjacent(&self, threshold: usize) -> usize {
        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _col) in row.iter().enumerate() {
                if self.grid[y][x] && self.count_rolls_around(y, x) < threshold {
                    count += 1;
                }
            }
        }
        count
    }

    fn mutate_count_with_few_adjacent(&mut self, threshold: usize) -> usize {
        let mut count = 0;
        loop {
            let mut to_remove = vec![];
            for (y, row) in self.grid.iter().enumerate() {
                for (x, _col) in row.iter().enumerate() {
                    if self.grid[y][x] && self.count_rolls_around(y, x) < threshold {
                        to_remove.push((y, x));
                    }
                }
            }

            if to_remove.is_empty() {
                break;
            }

            count += to_remove.len();
            while let Some((y, x)) = to_remove.pop() {
                self.grid[y][x] = false;
            }
        }
        count
    }

    fn count_rolls_around(&self, y: usize, x: usize) -> usize {
        let y = y as i32;
        let x = x as i32;
        let to_check = [
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y, x - 1),
            (y, x + 1),
            (y + 1, x - 1),
            (y + 1, x),
            (y + 1, x + 1),
        ];
        to_check
            .iter()
            .filter(|(y, x)| self.check_pos(*y, *x))
            .count()
    }

    fn check_pos(&self, y: i32, x: i32) -> bool {
        if y < 0 || x < 0 {
            return false;
        }

        let y = y as usize;
        let x = x as usize;
        let max_y = self.grid.len() - 1;
        let max_x = self.grid.first().unwrap().len() - 1;
        x <= max_x && y <= max_y && self.grid[y][x]
    }
}
