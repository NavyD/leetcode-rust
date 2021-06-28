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
    pub struct Solution;

    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let mut steps = 0;
            // cur_end表示当前范围的最后下标， cur_longest_pos表示当前范围下可达到的最长位置
            let (mut cur_end, mut cur_longest_pos) = (0, 0);
            // 0位置steps 已经加1 避免在nums.len() - 1时steps+1
            let end = nums.len() - 1;
            for i in 0..end {
                cur_longest_pos = cur_longest_pos.max(nums[i] as usize + i);
                // 当前范围走完，走下一步
                if i == cur_end {
                    cur_end = cur_longest_pos;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::jump);
        test(solution_greedy_reversed::Solution::jump);
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
