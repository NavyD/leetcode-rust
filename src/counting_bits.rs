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
    ///
    /// date=20220616, mem=2.5, mem_beats=80, runtime=4, runtime_beats=100
    ///
    /// date=20220630, mem=2.7, mem_beats=14, runtime=4, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn count_bits(n: i32) -> Vec<i32> {
            #[inline]
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
    /// * `bits[i] = bits[i & (i - 1)] + 1;`: i & (i - 1) 可以去掉 i 最右边的一个 1（如果有），
    /// 因此 i & (i - 1）是比 i 小的，而且 i & (i - 1) 的 1 的个数已经在前面算过了，
    /// 所以 i 的 1 的个数就是 i & (i - 1) 的 1 的个数加上 1
    ///
    /// * `bits[i] = bits[i>>1]+(i&1)`: i >> 1 会把最低位去掉，因此 i >> 1 也是比 i 小的，
    /// 同样也是在前面的数组里算过。当 i 的最低位是 0，则 i 中 1 的个数和 i >> 1 中 1 的个数相同；
    /// 当 i 的最低位是 1，i 中 1 的个数是 i >> 1 中 1 的个数再加 1
    ///
    /// 参考：
    ///
    /// * [比特位计数](https://leetcode.cn/problems/counting-bits/solution/bi-te-wei-ji-shu-by-leetcode-solution-0t1i/)
    /// * [清晰的思路](https://leetcode.cn/problems/counting-bits/solution/hen-qing-xi-de-si-lu-by-duadua/)
    ///
    /// ### Submissions
    ///
    /// date=20220615, mem=2.9, mem_beats=5, runtime=4, runtime_beats=100
    ///
    /// date=20220616, mem=2.6, mem_beats=28, runtime=4, runtime_beats=100
    ///
    /// date=20220630, mem=2.9, mem_beats=7, runtime=4, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn count_bits(n: i32) -> Vec<i32> {
            let n = (n + 1) as usize;
            let mut bits = vec![0; n];

            let mut res = vec![];
            for i in 0..n {
                bits[i] = bits[i >> 1] + (i & 1);
                res.push(bits[i] as i32)
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_loop::Solution::count_bits);
        test(solution_dp::Solution::count_bits);
    }

    fn test(f: impl Fn(i32) -> Vec<i32>) {
        assert_eq!(f(2), [0, 1, 1].to_vec());
        assert_eq!(f(5), [0, 1, 1, 2, 1, 2].to_vec());
    }
}
