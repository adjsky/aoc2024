const INPUT: &str = include_str!("../../data/day01.txt");

fn main() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = INPUT
        .lines()
        .map(|l| {
            let ns: Vec<i32> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();

            (ns[0], ns[1])
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let sum: i32 = (0..left.len())
        .map(|idx| (left[idx] - right[idx]).abs())
        .sum();

    println!("{}", sum);
}
