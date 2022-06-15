pub mod solution_loop {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [比特位计数](https://leetcode.cn/problems/counting-bits/solution/bi-te-wei-ji-shu-by-leetcode-solution-0t1i/)
    ///
    /// ### Submissions
    ///
    /// date=20220615, mem=2.7, mem_beats=14, runtime=4, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn count_bits(n: i32) -> Vec<i32> {
            fn count_ones(mut i: i32) -> i32 {
                let mut count = 0;
                while i != 0 {
                    i &= i - 1;
                    count += 1;
                }
                count
            }

            (0..=n).map(count_ones).collect()
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// or
    ///
    /// ```no
    /// bits[i] = bits[i & (i - 1)] + 1;
    /// ```
    ///
    /// 参考：
    ///
    /// * [比特位计数](https://leetcode.cn/problems/counting-bits/solution/bi-te-wei-ji-shu-by-leetcode-solution-0t1i/)
    /// * [清晰的思路](https://leetcode.cn/problems/counting-bits/solution/hen-qing-xi-de-si-lu-by-duadua/)
    ///
    /// ### Submissions
    ///
    /// date=20220615, mem=2.9, mem_beats=5, runtime=4, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn count_bits(n: i32) -> Vec<i32> {
            let n = (n + 1) as usize;
            let mut bits = vec![0; n];

            (0..n)
                .map(|i| {
                    bits[i] = bits[i >> 1] + (i & 1);
                    bits[i] as i32
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_loop::Solution::count_bits);
        test(solution_dp::Solution::count_bits)
    }

    fn test(f: impl Fn(i32) -> Vec<i32>) {
        assert_eq!(f(2), [0, 1, 1].to_vec());
        assert_eq!(f(5), [0, 1, 1, 2, 1, 2].to_vec());
    }
}
