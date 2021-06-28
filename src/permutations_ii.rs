//! 如何对list中的list去重
//!
//! - set
//! - 搜索前判断是否已交换
//!
//! 如何对一个list全排列
//!
//! - 递归回溯

pub mod solution_sort {
    /// `println!("swap: {},{} {:?}", i, cur_idx, nums);`
    /// sort                    set
    /// swap: 0,0 [1, 2, 2]     swap: 0,0 [1, 2, 2]
    /// swap: 1,1 [1, 2, 2]     swap: 1,1 [1, 2, 2]
    /// swap: 2,2 [1, 2, 2]     swap: 2,2 [1, 2, 2]
    /// swap: 2,1 [1, 2, 2]     swap: 2,1 [1, 2, 2]
    /// swap: 2,2 [1, 2, 2]     swap: 2,2 [1, 2, 2]
    /// swap: 1,0 [2, 1, 2]     swap: 1,0 [2, 1, 2]
    /// swap: 1,1 [2, 1, 2]     swap: 1,1 [2, 1, 2]
    /// swap: 2,2 [2, 1, 2]     swap: 2,2 [2, 1, 2]
    /// swap: 2,1 [2, 2, 1]     swap: 2,1 [2, 2, 1]
    /// swap: 2,2 [2, 2, 1]     swap: 2,2 [2, 2, 1]
    /// swap: 2,0 [2, 1, 2]     swap: 2,0 [2, 2, 1]
    /// swap: 1,1 [2, 1, 2]     swap: 1,1 [2, 2, 1]
    /// swap: 2,2 [2, 1, 2]     swap: 2,2 [2, 2, 1]
    /// swap: 2,1 [2, 2, 1]     swap: 2,1 [2, 1, 2]
    /// swap: 2,2 [2, 2, 1]     swap: 2,2 [2, 1, 2]
    ///
    ///
    /// 为何不能使用&mut nums或再次swap，而是要使用clone
    ///
    /// example: [1,2,2]
    ///
    /// - 如果在回溯时使用swap恢复，同层不影响前后，下层改变不影响上层，则有多处重复 set
    ///
    /// ```ignore
    /// cur:0                                               [1,2,2]
    ///                                   / ex 0,0              | ex 0,1                            \ ex 0,2
    /// cur:1                       [1,2,2]                   [2,1,2]                               [2,2,1]
    ///                       / ex 1,1    \ ex 1,2         /         \                             /        \       
    /// cur:2             [1,2,2]         [1,2,2]      [2,1,2]      [2,2,1]                    [2,2,1]      [2,1,2]
    ///         all:ex 2,2 /               \             /               \                       /               \
    /// cur:3       [1,2,2]              [1,2,2]   [2,1,2]              [2,2,1]             [2,2,1]              [2,1,2]
    /// ```
    ///
    /// - 如果没有再次恢复，使用clone，下层的改变不会影响上层，只有同层前面的swap影响后面的。如swap 2,0 被swap 1,0影响
    ///
    /// ```ignore
    /// cur:0                                               [1,2,2]
    ///                                   / ex 0,0              | ex 0,1                            \ ex 0,2
    /// cur:1                       [1,2,2]                   [2,1,2]                               [2,1,2]
    ///                       / ex 1,1    \ ex 1,2         /         \                             /        \       
    /// cur:2             [1,2,2]         [1,2,2]      [2,1,2]      [2,2,1]                    [2,1,2]      [2,2,1]
    ///         all:ex 2,2 /               \             /               \                       /               \
    /// cur:3       [1,2,2]              [1,2,2]   [2,1,2]              [2,2,1]             [2,1,2]              [2,2,1]
    /// ```
    ///
    /// 两种情况的区别：第二种在cur=0时ex 0,2时被前面的ex 0,1结果影响[2,1,2]，使结果一样
    ///
    /// 在第二种情况中可以使用条件`i == cur_idx || &nums[i] != &nums[cur_idx]`过滤。在有序的list中，同层相同的元素会
    /// swap i,cur_idx 一直在当前cur_idx层重复
    ///
    /// ### submissions
    ///
    /// date=20201118, mem=2.1, mem_beats=96, runtime=4, runtime_beats=95, url=https://leetcode-cn.com/submissions/detail/124411651/
    ///
    /// date=20201123, mem=2, mem_beats=97, runtime=4, runtime_beats=95, url=https://leetcode-cn.com/submissions/detail/125656670/
    ///
    /// date=20201124, mem=2, mem_beats=79, runtime=4, runtime_beats=95, url=https://leetcode-cn.com/submissions/detail/125888656/
    pub struct Solution;

    impl Solution {
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            fn helper(mut nums: Vec<i32>, cur_idx: usize, res: &mut Vec<Vec<i32>>) {
                if nums.len() == cur_idx {
                    res.push(nums);
                    return;
                }
                for i in cur_idx..nums.len() {
                    if i == cur_idx || nums[i] != nums[cur_idx] {
                        nums.swap(i, cur_idx);
                        helper(nums.clone(), cur_idx + 1, res);
                    }
                }
            }
            nums.sort_unstable();
            let mut res = Vec::new();
            helper(nums, 0, &mut res);
            res
        }
    }
}

pub mod solution_set {
    use std::collections::HashSet;

    /// # 思路
    ///
    /// 使用set来过滤结果
    ///
    /// ```ignore
    ///                      1,2,3
    ///         /               |                   \
    ///        1                2                   3
    ///       / \
    ///      2   3          1       3           1       2
    ///     /     \
    ///    3       2    3               1   2               1
    /// ```ignore
    ///
    /// ### Submissions
    ///
    /// date=20201118, mem=2.1, mem_beats=46, runtime=20, runtime_beats=13, url=https://leetcode-cn.com/submissions/detail/124379964/
    ///
    /// date=20201123, mem=2.1, mem_beats=48, runtime=16, runtime_beats=13, url=https://leetcode-cn.com/submissions/detail/125582953/
    pub struct Solution;

    impl Solution {
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            fn helper(nums: &mut Vec<i32>, cur_idx: usize, res: &mut HashSet<Vec<i32>>) {
                if nums.len() == cur_idx {
                    res.insert(nums.to_vec());
                    return;
                }
                for i in cur_idx..nums.len() {
                    nums.swap(i, cur_idx);
                    helper(nums, cur_idx + 1, res);
                    nums.swap(i, cur_idx);
                }
            }

            let mut res = HashSet::new();
            helper(&mut nums, 0, &mut res);
            res.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_set::Solution::permute_unique);
    }

    fn test<F: Fn(Vec<i32>) -> Vec<Vec<i32>>>(func: F) {
        func(vec![1, 2, 2]);
    }
}
