//! 如何在递归时选择合适的参数找出当前path下允许的nums.
//! 如当nums=[1,2,3] path=0，选择num=2
//! 当nums=[1,2,3],path=1，如何选择num不出现重复
//!

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
    /// 下面这个是迭代方式
    /// 
    /// ```rust,ignore
    /// fn helper(nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    ///     if path.len() == nums.len() {
    ///         res.push(path.to_vec());
    ///         return;
    ///     }
    ///     for n in nums {
    ///         if path.iter().position(|v| v == n).is_none() {
    ///             path.push(*n);
    ///             Self::helper(nums, path, res);
    ///             path.pop();
    ///         }
    ///     }
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
    pub struct Solution;

    impl Solution {
        pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut res = vec![];
            Self::helper(&mut nums, 0, &mut res);
            res
        }

        fn helper(nums: &mut [i32], start: usize, res: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                res.push(nums.to_vec());
                return;
            }
            for i in start..nums.len() {
                nums.swap(start, i);
                Self::helper(nums, start + 1, res);
                nums.swap(start, i);
            }
        }
    }
}
