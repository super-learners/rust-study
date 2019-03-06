impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut stack = Vec::new();
        let minus = x < 0;
        let mut n: Option<i32> = x.checked_abs();

        while n.unwrap_or(0) > 0 {
            stack.insert(0, n.unwrap_or(0) % 10);
            n = Some(n.unwrap_or(0) / 10);
        }
        while !stack.is_empty() && n.is_some() {
            let m = stack.pop().unwrap_or(0);
            n = n.and_then(|a| a.checked_mul(10))
                 .and_then(|a| a.checked_add(m));
        }
        if minus {
            n = Some(-n.unwrap_or(0));
        }
        return n.unwrap_or(0);
    }
}
