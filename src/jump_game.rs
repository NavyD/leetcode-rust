/// 首次写的拉跨
///
/// ```ignore
/// let mut steps = 0;
/// let mut i = 0;
/// while i < nums.len() && steps < nums.len() {
///     let step = nums[i] as usize;
///     steps += step;
///     i += step;
/// }
/// steps >= nums.len()
/// ```
///
/// 第3次与jump game ii有点混了
pub mod solution_greedy {
    /// # 思路
    ///
    /// 如果一个位置能够到达，那么这个位置左侧所有位置都能到达
    ///
    /// - 如果某一个作为 起跳点 的格子可以跳跃的距离是 3，那么表示后面 3 个格子都可以作为 起跳点。
    /// - 可以对每一个能作为 起跳点 的格子都尝试跳一次，把 能跳到最远的距离 不断更新。
    /// - 如果可以一直跳到最后，就成功了。
    ///
    /// 参考：
    ///
    /// - [【跳跃游戏】别想那么多，就挨着跳吧](https://leetcode-cn.com/problems/jump-game/solution/55-by-ikaruga/)
    ///
    /// ### Submissions
    ///
    /// date=20210112, mem=2.1, mem_beats=83, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137791586/
    ///
    /// date=20210113, mem=2.2, mem_beats=35, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138057889/
    ///
    /// date=20210124, mem=2.2, mem_beats=40, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140731511/
    ///
    /// date=20210307, mem=2.1, mem_beats=69, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152094236/
    ///
    /// date=20210709, mem=2.2, mem_beats=20, runtime=4, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/193894504/
    pub struct Solution;

    impl Solution {
        pub fn can_jump(nums: Vec<i32>) -> bool {
            let mut farthest_pos = 0;
            for i in 0..nums.len() {
                if i > farthest_pos {
                    return false;
                }
                farthest_pos = farthest_pos.max(i + nums[i] as usize);
            }
            true
        }
    }
}

pub mod solution_greedy_reversed {
    /// # 思路
    ///
    /// 倒着推。如果前个位置能到达当前位置，则更新当前位置。
    /// 当遍历完后，pos==0时表示可以到达
    ///
    /// 参考：
    ///
    /// - [顺着推、倒着推两种方式，击败了99%的java用户](https://leetcode-cn.com/problems/jump-game/solution/shun-zhao-tui-dao-zhao-tui-liang-chong-fang-shi-ji/)
    ///
    /// 注意不能使用`if nums[i] as usize + i < pos {return false;}`替换，在[2,0,0j时可能导致前面能到达的
    /// 不能被访问。
    ///
    /// ### Submissions
    ///
    /// date=20210112, mem=2.2, mem_beats=55, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137793447/
    ///
    /// date=20210113, mem=2.3, mem_beats=11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138058713/
    ///
    /// date=20210124, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140732473/
    ///
    /// date=20210307, mem=2, mem_beats=95, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152096208/
    ///
    /// date=20210709, mem=2.1, mem_beats=37, runtime=4, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/193895494/
    pub struct Solution;

    impl Solution {
        pub fn can_jump(nums: Vec<i32>) -> bool {
            let mut pos = nums.len() - 1;
            for i in (0..nums.len() - 1).rev() {
                // i位置能到达pos
                if nums[i] as usize + i >= pos {
                    pos = i;
                }
            }
            pos == 0
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// ## 方法1
    ///
    /// 定义problem(i)表示从0到i可以跳出的最远距离，最后只需要判断`problem(i)>=nums.len-1`。
    /// 最远距离可由之前距离i-1与当前距离i+nums[i]决定，如果i-1的距离可以到达i：`problem(i-1)>=i`则有
    /// 否则当前i不可达直接返回false
    ///
    /// dp方程：`dp[i] = dp[i-1].max(i+nums[i]) if dp[i-1]>=i`
    ///
    /// 初始化：dp[0]=nums[0]，当nums.len=1时有比较`dp[0]>=0, nums[0]>=0`
    ///
    /// ## 方法2
    ///
    /// 定义problem(i)表示i是否能够被0..i-1到达。那么当前位置可由上个位置是否可达dp[0..j]=true
    /// 与 由上个位置dp[j]到i可达决定
    ///
    /// dp方程：`dp[i] = dp[j] && j + nums[j] >= i for j in 0..i`
    ///
    /// 初始化：dp[0]=true
    ///
    /// ```
    /// pub fn can_jump(nums: Vec<i32>) -> bool {
    ///     let n = nums.len();
    ///     let mut dp = vec![false; n];
    ///     dp[0] = true;
    ///     for i in 1..n {
    ///         // rev相当找最长的 可以提高速度 552 ms -> 48 ms
    ///         for j in (0..i).rev() {
    ///             if dp[j] && j + nums[j] as usize >= i {
    ///                 dp[i] = true;
    ///                 break;
    ///             }
    ///         }
    ///     }
    ///     dp[n - 1]
    /// }
    /// assert!(!can_jump(vec![3, 2, 1, 0, 4]));
    /// ```
    ///
    /// 参考：
    ///
    /// * [[跳跃游戏] 动态规划 简单易懂 Python](https://leetcode-cn.com/problems/jump-game/solution/tiao-yue-you-xi-dong-tai-gui-hua-jian-da-ndz1/)
    /// * [动态规划与贪心算法解决此问题](https://leetcode-cn.com/problems/jump-game/solution/dong-tai-gui-hua-yu-tan-xin-suan-fa-jie-jue-ci-wen/)
    ///
    /// ### Submissions
    ///
    /// date=20210709, mem=2.2, mem_beats=25, runtime=8, runtime_beats=51, url=https://leetcode-cn.com/submissions/detail/193916234/
    pub struct Solution;

    impl Solution {
        pub fn can_jump(nums: Vec<i32>) -> bool {
            let n = nums.len();
            let mut dp = vec![0; n];
            dp[0] = nums[0] as usize;

            for i in 1..n {
                if dp[i - 1] >= i {
                    dp[i] = dp[i - 1].max(nums[i] as usize + i);
                } else {
                    // dp[i] = dp[i - 1];
                    return false;
                }
            }
            dp[n - 1] >= n - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::can_jump);
        test(solution_greedy_reversed::Solution::can_jump);
        test(solution_dp::Solution::can_jump);
    }

    fn test<F: Fn(Vec<i32>) -> bool>(f: F) {
        assert!(f(vec![2, 3, 1, 1, 4]));
        assert!(!f(vec![3, 2, 1, 0, 4]));
        assert!(f(vec![0]));
        assert!(f(vec![2, 0, 0]));
    }
}
