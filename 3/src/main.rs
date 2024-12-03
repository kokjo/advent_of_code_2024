fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut state = 0;
    let mut x = 0;
    let mut y = 0;
    let mut acc = 0;
    for char in input.chars() {
        state = match state {
            0 => if char == 'm' { x = 0; y = 0; 1 } else if char == 'd' { 6 } else { 0 }, // part 2
            // 0 => if char == 'm' { x = 0; y = 0; 1 } else { 0 }, // part 1
            1 => if char == 'u' { 2 } else { 0 },
            2 => if char == 'l' { 3 } else { 0 },
            3 => if char == '(' { 4 } else { 0 },
            4 => if let Some(d) = char.to_digit(10) { x = 10*x + d; 4 } else if char == ',' { 5 } else { 0 },
            5 => if let Some(d) = char.to_digit(10) { y = 10*y + d; 5 } else if char == ')' { acc += x * y; 0 } else { 0 },
            6 => if char == 'o' { 7 } else { 0 },
            7 => if char == 'n' { 8 } else { 0 },
            8 => if char == '\'' { 9 } else { 0 },
            9 => if char == 't' { 10 } else { 0 },
            10 => if char == '(' { 11 } else { 0 },
            11 => if char == ')' { 12 } else { 0 },
            12 => if char == 'd' { 13 } else { 12 },
            13 => if char == 'o' { 14 } else { 12 },
            14 => if char == '(' { 15 } else { 12 },
            15 => if char == ')' { 0 } else { 12 },
            _ => panic!("invalid state"),
        }
    }
    println!("acc = {}", acc);
}
