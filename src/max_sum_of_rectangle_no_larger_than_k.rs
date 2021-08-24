pub mod solution_dp {
    pub struct Solution;

    impl Solution {
        /// dp[i][j] = dp[i-1][j]+dp[i-1][j-1]+dp[i][j-1]+matrix[i-1][j-1] if
        ///
        /// `dp(i1,j1,i2,j2) = dp(i1,j1,i2 - 1,j2) + dp(i1,j1,i2,j2 - 1) - dp(i1,j1,i2 - 1,j2 - 1) + matrix[i2 - 1][j2 - 1];`
        ///
        /// 超出时间限制：36 / 39 个通过测试用例
        ///
        /// 参考：
        ///
        /// - [Java，从暴力开始优化，配图配注释](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/solution/javacong-bao-li-kai-shi-you-hua-pei-tu-pei-zhu-shi/)
        pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut dp = vec![vec![vec![vec![0; cols + 1]; rows + 1]; cols + 1]; rows + 1];
            let mut res = i32::MIN;
            for i0 in 1..=rows {
                for j0 in 1..=cols {
                    dp[i0][j0][i0][j0] = matrix[i0 - 1][j0 - 1];
                    for i1 in i0..=rows {
                        for j1 in j0..=cols {
                            dp[i0][j0][i1][j1] = dp[i0][j0][i1 - 1][j1]
                                + dp[i0][j0][i1][j1 - 1]
                                + matrix[i1 - 1][j1 - 1]
                                - dp[i0][j0][i1 - 1][j1 - 1];
                            if dp[i0][j0][i1][j1] <= k {
                                res = res.max(dp[i0][j0][i1][j1]);
                            }
                        }
                    }
                }
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
        test(solution_dp::Solution::max_sum_submatrix);
    }

    fn test<F: Fn(Vec<Vec<i32>>, i32) -> i32>(f: F) {
        assert_eq!(f(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
        assert_eq!(f(vec![vec![2, 2, -1]], 3), 3);
        assert_eq!(f(vec![vec![2, 2, -1]], 0), -1);
        assert_eq!(
            f(
                vec![vec![5, -4, -3, 4], vec![-3, -4, 4, 5], vec![5, 1, 5, -4]],
                10
            ),
            10
        );
    }
}
