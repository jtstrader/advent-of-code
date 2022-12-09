use advent_of_code::solve;

fn main() {
    solve(part_1, part_2);
}

fn part_1(input: &String) -> usize {
    let pairs: Vec<((usize, usize), (usize, usize))> = input
        .lines()
        .flat_map(|line| line.split_once(','))
        .map(|pair| (pair.0.split_once('-'), pair.1.split_once('-')))
        .map(|(pair1, pair2)| {
            let tuple_conv = |t: (&str, &str)| -> (usize, usize) {
                (t.0.parse().unwrap(), t.1.parse().unwrap())
            };
            let pair1 = pair1.expect("input should be properly formatted");
            let pair2 = pair2.expect("input should be properly formatted");
            (tuple_conv(pair1), tuple_conv(pair2))
        })
        .collect();

    let mut count = 0;
    for (p1, p2) in pairs {
        if (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1) {
            count += 1;
        }
    }

    count
}

fn part_2(input: &String) -> usize {
    let pairs: Vec<((usize, usize), (usize, usize))> = input
        .lines()
        .flat_map(|line| line.split_once(','))
        .map(|pair| (pair.0.split_once('-'), pair.1.split_once('-')))
        .map(|(pair1, pair2)| {
            let tuple_conv = |t: (&str, &str)| -> (usize, usize) {
                (t.0.parse().unwrap(), t.1.parse().unwrap())
            };
            let pair1 = pair1.expect("input should be properly formatted");
            let pair2 = pair2.expect("input should be properly formatted");
            (tuple_conv(pair1), tuple_conv(pair2))
        })
        .collect();

    let contained = |n: usize, l: usize, u: usize| n >= l && n <= u;

    let mut count = 0;
    for (p1, p2) in pairs {
        if contained(p1.0, p2.0, p2.1)
            || contained(p1.1, p2.0, p2.1)
            || contained(p2.0, p1.0, p1.1)
            || contained(p2.1, p1.0, p1.1)
        {
            count += 1;
        }
    }

    count
}
