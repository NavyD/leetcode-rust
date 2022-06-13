pub mod solution_loop {
    /// # 思路
    ///
    /// ### Submission
    ///
    /// date=20220612, mem=2, mem_beats=57, runtime=4, runtime_beats=93
    ///
    /// date=20220613, mem=2.1, mem_beats=44, runtime=16, runtime_beats=27
    pub struct Solution;

    impl Solution {
        pub fn is_power_of_three(mut n: i32) -> bool {
            if n <= 0 {
                return false;
            }
            while n % 3 == 0 {
                n /= 3;
            }
            n == 1
        }
    }
}

pub mod solution_divisor {
    /// # 思路
    ///
    /// 在题目给定的 32 位有符号整数的范围内，最大的 3 的幂为 3^19=1162261467。
    /// 我们只需要判断 n 是否是 3^19的约数即可
    ///
    /// ### Submission
    ///
    /// date=20220612, mem=1.9, mem_beats=91, runtime=8, runtime_beats=81
    ///
    /// date=20220613, mem=2, mem_beats=67, runtime=4, runtime_beats=93
    pub struct Solution;

    impl Solution {
        pub fn is_power_of_three(n: i32) -> bool {
            static MAX_DIV: i32 = 3_i32.pow(19);
            n > 0 && MAX_DIV % n == 0
        }
    }
}

pub mod solution_table {

    /// # 思路
    ///
    /// ### Submission
    ///
    /// date=20220612, mem=2, mem_beats=62, runtime=4, runtime_beats=93
    pub struct Solution;

    impl Solution {
        pub fn is_power_of_three(n: i32) -> bool {
            matches!(
                n,
                1 | 3
                    | 9
                    | 27
                    | 81
                    | 243
                    | 729
                    | 2187
                    | 6561
                    | 19683
                    | 59049
                    | 177147
                    | 531441
                    | 1594323
                    | 4782969
                    | 14348907
                    | 43046721
                    | 129140163
                    | 387420489
                    | 1162261467
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_loop::Solution::is_power_of_three);
        test(solution_divisor::Solution::is_power_of_three);
        test(solution_table::Solution::is_power_of_three);
    }

    fn test(f: impl Fn(i32) -> bool) {
        assert!(f(27));
        assert!(f(9));
        assert!(!f(0));
        assert!(!f(45));
    }
}
