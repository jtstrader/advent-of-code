use advent_of_code::solve;

fn main() {
    solve(part_1, part_2);
}

fn part_1(input: &String) -> usize {
    let (mut elf, mut max_elf) = (0, 0);

    for s in input.split('\n') {
        let j = s.trim();
        if j.is_empty() {
            max_elf = usize::max(elf, max_elf);
            elf = 0;
        } else {
            elf += j.parse::<usize>().unwrap();
        }
    }

    if elf != 0 {
        max_elf = usize::max(elf, max_elf);
    }

    max_elf
}

fn part_2(input: &String) -> usize {
    let mut v: Vec<usize> = Vec::new();
    let mut elf = 0;

    for s in input.split('\n') {
        let j = s.trim();
        if j.is_empty() {
            v.push(elf);
            elf = 0;
        } else {
            elf += j.parse::<usize>().unwrap();
        }
    }

    if elf != 0 {
        v.push(elf);
    }

    v.sort_by(|a, b| b.cmp(a));
    v[..3].iter().sum()
}
