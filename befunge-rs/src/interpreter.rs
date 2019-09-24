enum PcDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Coordinate {
    x: i8,
    y: i8,
}

pub struct Environment {
    stack: Vec<i8>,
    coordinate: Coordinate,
    dx: i8,
    dy: i8,
    string_mode: bool,
    pc_direction: PcDirection,
    commands: Vec<String>,
    output: String,
}

pub trait Interpreter {
    fn new() -> Self;
    fn add(&mut self);
    fn subtract(&mut self);
    fn multiply(&mut self);
    fn integer_division(&mut self);
    fn modulo(&mut self);
    fn logical_not(&mut self);
    fn greater_than(&mut self);
    fn duplicate(&mut self);
    fn print_int(&mut self);
    fn print_char(&mut self);
    fn enter_string_mode(&mut self);
    fn exit_string_mode(&mut self);
    fn nop(&mut self);
    fn step(&mut self);
    fn horizontal_if(&mut self);
    fn get(&mut self);
    fn run(&mut self, command: &str) -> &str;
}

impl Interpreter for Environment {
    fn new() -> Environment {
        Environment {
            stack: Vec::new(),
            coordinate: Coordinate { x: 0, y: 0 },
            dx: 0,
            dy: 0,
            string_mode: false,
            pc_direction: PcDirection::Right,
            commands: vec![String::from("")],
            output: String::from(""),
        }
    }

    fn add(&mut self) {
        let a: Option<i8> = self.stack.pop();
        let b: Option<i8> = self.stack.pop();

        match (a, b) {
            (Some(x), Some(y)) => self.stack.push(x + y),
            _ => (),
        }
    }

    fn subtract(&mut self) {
        let a: Option<i8> = self.stack.pop();
        let b: Option<i8> = self.stack.pop();

        match (a, b) {
            (Some(x), Some(y)) => self.stack.push(y - x),
            _ => (),
        }
    }

    fn multiply(&mut self) {
        let a: Option<i8> = self.stack.pop();
        let b: Option<i8> = self.stack.pop();

        match (a, b) {
            (Some(x), Some(y)) => self.stack.push(x * y),
            _ => (),
        }
    }

    fn integer_division(&mut self) {
        let a: Option<i8> = self.stack.pop();
        let b: Option<i8> = self.stack.pop();

        match (a, b) {
            (Some(x), Some(y)) => self.stack.push(y / x),
            _ => (),
        }
    }

    fn modulo(&mut self) {
        let a: Option<i8> = self.stack.pop();
        let b: Option<i8> = self.stack.pop();

        match (a, b) {
            (Some(x), Some(y)) => self.stack.push(y % x),
            _ => (),
        }
    }

    fn logical_not(&mut self) {
        let a: Option<i8> = self.stack.pop();

        match a {
            Some(x) => self.stack.push(if x == 0 { 1 } else { 0 }),
            _ => (),
        }
    }

    fn greater_than(&mut self) {
        let a: Option<i8> = self.stack.pop();
        let b: Option<i8> = self.stack.pop();

        match (a, b) {
            (Some(x), Some(y)) => self.stack.push(if y > x { 1 } else { 0 }),
            _ => (),
        }
    }

    fn duplicate(&mut self) {
        let n: Option<i8> = self.stack.pop();

        match n {
            Some(x) => {
                self.stack.push(x);
                self.stack.push(x);
            },
            _ => (),
        }
    }

    fn print_int(&mut self) {
        let n: Option<i8> = self.stack.pop();

        match n {
            Some(x) => {
                print!("{}", x);
                self.output.push_str(x.to_string().as_str());
            },
            _ => (),
        }
    }

    fn print_char(&mut self) {
        let n: Option<i8> = self.stack.pop();

        match n {
            Some(x) => {
                let c = x as u8 as char;
                print!("{}", c);
                self.output.push(c);
            },
            _ => (),
        }
    }

    fn enter_string_mode(&mut self) {
        self.string_mode = true;
    }

    fn exit_string_mode(&mut self) {
        self.string_mode = false;
    }

    fn nop(&mut self) {
        self.step();
    }

    fn step(&mut self) {
        match self.pc_direction {
            PcDirection::Left => {
                self.dx = -1;
                self.dy = 0;
            },
            PcDirection::Right => {
                self.dx = 1;
                self.dy = 0;
            },
            PcDirection::Up => {
                self.dx = 0;
                self.dy = -1;
            },
            PcDirection::Down => {
                self.dx = 0;
                self.dy = 1;
            }
        }
        self.coordinate.x += self.dx;
        self.coordinate.y += self.dy;
    }

    fn horizontal_if(&mut self) {
        let n: Option<i8> = self.stack.pop();

        match n {
            Some(x) => {
                self.pc_direction = if x == 0 { PcDirection::Right } else {PcDirection::Left };
            },
            _ => (),
        }
    }

    fn get(&mut self) {
        let y: Option<i8> = self.stack.pop();
        let x: Option<i8> = self.stack.pop();

        match (x, y) {
            (Some(a), Some(b)) => {
                let ch = self.commands
                    .get(b as usize)
                    .unwrap()
                    .as_bytes()[a as usize] as char;

                self.stack.push(ch as i8);
            },
            _ => (),
        }
    }

    fn run(&mut self, command: &str) -> &str {
        self.commands = command.to_string()
            .split('\n')
            .map(String::from)
            .collect();

        loop {
            let ch = self.commands
                .get(self.coordinate.y as usize)
                .unwrap()
                .as_bytes()[self.coordinate.x as usize] as char;

            if self.string_mode {
                if ch != '"' {
                    self.stack.push(ch as i8);
                } else {
                    self.exit_string_mode();
                }
                self.step();
                continue;
            }

            match ch {
                '0'...'9' => self.stack.push(ch.to_digit(10).unwrap() as i8),
                '+' => {
                    self.add();
                },
                '"' => {
                    self.enter_string_mode();
                },
                '-' => {
                    self.subtract();
                },
                '*' => {
                    self.multiply();
                },
                '/' => {
                    self.integer_division();
                },
                '%' => {
                    self.modulo();
                },
                '!' => {
                    self.logical_not();
                },
                '`' => {
                    self.greater_than();
                },
                '>' => {
                    self.pc_direction = PcDirection::Right;
                },
                '<' => {
                    self.pc_direction = PcDirection::Left;
                },
                '^' => {
                    self.pc_direction = PcDirection::Up;
                },
                'v' => {
                    self.pc_direction = PcDirection::Down;
                },
                ':' => {
                    self.duplicate();
                },
                '#' => {
                    self.nop();
                },
                '.' => {
                    self.print_int();
                }
                ',' => {
                    self.print_char();
                },
                '_' => {
                    self.horizontal_if();
                },
                'g' => {
                    self.get();
                },
                '@' => {
                    break;
                }
                _ => (),
            }
            self.step();
        }

        return self.output.as_str();
    }
}
