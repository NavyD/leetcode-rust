pub mod solution_violent {
    /// # 思路
    ///
    /// 面积 = 边长 x 边长，由于正方形的面积等于边长的平方，因此要找到最大正方形的面积，
    /// 首先需要找到最大正方形的边长，然后计算最大边长的平方即可。
    ///
    /// 暴力法是最简单直观的做法，具体做法如下：
    ///
    /// - 遍历矩阵中的每个元素，每次遇到 1，则将该元素作为正方形的左上角；
    /// - 确定正方形的左上角后，根据左上角所在的行和列计算可能的最大正方形的边长
    /// （正方形的范围不能超出矩阵的行数和列数），在该边长范围内寻找只包含 1 的最大正方形；
    /// - 每次在下方新增一行以及在右方新增一列，判断新增的行和列是否满足所有元素都是 1
    ///
    /// 参考：
    ///
    /// - [最大正方形 方法一：暴力法](https://leetcode-cn.com/problems/maximal-square/solution/zui-da-zheng-fang-xing-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20210823, mem=4.9, mem_beats=22, runtime=12, runtime_beats=32, url=https://leetcode-cn.com/submissions/detail/210246498/
    pub struct Solution;

    impl Solution {
        pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
            const ONE: char = '1';
            const ZERO: char = '0';
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut max_side = 0;

            for i in 0..rows {
                for j in 0..cols {
                    // 遇到1时将该点作为正方形起点
                    if matrix[i][j] == ONE {
                        max_side = max_side.max(1);
                        // 找到可能的最大正方形边长作为下标偏移量
                        let cur_side = (rows - i).min(cols - j);
                        // 对每次增加的行列 检查为行为起点
                        for k in 1..cur_side {
                            // 检查对角最后位置是否为1
                            if matrix[i + k][j + k] == ZERO {
                                break;
                            }

                            let mut has_square = true;
                            // 检查以列为起点看下标是否为1
                            for m in 0..k {
                                if matrix[i + k][j + m] == ZERO || matrix[i + m][j + k] == ZERO {
                                    has_square = false;
                                    break;
                                }
                            }

                            // 出现1表示当前位置是 并记录最大边长
                            if has_square {
                                max_side = max_side.max(k + 1);
                            } else {
                                // 出现0表示当前位置之后不是正方形
                                break;
                            }
                        }
                    }
                }
            }
            (max_side * max_side) as i32
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// 用 dp(i,j) 表示以 (i, j) 为右下角，且只包含 1 的正方形的边长最大值。如果我们能计算出所有 dp(i,j) 的值，
    /// 那么其中的最大值即为矩阵中只包含 1 的正方形的边长最大值，其平方即为最大正方形的面积。
    ///
    /// 那么如何计算 dp 中的每个元素值呢？对于每个位置 (i,j)，检查在矩阵中该位置的值：
    ///
    /// - 如果该位置的值是 0，则 dp(i,j)=0，因为当前位置不可能在由 1 组成的正方形中；
    /// - 如果该位置的值是 1，则 dp(i,j) 的值由其 ***上方、左方和左上方*** 的三个相邻位置的 dp 值决定。
    /// 具体而言，当前位置的元素值等于三个相邻位置的元素中的最小值加 1
    ///
    /// 状态转移方程如下：`dp[i][j] = min(dp[i - 1][j - 1], dp[i - 1][j], dp[i][j - 1]) + 1 if matrix[i - 1][j - 1] == '1'`
    ///
    /// 初始化：如果 i 和 j 中至少有一个为 0，则以位置 (i,j) 为右下角的最大正方形的边长只能是 1，
    /// 为了避免到边的判断处理，在最左侧加上一列 `dp[i][0] = 0` ，在左上边加上一行 `dp[0][j] = 0`，
    /// `new dp[rows + 1][cols + 1]`
    ///
    /// ![](https://pic.leetcode-cn.com/8c4bf78cf6396c40291e40c25d34ef56bd524313c2aa863f3a20c1f004f32ab0-image.png)
    ///
    /// - [理解 三者取最小+1](https://leetcode-cn.com/problems/maximal-square/solution/li-jie-san-zhe-qu-zui-xiao-1-by-lzhlyle/)
    /// - [最大正方形 方法二：动态规划](https://leetcode-cn.com/problems/maximal-square/solution/zui-da-zheng-fang-xing-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20210823, mem=4.9, mem_beats=13, runtime=8, runtime_beats=84, url=https://leetcode-cn.com/submissions/detail/210316151/
    pub struct Solution;

    impl Solution {
        pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut max_side = 0;

            let mut dp = vec![vec![0; cols + 1]; rows + 1];

            for i in 1..=rows {
                for j in 1..=cols {
                    if matrix[i - 1][j - 1] == '1' {
                        dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - 1]).min(dp[i][j - 1]) + 1;
                        max_side = max_side.max(dp[i][j]);
                    }
                }
            }
            (max_side * max_side) as i32
        }
    }
}

pub mod solution_dp_optimized {

    /// # 思路
    ///
    /// 参考：
    ///
    /// - [理解 三者取最小+1](https://leetcode-cn.com/problems/maximal-square/solution/li-jie-san-zhe-qu-zui-xiao-1-by-lzhlyle/)
    ///
    /// ### Submissions
    ///
    /// date=20210823, mem=4.9, mem_beats=32, runtime=8, runtime_beats=83, url=https://leetcode-cn.com/submissions/detail/210325516/
    pub struct Solution;

    impl Solution {
        pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut max_side = 0;

            let mut dp = vec![0; cols + 1];
            for i in 1..=rows {
                let mut top_left = 0;
                for j in 1..=cols {
                    let next_top_left = dp[j];
                    if matrix[i - 1][j - 1] == '1' {
                        dp[j] = dp[j].min(dp[j - 1]).min(top_left) + 1;
                        max_side = max_side.max(dp[j]);
                    } else {
                        dp[j] = 0;
                    }
                    top_left = next_top_left;
                }
            }
            (max_side * max_side) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_violent::Solution::maximal_square);
        test(solution_dp::Solution::maximal_square);
        test(solution_dp_optimized::Solution::maximal_square);
    }

    fn test<F: Fn(Vec<Vec<char>>) -> i32>(f: F) {
        assert_eq!(
            f(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
        assert_eq!(f(vec![vec!['1', '1'], vec!['1', '1']]), 4);
        assert_eq!(f(vec![vec!['0', '1'], vec!['1', '0']]), 1);
        assert_eq!(f(vec![vec!['0']]), 0);
    }
}
