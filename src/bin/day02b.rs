const INPUT: &str = include_str!("../../data/day02.txt");

fn main() {
    let n_safe = INPUT
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()))
        .filter(|r| {
            let a = is_record_safe(r.clone().collect(), Order::Ascending, 0, false);
            let b = is_record_safe(r.clone().collect(), Order::Descending, 0, false);

            a || b
        })
        .count();

    println!("{}", n_safe);
}

#[derive(Clone, Copy)]
enum Order {
    Ascending,
    Descending,
}

fn is_record_safe(mut r: Vec<i32>, order: Order, check_at: usize, is_skipped: bool) -> bool {
    if check_at == r.len() - 1 {
        return true;
    }

    let is_safe = check_levels(r[check_at], r[check_at + 1], order);

    if !is_safe {
        if is_skipped {
            return false;
        }

        if check_at < r.len() - 2 && check_levels(r[check_at + 1], r[check_at + 2], order) {
            r.remove(check_at);

            return is_record_safe(r, order, if check_at == 0 { 0 } else { check_at - 1 }, true);
        }

        r.remove(check_at + 1);

        return is_record_safe(r, order, check_at, true);
    }

    return is_record_safe(r, order, check_at + 1, is_skipped);
}

fn check_levels(a: i32, b: i32, order: Order) -> bool {
    let cmp = match order {
        Order::Ascending => |a, b| a < b,
        Order::Descending => |a, b| a > b,
    };

    let diff = (a - b).abs();

    if diff > 3 {
        return false;
    }

    return cmp(a, b);
}
