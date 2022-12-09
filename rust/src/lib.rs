use std::io::Result;

/// Run Advent of Code for both parts
pub fn solve<F1, F2, R1, R2>(part1: F1, part2: F2)
where
    F1: Fn(&String) -> R1,
    F2: Fn(&String) -> R2,
    R1: std::fmt::Display,
    R2: std::fmt::Display,
{
    let input = read_data().expect("input file should be present before attempting to run");
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

/// Load input from a file. Files follow a certain path based on the name of the binary
fn read_data<'a>() -> Result<String> {
    // Runs on Windows, use backslash;
    std::fs::read_to_string(
        || -> Option<String> {
            Some(
                std::env::args()
                    .next()?
                    .split('\\')
                    .last()?
                    .split_once('.')?
                    .0
                    .split_once('_')
                    .map(|x| format!(".\\src\\data\\{}\\{}.txt", x.0, x.1))?,
            )
        }()
        .expect("solution file should be formatted YEAR_P#.rs"),
    )
}
