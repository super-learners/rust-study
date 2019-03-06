impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut v = vec![];
        let mut n = x;
        while n != 0 {
            v.push(n % 10);
            n /= 10;
        }

        let y = v.into_iter()
            .rev()
            .enumerate()
            .map(|(i, a)| a as i64 * 10i64.pow(i as u32))
            .sum();

        if y < (std::i32::MIN as i64) || (std::i32::MAX as i64) < y {
            0
        } else {
            y as i32
        }
    }
}
