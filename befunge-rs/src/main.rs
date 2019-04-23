fn main() {
    let cmd = String::from("64+\"!dlroW ,olleH\">:#,_@");
    let mut stack: Vec<u32> = Vec::new();
    let mut index: usize = 0;
    let mut quote_mode: bool = false;
    let mut pc_direction: u32 = 1; // 1: right 2: left 3: up 4: down

    while index < cmd.len() {
        let ch = cmd.as_bytes()[index] as char;
        if quote_mode {
            if ch != '"' {
                stack.push(ch as u32);
            } else {
                quote_mode = false;
            }
            index += 1;
            continue;
        }
        match ch {
            '0' ... '9' => stack.push(ch.to_digit(10).unwrap()),
            '+' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(a + b)
            },
            '"' => {
                quote_mode = true;
            },
            '>' => {
                pc_direction = 1;
            },
            '<' => {
                pc_direction = 2;
            },
            ':' => {
                let n = stack.pop().unwrap_or(0);
                stack.push(n);
                stack.push(n);
            },
            '#' => {
                if pc_direction == 1 {
                    index += 1
                } else if pc_direction == 2 {
                    index -= 1
                }
            },
            '.' => {
                let n = stack.pop().unwrap();
                print!("{}", n);
            }
            ',' => {
                let c = stack.pop().unwrap() as u8 as char;
                print!("{}", c);
            },
            '_' => {
                let n = stack.pop().unwrap();
                if n == 0 {
                    pc_direction = 1;
                } else {
                    pc_direction = 2;
                }
            },
            '@' => {
                break;
            }
            _ => (),
        }
        if pc_direction == 1 {
            index += 1
        } else if pc_direction == 2 {
            index -= 1
        }
    }
}
