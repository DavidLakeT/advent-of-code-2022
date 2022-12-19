pub fn solve_part1() -> usize {
    let input = include_str!(
        r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day6.txt"
    );

    input
        .as_bytes()
        .windows(4)
        .position(|packer| {
            packer
                .iter()
                .enumerate()
                .all(|(index, char)| !packer[..index].contains(char))
        })
        .unwrap()
        + 4
}

pub fn solve_part2() -> usize {
    let input = include_str!(
            r"C:\Users\david\Desktop\Programacion\Rust\advent-of-code-2022\inputs\day6.txt"
    );

    input
        .as_bytes()
        .windows(14)
        .position(|packer| {
            packer
                .iter()
                .enumerate()
                .all(|(index, char)| !packer[..index].contains(char))
        })
        .unwrap()
    + 14
}