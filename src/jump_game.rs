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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::can_jump);
        test(solution_greedy_reversed::Solution::can_jump);
    }

    fn test<F: Fn(Vec<i32>) -> bool>(f: F) {
        assert!(f(vec![2, 3, 1, 1, 4]));
        assert!(!f(vec![3, 2, 1, 0, 4]));
        assert!(f(vec![0]));
        assert!(f(vec![2,0,0]));
    }
}
