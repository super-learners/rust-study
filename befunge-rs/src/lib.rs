mod interpreter;

#[cfg(test)]
mod tests {
    use super::interpreter::Environment;
    use super::interpreter::Interpreter;

    #[test]
    fn test_hello() {
        let cmd = String::from("64+\"!dlroW ,olleH\">:#,_@");
        let mut interpreter: Environment = Interpreter::new();
        let output = interpreter.run(&cmd);

        assert_eq!(output, "Hello, World!\n");
    }

    #[test]
    fn test_quine() {
        let cmd = String::from("01->1# +# :# 0# g# ,# :# 5# 8# *# 4# +# -# _@");
        let mut interpreter: Environment = Interpreter::new();
        let output = interpreter.run(&cmd);

        assert_eq!(output, "01->1# +# :# 0# g# ,# :# 5# 8# *# 4# +# -# _@");
    }
}