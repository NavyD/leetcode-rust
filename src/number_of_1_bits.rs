#[allow(non_snake_case)]
pub mod solution_bit {
    /// # 思路
    ///
    /// 把 n 的二进制位中的最低位的 1 变为 0 之后的结果。
    ///
    /// ```no
    /// 1.
    /// n   = 0b101
    /// n-1 = 0b100     &=>0b100
    /// 2.
    /// n   = 0b100
    /// n-1 = 0b011     &=>0b000
    /// ```
    ///
    /// [u32::count_ones][u32::count_ones]
    ///
    /// 参考：
    ///
    /// * [位 1 的个数](https://leetcode.cn/problems/number-of-1-bits/solution/wei-1de-ge-shu-by-leetcode-solution-jnwf/)
    ///
    /// ### Submissions
    ///
    /// date=20220609, mem=2, mem_beats=76, runtime=0, runtime_beats=100
    ///
    /// date=20220610, mem=2.2, mem_beats=16, runtime=0, runtime_beats=100
    ///
    /// date=20220617, mem=2, mem_beats=79, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn hammingWeight(mut n: u32) -> i32 {
            let mut count = 0;
            while n != 0 {
                n &= n - 1;
                count += 1;
            }
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_bit::Solution::hammingWeight);
    }

    fn test(f: impl Fn(u32) -> i32) {
        assert_eq!(f(0b00000000000000000000000000001011), 3);
        assert_eq!(f(0b00000000000000000000000010000000), 1);
        assert_eq!(f(0b11111111111111111111111111111101), 31);
    }
}
