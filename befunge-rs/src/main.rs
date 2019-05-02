mod interpreter;

use self::interpreter::*;

fn main() {
    let cmd = String::from("64+\"!dlroW ,olleH\">:#,_@");
    let mut interpreter: Environment = Interpreter::new();

    interpreter.run(&cmd);
}
