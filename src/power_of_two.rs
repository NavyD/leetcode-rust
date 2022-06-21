pub mod solution_bit {
    /// # 思路
    ///
    /// 这是 n 为 2 的幂的必要性，我论证一下充分性：
    /// n & n-1 可以把 n 最低位的 1 变 0，而当 n & n-1 == 0 时，则说明 n 只有一个 1。
    ///
    /// 若 n = 2^x 且 xx 为自然数（即 n 为 2 的幂），则一定满足以下条件：
    /// * 恒有 n & (n - 1) == 0，这是因为：
    ///   * n 二进制最高位为 1，其余所有位为 0；
    ///   * n - 1 二进制最高位为 0，其余所有位为 1；
    /// * 一定满足 n > 0
    ///
    /// 因此，通过 n > 0 且 n & (n - 1) == 0 即可判定是否满足 n = 2^x
    ///
    /// 参考：
    ///
    /// * [2 的幂 （位运算，极简解法 + 图表解析）](https://leetcode.cn/problems/power-of-two/solution/power-of-two-er-jin-zhi-ji-jian-by-jyd/)
    /// * [2 的幂](https://leetcode.cn/problems/power-of-two/solution/2de-mi-by-leetcode-solution-rny3/)
    ///
    /// ### Submission
    ///
    /// date=20220611, mem=1.9, mem_beats=95, runtime=0, runtime_beats=100
    ///
    /// date=20220612, mem=2.2, mem_beats=6, runtime=0, runtime_beats=100
    ///
    /// date=20220621, mem=2, mem_beats=56, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn is_power_of_two(n: i32) -> bool {
            n > 0 && (n & (n - 1)) == 0
        }
    }
}

pub mod solution_complement {
    /// # 思路
    ///
    /// 其中 -n 是 n 的相反数，是一个负数。该位运算技巧可以直接*获取 n 二进制表示的最低位的 1*
    ///
    /// 由于负数是按照补码规则在计算机中存储的，-n 的二进制表示为 n 的二进制表示的每一位取反再加上 1
    ///
    /// 参考：
    ///
    /// * [2 的幂](https://leetcode.cn/problems/power-of-two/solution/2de-mi-by-leetcode-solution-rny3/)
    ///
    /// ### Submission
    ///
    /// date=20220611, mem=1.9, mem_beats=95, runtime=0, runtime_beats=100
    ///
    /// date=20220612, mem=2.1, mem_beats=38, runtime=0, runtime_beats=100
    ///
    /// date=20220622, mem=2, mem_beats=62, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn is_power_of_two(n: i32) -> bool {
            n > 0 && (n & -n) == n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_bit::Solution::is_power_of_two);
        test(solution_complement::Solution::is_power_of_two);
    }

    fn test(f: impl Fn(i32) -> bool) {
        assert!(f(1));
        assert!(f(4));
        assert!(f(16));
        assert!(!f(5));
        assert!(!f(3));
    }
}
