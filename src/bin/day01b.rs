use std::collections::HashMap;

const INPUT: &str = include_str!("../../data/day01.txt");

fn main() {
    let (left, right): (Vec<i32>, Vec<i32>) = INPUT
        .lines()
        .map(|l| {
            let ns: Vec<i32> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();

            (ns[0], ns[1])
        })
        .unzip();

    let rmap = right.iter().fold(HashMap::new(), |mut m, n| {
        *m.entry(n).or_default() += 1;
        m
    });

    let sum: i32 = left.iter().map(|n| n * rmap.get(&n).unwrap_or(&0)).sum();

    println!("{}", sum);
}
