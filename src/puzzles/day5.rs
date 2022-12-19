use itertools::Itertools;

pub fn solve_part1() -> String {
    let input = include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day5.txt"
    );

    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (stacks_str, platforms_str) = stacks_input.rsplit_once('\n').unwrap();
    let stacks_amount = platforms_str
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks = vec![Vec::new(); stacks_amount];

    for line in stacks_str.lines().rev() {
        for (index, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1).unwrap();
            if second.is_alphabetic() {
                stacks[index].push(second);
            }
        }
    }

    let mut instructions = Vec::new();

    for line in instructions_input.lines() {
        let mut split = line.split_whitespace();
        let (_, amount, _, from, _, to) = (
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
        );

        let instruction = Instruction {
            amount: amount.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        };

        instructions.push(instruction);
    }

    for Instruction { amount, from, to } in instructions {
        for _ in 0..amount {
            if let Some(elements) = stacks[from].pop() {
                stacks[to].push(elements);
            }
        }
    }

    stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .collect()
}

pub fn solve_part2() -> String {
    let input = include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day5.txt"
    );

    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (stacks_str, platforms_str) = stacks_input.rsplit_once('\n').unwrap();
    let stacks_amount = platforms_str
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks = vec![Vec::new(); stacks_amount];

    for line in stacks_str.lines().rev() {
        for (index, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1).unwrap();
            if second.is_alphabetic() {
                stacks[index].push(second);
            }
        }
    }

    let mut instructions = Vec::new();

    for line in instructions_input.lines() {
        let mut split = line.split_whitespace();
        let (_, amount, _, from, _, to) = (
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(), 
        );

        let instruction = Instruction {
            amount: amount.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        };

        instructions.push(instruction);
    }

    for Instruction {
        amount,
        from,
        to,
    } in instructions
    {
        let origin_len = stacks[from].len();

        let elements = stacks[from].split_off(origin_len - amount);
        stacks[to].extend(elements);
    }

    stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .collect()
}

pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}
