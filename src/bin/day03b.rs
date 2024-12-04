const INPUT: &str = include_str!("../../data/day03.txt");

fn main() {
    let chars: Vec<char> = INPUT.chars().collect();

    let mut ignore = false;
    let mut position = 0;
    let mut sum: i32 = 0;

    loop {
        if position >= chars.len() {
            break;
        }

        if let Some(((left_num, right_num), read_position)) = read_mul(&chars, position) {
            if !ignore {
                sum += left_num * right_num;
            }

            position = read_position
        }
        if let Some((should_do, read_position)) = read_do(&chars, position) {
            ignore = !should_do;
            position = read_position
        } else {
            position += 1;
        }
    }

    println!("{}", sum);
}

fn read_do(chars: &Vec<char>, position: usize) -> Option<(bool, usize)> {
    if read_char(chars, position) != 'd' || read_char(chars, position + 1) != 'o' {
        return None;
    }

    if read_char(chars, position + 2) == '(' && read_char(chars, position + 3) == ')' {
        return Some((true, position + 3));
    }

    if read_char(chars, position + 2) == 'n'
        && read_char(chars, position + 3) == '\''
        && read_char(chars, position + 4) == 't'
        && read_char(chars, position + 5) == '('
        && read_char(chars, position + 6) == ')'
    {
        return Some((false, position + 6));
    }

    None
}

fn read_mul(chars: &Vec<char>, position: usize) -> Option<((i32, i32), usize)> {
    if read_char(chars, position) != 'm'
        || read_char(chars, position + 1) != 'u'
        || read_char(chars, position + 2) != 'l'
        || read_char(chars, position + 3) != '('
    {
        return None;
    }

    let (left_num, read_position) = read_number(chars, position + 4)?;

    if read_char(chars, read_position) != ',' {
        return None;
    }

    let (right_num, read_position) = read_number(chars, read_position + 1)?;

    if read_char(chars, read_position) != ')' {
        return None;
    }

    Some(((left_num, right_num), read_position))
}

fn read_number(chars: &Vec<char>, position: usize) -> Option<(i32, usize)> {
    let mut read_position = position;

    loop {
        if !read_char(chars, read_position).is_numeric() {
            break;
        }

        read_position += 1;
    }

    if read_position == position {
        return None;
    }

    Some((
        chars[position..read_position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap(),
        read_position,
    ))
}

fn read_char(chars: &Vec<char>, position: usize) -> char {
    if let Some(char) = chars.get(position) {
        return *char;
    } else {
        return '\0';
    }
}
