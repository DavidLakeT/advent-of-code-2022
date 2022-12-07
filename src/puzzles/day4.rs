pub fn solve_part1() -> u64 {
    let mut overlapped_pairs = 0;

    for line in include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day4.txt"
    )
    .lines()
    {
        let (worker1, worker2) = line.split_once(',').unwrap();
        let (worker1_start, worker1_end) = parse_interval(worker1.split_once('-').unwrap());
        let (worker2_start, worker2_end) = parse_interval(worker2.split_once('-').unwrap());

        if worker1_start <= worker2_start && worker1_end >= worker2_end
            || worker2_start <= worker1_start && worker2_end >= worker1_end
        {
            overlapped_pairs += 1;
        }
    }

    overlapped_pairs
}

pub fn solve_part2() -> u64 {
    let mut overlapped_pairs = 0;

    for line in include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day4.txt"
    )
    .lines()
    {
        let (worker1, worker2) = line.split_once(',').unwrap();
        let (worker1_start, worker1_end) = parse_interval(worker1.split_once('-').unwrap());
        let (worker2_start, worker2_end) = parse_interval(worker2.split_once('-').unwrap());

        if worker1_start <= worker2_end && worker2_start <= worker1_end {
            overlapped_pairs += 1;
        }
    }

    overlapped_pairs
}

fn parse_interval(interval: (&str, &str)) -> (u64, u64) {
    let first = interval.0.parse::<u64>().unwrap();
    let second = interval.1.parse::<u64>().unwrap();

    (first, second)
}
