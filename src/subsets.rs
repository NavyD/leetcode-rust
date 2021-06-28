//! 如何找到一个集合的所有不重复的子元素集合并去重
//!
//! 回溯算法可以从单个元素列出递归树找到该元素下所有不重复的
//! 子元素集合

pub mod solution_recursive {
    /// # 思路
    ///
    /// 参考：
    ///
    /// - [回溯思想团灭排列、组合、子集问题](https://leetcode-cn.com/problems/subsets/solution/hui-su-si-xiang-tuan-mie-pai-lie-zu-he-zi-ji-wen-t/)
    ///
    /// ### Submissions
    ///
    /// date=20201215, mem=2.1, mem_beats=21, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/131175081/
    ///
    /// date=20201217, mem=2.1, mem_beats=30, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/131761719/
    ///
    /// date=20201223, mem=2.1, mem_beats=16, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133216203/
    pub struct Solution;

    impl Solution {
        pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
            fn _backtrack(
                nums: &Vec<i32>,
                cur_idx: usize,
                path: &mut Vec<i32>,
                res: &mut Vec<Vec<i32>>,
            ) {
                res.push(path.clone());
                for i in cur_idx..nums.len() {
                    path.push(nums[i]);
                    _backtrack(nums, i + 1, path, res);
                    path.pop();
                }
            }
            let mut res = vec![];
            _backtrack(&nums, 0, &mut vec![], &mut res);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn basic() {
        fn test<F: Fn(Vec<i32>) -> Vec<Vec<i32>>>(func: F) {
            assert!(is_contains_vec2(
                &vec![
                    vec![3],
                    vec![1],
                    vec![2],
                    vec![1, 2, 3],
                    vec![1, 3],
                    vec![2, 3],
                    vec![1, 2],
                    vec![]
                ],
                &func(vec![1, 2, 3]),
            ));
        }
        test(solution_recursive::Solution::subsets);
    }
}
