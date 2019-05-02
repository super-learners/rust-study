extern crate befunge_rs;

use befunge_rs::interpreter::*;

fn test(cmd: &str, output: &str) {
    let mut interpreter: Environment = Interpreter::new();
    let _output = interpreter.run(cmd);
    assert_eq!(_output, output);
}

#[test]
fn hello_world() {
    test("64+\"!dlroW ,olleH\">:#,_@", "Hello, World!\n");
}

#[test]
fn quine() {
    test("01->1# +# :# 0# g# ,# :# 5# 8# *# 4# +# -# _@", "01->1# +# :# 0# g# ,# :# 5# 8# *# 4# +# -# _@");
}
