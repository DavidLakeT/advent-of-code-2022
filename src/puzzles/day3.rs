use itertools::Itertools;
use std::collections::HashMap;

fn create_priorities() -> HashMap<char, u64> {
    let mut map: HashMap<char, u64> = HashMap::with_capacity(52);

    for (index, char) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
    {
        map.insert(char, (index + 1) as u64);
    }

    map
}

pub fn solve_part1() -> u64 {
    let mut priorities_sum = 0;
    let priorities_map = create_priorities();

    'line_iter: for line in include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day3.txt"
    )
    .lines()
    {
        let split_size = line.len() / 2;
        let split = line.split_at(split_size);
        let mut char_map: HashMap<char, u64> = HashMap::with_capacity(split_size);

        for char in split.0.chars() {
            match char_map.get(&char) {
                Some(_) => {}
                None => {
                    char_map.insert(char, 1);
                }
            }
        }

        for char in split.1.chars() {
            match char_map.get(&char) {
                Some(value) => {
                    if *value == 1 {
                        priorities_sum += priorities_map.get(&char).unwrap();
                        continue 'line_iter;
                    }
                }
                None => {}
            }
        }
    }

    priorities_sum
}

pub fn solve_part2() -> u64 {
    let mut priorities_sum = 0;
    let priorities_map = create_priorities();

    'group_iter: for (line1, line2, line3) in include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day3.txt"
    )
    .lines()
    .tuples()
    {
        let mut char_map: HashMap<char, u64> = HashMap::new();

        for char in line1.chars() {
            match char_map.get(&char) {
                Some(_) => {}
                None => {
                    char_map.insert(char, 1);
                }
            }
        }

        for char in line2.chars() {
            match char_map.get(&char) {
                Some(value) => {
                    if *value == 1 {
                        char_map.insert(char, value + 1);
                    }
                }
                None => {}
            }
        }

        for char in line3.chars() {
            match char_map.get(&char) {
                Some(value) => {
                    if *value == 2 {
                        priorities_sum += priorities_map.get(&char).unwrap();
                        continue 'group_iter;
                    }
                }
                None => {}
            }
        }
    }

    priorities_sum
}
