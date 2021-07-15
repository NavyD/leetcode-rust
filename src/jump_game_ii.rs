///
/// 第一次写的，类似于[`jump_game`]，至少思路差不多了，
/// 但是没有考虑细节
///
/// ```ignore
/// pub fn jump(nums: Vec<i32>) -> i32 {
///     if nums.len() == 1 {
///         return 0;
///     }
///     let mut longest_jump = 0;
///     let mut step = 0;
///     for i in 0..nums.len() {
///         if longest_jump + i > nums.len() {
///             break;
///         }
///         let jump = nums[i] as usize + i;
///         if jump > longest_jump {
///             longest_jump = jump;
///             step += 1;
///         }
///     }
///     step
/// }
/// ```
pub mod solution_greedy {
    /// # 思路
    ///
    /// 贪婪算法，我们每次在可跳范围内选择可以使得跳的更远的位置
    ///
    /// 注意：
    ///
    /// for 循环中，i < nums.length - 1，少了末尾。因为开始的时候边界是第 0 个位置，steps 已经加 1 了。
    /// 如果最后一步刚好跳到了末尾，此时 steps 其实不用加 1 了。如果是 i < nums.length，i 遍历到最后的时候，
    /// 会进入 if 语句中，steps 会多加 1。
    ///
    /// 访问最后一个元素之前，我们的边界一定大于等于最后一个位置，否则就无法跳到最后一个位置了
    ///
    /// 参考：
    ///
    /// - [跳跃游戏 II](https://leetcode-cn.com/problems/jump-game-ii/solution/tiao-yue-you-xi-ii-by-leetcode-solution/)
    /// - [Concise O(n) one loop JAVA solution based on Greedy](https://leetcode.com/problems/jump-game-ii/discuss/18014/Concise-O(n)-one-loop-JAVA-solution-based-on-Greedy)
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/jump-game-ii/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-10/)
    ///
    /// ### Submissions
    ///
    /// date=20210114, mem=2.3, mem_beats=16, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138274557/
    ///
    /// date=20210115, mem=2.1, mem_beats=83, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138591872/
    ///
    /// date=20210520, mem=2, mem_beats=70, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/179175793/
    ///
    /// date=20210714, mem=2.1, mem_beats=71, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195600767/
    ///
    /// date=20210715, mem=2, mem_beats=80, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195921017/
    pub struct Solution;

    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let mut steps = 0;
            // cur_end表示当前范围的最后下标， cur_longest_pos表示当前范围下可达到的最长位置
            let (mut cur_end, mut longest_pos) = (0, 0);
            // 0位置steps 已经加1 避免在nums.len() - 1时steps+1
            let end = nums.len() - 1;
            for i in 0..end {
                longest_pos = longest_pos.max(nums[i] as usize + i);
                // 当前范围走完，走下一步
                if i == cur_end {
                    cur_end = longest_pos;
                    steps += 1;
                    // 提前检查到达
                    if cur_end >= end {
                        break;
                    }
                }
            }
            steps
        }
    }
}

pub mod solution_greedy_reversed {
    /// # 思路
    ///
    /// 反向查找出发位置
    ///
    /// 我们的目标是到达数组的最后一个位置，因此我们可以考虑最后一步跳跃前所在的位置，该位置通过跳跃能够到达最后一个位置。
    ///
    /// 如果有多个位置通过跳跃都能够到达最后一个位置，那么我们应该如何进行选择呢？直观上来看，我们可以「贪心」地选择距离最后一个位置最远的那个位置，
    /// 也就是对应下标最小的那个位置。因此，我们可以从左到右遍历数组，选择第一个满足要求的位置。
    ///
    /// 找到最后一步跳跃前所在的位置之后，我们继续贪心地寻找倒数第二步跳跃前所在的位置，以此类推，直到找到数组的开始位置。
    ///
    /// 参考：
    ///
    /// - [跳跃游戏 II](https://leetcode-cn.com/problems/jump-game-ii/solution/tiao-yue-you-xi-ii-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20210114, mem=2.3, mem_beats=16, runtime=524, runtime_beats=14, url=https://leetcode-cn.com/submissions/detail/138284960/
    ///
    /// date=20210115, mem=2.1, mem_beats=50, runtime=528, runtime_beats=14, url=https://leetcode-cn.com/submissions/detail/138592278/
    ///
    /// date=20210307, mem=2.3, mem_beats=10, runtime=528, runtime_beats=5, url=https://leetcode-cn.com/submissions/detail/152105220/
    ///
    /// date=20210520, mem=2.1, mem_beats=23, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/179180098/
    ///
    /// date=20210714, mem=2.1, mem_beats=71, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195600767/
    ///
    /// date=20210714, mem=2.1, mem_beats=63, runtime=12, runtime_beats=15, url=https://leetcode-cn.com/submissions/detail/195922734/
    pub struct Solution;

    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let mut steps = 0;
            let mut pos = nums.len() - 1;
            while pos > 0 {
                // 找能到达pos的最小（最长）位置
                pos = (0..pos).find(|i| nums[*i] as usize + i >= pos).unwrap();
                steps += 1;
            }
            steps
        }
    }
}

pub mod solution_bfs {

    /// # 思路
    ///
    /// 从i开始找能走最远的所有下标max, 在`i..max`中找下一层的`next i..max`，直到`max>=end`。
    /// 也就是在每一层算一步一层走i..max都试一遍找下一步最大值
    ///
    /// 参考：
    ///
    /// * [O(n), BFS solution comment](https://leetcode.com/problems/jump-game-ii/discuss/18028/O(n)-BFS-solution/143760)
    /// * [【宫水三叶】修改数据范围，可以从「简单 BFS」变为「挖掘性质」的贪心 DP 题](https://leetcode-cn.com/problems/jump-game-ii/solution/gong-shui-san-xie-xiu-gai-shu-ju-fan-wei-wylq/)
    /// ### Submissions
    ///
    /// date=20210714, mem=2.2, mem_beats=25, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195608040/
    ///
    /// date=20210715, mem=2.2, mem_beats=34, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195927371/
    pub struct Solution;

    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            if nums.len() <= 1 {
                return 0;
            }

            let (mut cur_max, mut idx) = (0, 0);
            let mut level = 0;
            while idx <= cur_max {
                level += 1;
                // 下一层最大的
                let mut next_max = cur_max;
                while idx <= cur_max {
                    // 在当前层找 最跳最远的下标 作为下一层
                    next_max = next_max.max(idx + nums[idx] as usize);
                    if next_max >= nums.len() - 1 {
                        return level;
                    }
                    idx += 1;
                }
                cur_max = next_max;
            }
            // 无法跳转到最后
            unreachable!()
        }
    }
}

pub mod solution_dp {

    /// # 思路
    ///
    /// 定义problem(i)表示最少的跳跃次数，对于i只需要在不断更新0..i对应的最小次数problem(i)
    ///
    /// dp方程：`dp[i] = dp[i].min(dp[j] + 1) for j in 0..i if j + nums[j] >= i`
    ///
    /// 初始化：dp[0] = 0, dp[1..n] = max
    ///
    /// 参考：
    ///
    /// * [动态规划 跳跃游戏 II](https://leetcode-cn.com/problems/jump-game-ii/solution/dong-tai-gui-hua-jie-fa-by-alchemist-5r/)
    ///
    /// ### Submissions
    ///
    /// date=20210714, mem=2.1, mem_beats=69, runtime=236, runtime_beats=5, url=https://leetcode-cn.com/submissions/detail/195613150/
    ///
    /// date=20210714, mem=2, mem_beats=86, runtime=236, runtime_beats=5, url=https://leetcode-cn.com/submissions/detail/195934926/
    pub struct Solution;

    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            let mut dp = vec![i32::MAX; n];
            dp[0] = 0;

            for i in 1..n {
                for j in 0..i {
                    // 可以到达i
                    if j + nums[j] as usize >= i {
                        // 跳跃 更新
                        dp[i] = dp[i].min(dp[j] + 1);
                    }
                }
            }
            dp[n - 1] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::jump);
        test(solution_greedy_reversed::Solution::jump);
        test(solution_bfs::Solution::jump);
        test(solution_dp::Solution::jump);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(f(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(f(vec![1]), 0);
        assert_eq!(f(vec![0]), 0);
        assert_eq!(f(vec![1, 1, 1, 1]), 3);
        assert_eq!(f(vec![1, 2]), 1);
        assert_eq!(f(vec![2, 1]), 1);
    }
}
