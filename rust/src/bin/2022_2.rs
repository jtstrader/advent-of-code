use std::collections::HashMap;

use advent_of_code::solve;
fn main() {
    solve(part_1, part_2);
}

fn part_1(input: &String) -> i32 {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|s| {
            s.trim()
                .split(' ')
                .map(|c| c.chars().next().unwrap())
                .collect()
        })
        .collect();

    let hit: HashMap<char, i32> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    let res: HashMap<char, i32> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

    let mut score = 0;

    for game in input {
        let (h, r) = (game[0], game[1]);
        let game_code = res[&r] - hit[&h];
        score += res[&r]
            + match game_code {
                -2 | 1 => 6,
                0 => 3,
                _ => 0,
            };
    }
    score
}

fn part_2(input: &String) -> i32 {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|s| {
            s.trim()
                .split(' ')
                .map(|c| c.chars().next().unwrap())
                .collect()
        })
        .collect();

    let hit: HashMap<char, i32> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    let win: HashMap<char, i32> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

    let mut score = 0;

    for game in input {
        let (h, r) = (game[0], game[1]);
        let win_status = win[&r];

        score += (win_status - 1) * 3
            + match win_status {
                1 => (hit[&h] - 2).rem_euclid(3) + 1,
                2 => hit[&h],
                _ => (hit[&h] % 3) + 1,
            };
    }
    score
}
