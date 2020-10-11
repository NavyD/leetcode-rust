pub mod solution_dp {

    /// # 思路
    /// 
    /// 设dp[i]为在i个梯子时可走的步数，第i个梯子可以是i-1和i-2个上来的，
    /// 可有`dp[i] = dp[i-1] + dp[i-2]`
    /// 
    /// ```ignore
    /// dp[0] = 1
    /// dp[1] = 1
    /// dp[2] = 2
    /// dp[3] = 3
    /// dp[4] = 5 // 1111, 112, 121, 211, 22
    /// dp[5] = 8 // 11111, 1112, 1121, 1211, 2111, 122, 212, 221
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200621, mem=2, mem_beats=55.93, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356338066/,
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            let n = n as usize;
            let mut dp = vec![1; n + 1];
            for i in 2..=n {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            dp[n]
        }
    }
}

pub mod solution_dp_optimized {
    /// ### Submissions
    /// 
    /// date=20200621, mem=2.1, mem_beats=22.22, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98491120/
    /// 
    /// date=20201011, mem=2.1, mem_beats=6.98, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/114824244/
    ///
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            if n <= 1 {
                return 1;
            }
            let (mut prev1, mut prev2, mut cur) = (1, 1, 1);
            for _ in 2..=n {
                cur = prev1 + prev2;
                prev1 = prev2;
                prev2 = cur;
            }
            cur
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::climb_stairs);
        test(solution_dp_optimized::Solution::climb_stairs);
    }

    fn test<F: Fn(i32) -> i32>(func: F) {
        assert_eq!(func(0), 1);
        assert_eq!(func(1), 1);
        assert_eq!(func(2), 2);
        assert_eq!(func(3), 3);
        assert_eq!(func(4), 5);
    }
}
