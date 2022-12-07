use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut elf_info = get_elf_info()?;
    elf_info.sort_by(|a, b| b.cmp(a));

    println!("Part 1 Answer: {}", elf_info[0]);
    println!("Part 2 Answer: {:?}", &elf_info[0..3].iter().sum::<i32>());

    Ok(())
}

fn get_elf_info() -> Result<Vec<i32>> {
    let mut elf = 0;
    let mut v: Vec<i32> = Vec::new();
    let mut f = File::open("./src/data/2022input1.txt")?;
    let mut lines = String::new();
    f.read_to_string(&mut lines)?;

    for s in lines.split('\n') {
        let j = s.trim();
        if j.len() == 0 {
            v.push(elf);
            elf = 0;
        } else {
            elf += j.parse::<i32>().unwrap();
        }
    }

    if elf != 0 {
        v.push(elf);
    }

    Ok(v)
}
