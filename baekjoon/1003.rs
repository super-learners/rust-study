use std::io;
use std::collections::HashMap;

fn main() {
    let num_of_testcases = read_line_from_stdin().trim()
        .parse().expect("Error parsing the input");

    for _i in 0..num_of_testcases {
        let num = read_line_from_stdin().trim()
            .parse().expect("Error parsing the input");
        let answer = solve_with_cheatsheet(num);
        println!("{} {}", answer.0, answer.1);
    }
}

fn read_line_from_stdin() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading from stdin");
        return input;
}

fn solve_inefficiently(num: i32) -> (i32, i32) {
    fn simulate_fibonacci(num: i32, tup: &mut (i32, i32)) -> (i32, i32) {
        if num == 0 {
            tup.0 += 1;
        } else if num == 1 {
            tup.1 += 1;
        } else {
            simulate_fibonacci(num - 1, tup);
            simulate_fibonacci(num - 2, tup);
        }
        *tup
    }
    simulate_fibonacci(num, &mut (0, 0))
}

fn solve_with_cheatsheet(num: i32) -> (i32, i32) {
    let mut cheatsheet: HashMap<i32, (i32, i32)> = HashMap::new();
    cheatsheet.insert(0, (1, 0));
    cheatsheet.insert(1, (0, 1));

    let mut pp = (1, 0);
    let mut p = (0, 1);
    for i in 2..(num + 1) {
        let c = (pp.0 + p.0, pp.1 + p.1);
        cheatsheet.insert(i, c);
        pp = p;
        p = c;
    }

    *cheatsheet.get(&num).expect("impossible")
}
