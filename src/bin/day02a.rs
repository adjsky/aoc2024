const INPUT: &str = include_str!("../../data/day02.txt");

fn main() {
    let n_safe = INPUT
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse::<i32>().unwrap()))
        .filter(|r| {
            let a = r.clone().is_sorted_by(|a, b| a > b && (a - b).abs() <= 3);
            let b = r.clone().is_sorted_by(|a, b| a < b && (a - b).abs() <= 3);

            a || b
        })
        .count();

    println!("{}", n_safe);
}
