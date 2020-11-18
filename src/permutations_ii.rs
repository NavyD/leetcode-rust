//! 如何对list中的list去重
//!
//! - set
//! - 搜索前判断是否已交换
//!
//! 如何对一个list全排列
//!
//! - 递归回溯

pub mod solution_sort {
    /// 为何不能使用&mut nums或再次swap，而是要使用clone
    ///
    /// ### submissions
    ///
    /// date=20201118, mem=2.1, mem_beats=16, runtime=4, runtime_beats=95, url=https://leetcode-cn.com/submissions/detail/124411651/
    ///                  1,2,2
    ///         /       |       \
    ///        1        2        x
    ///     2    x    1  2
    ///  2           2    1
    ///
    pub struct Solution;

    impl Solution {
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort_unstable();
            let mut res = vec![];
            Self::helper(nums, 0, &mut res);
            res
        }

        fn helper(mut nums: Vec<i32>, cur_idx: usize, res: &mut Vec<Vec<i32>>) {
            if cur_idx == nums.len() {
                res.push(nums);
                return;
            }

            for i in cur_idx..nums.len() {
                if i == cur_idx || &nums[i] != &nums[cur_idx] {
                    nums.swap(i, cur_idx);
                    Self::helper(nums.clone(), cur_idx + 1, res);
                }
            }
        }
    }
}

pub mod solution_set {
    use std::collections::HashSet;

    ///                     1,2,3
    ///         /               |                   \
    ///        1                2                   3
    ///       / \
    ///      2   3          1       3           1       2
    ///     /     \
    ///    3       2    3               1   2               1
    ///
    /// ### Submissions
    ///
    /// date=20201118, mem=2.1, mem_beats=46, runtime=20, runtime_beats=13, url=https://leetcode-cn.com/submissions/detail/124379964/
    pub struct Solution;

    impl Solution {
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut set = HashSet::new();
            Self::helper(&mut nums, 0, &mut set);
            set.into_iter().collect()
        }

        fn helper(nums: &mut Vec<i32>, cur_idx: usize, set: &mut HashSet<Vec<i32>>) {
            if nums.len() <= cur_idx {
                set.insert(nums.to_vec());
                return;
            }

            for i in cur_idx..nums.len() {
                nums.swap(cur_idx, i);
                Self::helper(nums, cur_idx + 1, set);
                nums.swap(cur_idx, i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_sort::Solution::permute_unique);
    }

    fn test<F: Fn(Vec<i32>) -> Vec<Vec<i32>>>(func: F) {
        func(vec![1, 2, 2]);
    }
}
