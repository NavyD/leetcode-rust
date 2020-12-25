//! 如何在递归时选择合适的参数找出当前path下允许的nums.
//! 如当nums=[1,2,3] path=0，选择num=2
//! 当nums=[1,2,3],path=1，如何选择num不出现重复

/// 总结
/// 
/// 注意，无论使用`i`或`cur_idx`作为下层索引都无法正确找到
/// 
/// 使用i:`[[1,2,3]]`，由于i+1在后面迭代时会一直增加，递归到下面依然i在增加，
/// 不会在回到i=0
/// 
/// 使用cur_idx:`[[1,2,3],[1,3,3],[2,2,3],[2,3,3],[3,2,3],[3,3,3]]`，这个当
/// path=[1],cur_idx=1,i=2,path.push(3) => path=[1,3]递归下层后cur_idx=2 => path=[1,3,3]
/// 导致出现重复
/// 
/// ```ignore
/// fn _backtrack(
///     nums: &Vec<i32>,
///     cur_idx: usize,
///     path: &mut Vec<i32>,
///     res: &mut Vec<Vec<i32>>,
/// ) {
///     if cur_idx == nums.len() {
///         res.push(path.clone());
///         return;
///     }
///     for i in cur_idx..nums.len() {
///         path.push(nums[i]);
///         _backtrack(nums, cur_idx + 1, path, res);
///         path.pop();
///     }
/// }
/// ```
/// 
/// 使用这种通用回溯模板不行，从cur_idx开始迭代的方式会出现重复。
/// 
/// 在递归树中每层都选择全部元素，在去下层时过滤已选择的元素即可。
pub mod solution_dfs {
    /// # 思路
    ///
    /// 用递归回溯
    ///
    /// 如何选择num
    ///
    /// - 迭代查找
    /// - 标记：使用一个bool数组标记对应nums已被添加的下标
    /// - swap
    ///
    /// swap过的nums被分为两部分，左边的表示已经填过的数，右边表示待填的数，在递归到nums都被
    /// swap完成后表示nums是一个新的全排列
    ///
    /// 当[1,2,3]在第二层递归时，[1,2,3]要选择的是3到下层，则swap(1, 2) =>
    /// 到下层时nums= [1,3,2]。当第1层用选择的是2而不是1时，swap(0, 1) => 在第2层
    /// 时：nums=[2,1,3]
    ///
    /// 注意使用的是start: `Self::helper(nums, start + 1, res);`而不是i+1
    ///
    /// 下面这个是迭代方式
    ///
    /// ```rust,ignore
    /// pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    ///     fn _backtrack(nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    ///         if path.len() == nums.len() {
    ///             res.push(path.to_vec());
    ///             return;
    ///         }
    ///         for n in nums {
    ///             if !path.contains(n) {
    ///                 path.push(*n);
    ///                 _backtrack(nums, path, res);
    ///                 path.pop();
    ///             }
    ///         }
    ///     }
    ///     let mut res = vec![];
    ///     _backtrack(&nums, &mut vec![], &mut res);
    ///     res
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [全排列](https://leetcode-cn.com/problems/permutations/solution/quan-pai-lie-by-leetcode-solution-2/)
    /// - [回溯算法入门级详解 + 练习（持续更新）](https://leetcode-cn.com/problems/permutations/solution/hui-su-suan-fa-python-dai-ma-java-dai-ma-by-liweiw/)
    ///
    /// ### Submissions
    ///
    /// date=20201028, mem=2, mem_beats=68.89, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/119179633/
    ///
    /// date=20201031, mem=2, mem_beats=61.7, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/119955012/
    ///
    /// date=20201124, mem=2, mem_beats=67, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/125889758/
    /// 
    /// date=20201225, mem=2.1, mem_beats=51, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133659820/
    pub struct Solution;

    impl Solution {
        pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            fn _backtrack(nums: &mut Vec<i32>, cur_idx: usize, res: &mut Vec<Vec<i32>>) {
                if cur_idx == nums.len() {
                    res.push(nums.to_vec());
                    return;
                }
                for i in cur_idx..nums.len() {
                    nums.swap(cur_idx, i);
                    _backtrack(nums, cur_idx + 1, res);
                    nums.swap(cur_idx, i);
                }
            }
            let mut res = vec![];
            _backtrack(&mut nums, 0, &mut res);
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
            let expected = vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]];
            let res = func(vec![1,2,3]);
            assert!(is_contains_vec2(&res, &expected));
            assert!(is_contains_vec2(&expected, &res));
        }

        test(solution_dfs::Solution::permute);
    }
}
