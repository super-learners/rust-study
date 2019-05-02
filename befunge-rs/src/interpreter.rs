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
    fn duplicate(&mut self);
    fn print_int(&mut self) -> i8;
    fn print_char(&mut self) -> char;
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
        let a: i8 = self.stack.pop().unwrap();
        let b: i8 = self.stack.pop().unwrap();

        self.stack.push(a + b);
    }

    fn subtract(&mut self) {
        let a: i8 = self.stack.pop().unwrap();
        let b: i8 = self.stack.pop().unwrap();

        self.stack.push(b - a);
    }

    fn multiply(&mut self) {
        let a: i8 = self.stack.pop().unwrap();
        let b: i8 = self.stack.pop().unwrap();

        self.stack.push(a * b);
    }

    fn integer_division(&mut self) {
        let a: i8 = self.stack.pop().unwrap();
        let b: i8 = self.stack.pop().unwrap();

        self.stack.push(b / a);
    }

    fn modulo(&mut self) {
        let a: i8 = self.stack.pop().unwrap();
        let b: i8 = self.stack.pop().unwrap();

        self.stack.push(b % a);
    }

    fn logical_not(&mut self) {
        let a: i8 = self.stack.pop().unwrap();
        self.stack.push(if a == 0 { 1 } else { 0 });
    }


    fn duplicate(&mut self) {
        let n: i8 = self.stack.pop().unwrap_or(0);

        self.stack.push(n);
        self.stack.push(n);
    }

    fn print_int(&mut self) -> i8 {
        let n = self.stack.pop().unwrap();
        print!("{}", n);

        self.output.push_str(n.to_string().as_str());

        return n;
    }

    fn print_char(&mut self) -> char {
        let c = self.stack.pop().unwrap() as u8 as char;
        print!("{}", c);

        self.output.push(c);

        return c;
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
        let n = self.stack.pop().unwrap();
        if n == 0 {
            self.pc_direction = PcDirection::Right;
        } else {
            self.pc_direction = PcDirection::Left;
        }
    }

    fn get(&mut self) {
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();

        let ch = self.commands
            .get(y as usize)
            .unwrap()
            .as_bytes()[x as usize] as char;

        self.stack.push(ch as i8);
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
