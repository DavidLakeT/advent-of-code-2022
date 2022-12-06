use itertools::{FoldWhile, Itertools};
use std::cmp::Reverse;

pub fn solve_part1() -> u64 {
    include_str!(r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day1.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max()
        .unwrap_or(0)
}

pub fn solve_part2() -> u64 {
    include_str!(r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day1.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            it.fold_while(None, |acc: Option<u64>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>()
}
