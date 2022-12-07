use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let input = read_data()?;

    println!("Part 1 Answer: {}", part_1(&input));
    println!("Part 2 Answer: {}", part_2(&input));

    Ok(())
}

fn part_1(input: &Vec<Vec<char>>) -> i32 {
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

fn part_2(input: &Vec<Vec<char>>) -> i32 {
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

fn read_data() -> Result<Vec<Vec<char>>> {
    let mut lines = String::new();
    let mut f = File::open("./src/data/2022input2.txt")?;
    f.read_to_string(&mut lines)?;

    Ok(lines
        .split('\n')
        .map(|s| {
            s.trim()
                .split(' ')
                .map(|c| c.chars().next().unwrap())
                .collect()
        })
        .collect())
}
