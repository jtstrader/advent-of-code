use advent_of_code::solve;
use std::collections::{HashMap, HashSet};

fn main() {
    solve(part_1, part_2);
}

fn part_1(input: &String) -> usize {
    let rucksack: Vec<(String, String)> = input
        .lines()
        .map(|x| {
            (
                x[..x.len() / 2].to_string(),
                x[x.len() / 2..].trim().to_string(),
            )
        })
        .collect();

    // Try using hash map first
    let prio: HashMap<char, usize> = HashMap::from_iter(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i + 1)),
    );

    let mut count = 0;
    for r in rucksack.iter() {
        let mut c_set1: [usize; 52] = [0; 52];
        let mut c_set2: [usize; 52] = [0; 52];
        for (c1, c2) in r.0.chars().zip(r.1.chars()) {
            c_set1[prio[&c1] - 1] += 1;
            c_set2[prio[&c2] - 1] += 1;
        }

        for (idx, (i1, i2)) in c_set1.iter().zip(c_set2.iter()).enumerate() {
            if *i1 > 0 && *i2 > 0 {
                count += idx + 1;
                break;
            }
        }
    }

    count
}

fn part_2(input: &String) -> usize {
    let rucksack: Vec<(String, String, String)> = {
        let mut v: Vec<(String, String, String)> = Vec::new();
        let mut iter = input.lines();
        while let Some(s1) = iter.next() {
            v.push((
                s1.trim().to_owned(),
                iter.next().unwrap().trim().to_owned(),
                iter.next().unwrap().trim().to_owned(),
            ));
        }
        v
    };

    let mut count: usize = 0;
    for (r1, r2, r3) in rucksack {
        let (s1, s2, s3) = (
            HashSet::<char>::from_iter(r1.chars()),
            HashSet::<char>::from_iter(r2.chars()),
            HashSet::<char>::from_iter(r3.chars()),
        );

        for s in s1 {
            if s2.contains(&s) && s3.contains(&s) {
                count += match s.is_uppercase() {
                    true => s as usize - 'A' as usize + 27,
                    false => s as usize - 'a' as usize + 1,
                };
            }
        }
    }

    count
}
