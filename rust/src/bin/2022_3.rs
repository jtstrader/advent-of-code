use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let split_rucksack = read_data_part_1()?;
    let rucksack_threes = read_data_part_2()?;

    println!("Part 1 Answer: {}", part_1(split_rucksack));

    println!("{:#?}", rucksack_threes);

    Ok(())
}

fn part_1(rucksack: Vec<(String, String)>) -> usize {
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

fn read_data_part_1() -> Result<Vec<(String, String)>> {
    let mut s = String::new();
    let mut f = File::open("./src/data/2022input3.txt")?;
    f.read_to_string(&mut s)?;

    Ok(s.split('\n')
        .map(|x| {
            (
                x[..x.len() / 2].to_string(),
                x[x.len() / 2..].trim().to_string(),
            )
        })
        .collect())
}

fn read_data_part_2() -> Result<Vec<(String, String, String)>> {
    let mut v: Vec<(String, String, String)> = Vec::new();
    let mut s = String::new();
    let mut f = File::open("./src/data/2022input3.txt")?;
    f.read_to_string(&mut s)?;

    let mut iter = s.split('\n');
    while let Some(s1) = iter.next() {
        v.push((
            s1.trim().to_owned(),
            iter.next().unwrap().trim().to_owned(),
            iter.next().unwrap().trim().to_owned(),
        ));
    }

    Ok(v)
}
