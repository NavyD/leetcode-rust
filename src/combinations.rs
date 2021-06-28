//! 如何找组合
//!
//! 如何剪枝

/// 总结
///
/// 回溯：
///
/// 关键在于没有搞清如何传入下层cur的是什么：cur+1还是i+1。
///
/// ```ignore
///
/// cur:    [[1,2],[1,3],[1,4],[2,2],[2,3],[2,4],[3,2],[3,3],[3,4],[4,2],[4,3],[4,4]]
/// i:      [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
/// ```
///
/// cur+1会在第一层cur=1时表现正常，当cur=2时会出现重复。第一层是cur=1，i=2，下层就是
/// cur=2,i=2开始
///
/// 但如果是用i传入。后面可选的就只有i+1..n，下层不会和上层出现重复
///
/// 剪枝：当还要的元素数量k - path.len() > 剩下的数量 n - cur时提前退出
pub mod solution_dfs {
    /// # 思路
    ///
    /// 对每一层递归回溯
    ///
    /// ![](https://pic.leetcode-cn.com/1599488203-TzmCXb-image.png)
    ///
    /// ```rust
    /// # use leetcode_rust::utils::is_contains_vec2;
    /// pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    ///     fn _backtrack(n: i32, k: i32, path: &mut Vec<i32>, cur_idx: i32, res: &mut Vec<Vec<i32>>) {
    ///         if path.len() == k as usize {
    ///             res.push(path.clone());
    ///             return;
    ///         }
    ///         for i in cur_idx..=n {
    ///             path.push(i);
    ///             _backtrack(n, k, path, i + 1, res);
    ///             path.pop();
    ///         }
    ///     }
    ///     let mut res = vec![];
    ///     _backtrack(n, k, &mut vec![], 1, &mut res);
    ///     res
    /// }
    ///
    /// assert!(is_contains_vec2(
    ///     &[[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    ///         .iter()
    ///         .map(|e| e.to_vec())
    ///         .collect::<Vec<_>>(),
    ///     &combine(4, 2)
    /// ));
    /// ```
    ///
    /// 优化：分析搜索起点的上界进行剪枝
    ///
    /// 当n = 6 ，k = 4：
    ///
    /// 1. 在第一层时如果当cur_idx=4时，后面可选的4,5,6不够4个数。基础上界：`cur_idx >= n - k + 1`时退出
    ///
    /// 2. 同样，在到第i层时同样会出现不够数的情况。在第i层要需要选择的个数`k - path.len()`，当i层
    /// 剩下的元素数量不够时返回：`cur_idx + k - path.len() + 1 >= n`
    ///
    /// ![](https://pic.leetcode-cn.com/3ddd55697423b5831cbbd42f4b901ebbade0daa456c651a70c758fe359d8a0d1-image.png)
    ///
    /// - path.size() == 1 的时候，接下来要选择 3 个数，搜索起点最大是 4，最后一个被选的组合是 [4, 5, 6]；
    /// - path.size() == 2 的时候，接下来要选择 2 个数，搜索起点最大是 5，最后一个被选的组合是 [5, 6]；
    /// - path.size() == 3 的时候，接下来要选择 1 个数，搜索起点最大是 6，最后一个被选的组合是 [6]；
    ///
    /// `搜索起点的上界 + 接下来要选择的元素个数 - 1 = n`  => `搜索起点的上界 = n - (k - path.size()) + 1`
    ///
    /// 另外，可以使用k作为终止条件，与上面的上界条件是一样的
    ///
    /// ```ignore
    /// if k == 0 {
    ///     res.push(path.clone());
    ///     return;
    /// }
    /// // 第二步：
    /// for i in start..(n - k + 1) {
    ///     path.push(i + 1);
    ///     backtrack(n, k - 1, i + 1, path, res);
    ///     path.pop();
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [回溯算法 + 剪枝（Java）](https://leetcode-cn.com/problems/combinations/solution/hui-su-suan-fa-jian-zhi-python-dai-ma-java-dai-ma-/581687/)
    ///
    /// 在leetcode上还有一种更好的方法，但是还是回溯有思想，有时间再掌握[Short Iterative C++ Answer 8ms](https://leetcode.com/problems/combinations/discuss/26992/Short-Iterative-C%2B%2B-Answer-8ms)
    ///
    /// ### Submissions
    ///
    /// date=20201027, mem=2.9, mem_beats=5.51, runtime=12, runtime_beats=58.46, url=https://leetcode-cn.com/submissions/detail/118929760/
    ///
    /// date=20201028, mem=2.7, mem_beats=98.43, runtime=8, runtime_beats=99, url=https://leetcode-cn.com/submissions/detail/119148220/
    ///
    /// date=20201222, mem=2.8, mem_beats=72, runtime=8, runtime_beats=99, url=https://leetcode-cn.com/submissions/detail/133001533/
    ///
    /// date=20201222, mem=2.9, mem_beats=28, runtime=8, runtime_beats=97, url=https://leetcode-cn.com/submissions/detail/135964556/
    pub struct Solution;

    impl Solution {
        pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
            fn _backtrack(
                n: i32,
                k: i32,
                cur_idx: i32,
                path: &mut Vec<i32>,
                res: &mut Vec<Vec<i32>>,
            ) {
                if path.len() == k as usize {
                    res.push(path.clone());
                    return;
                }
                for i in cur_idx..=n - (k - path.len() as i32) + 1 {
                    path.push(i);
                    _backtrack(n, k, i + 1, path, res);
                    path.pop();
                }
            }
            let mut res = vec![];
            _backtrack(n, k, 1, &mut vec![], &mut res);
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
        fn test<F: Fn(i32, i32) -> Vec<Vec<i32>>>(func: F) {
            assert!(is_contains_vec2(
                &[[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
                    .iter()
                    .map(|e| e.to_vec())
                    .collect::<Vec<_>>(),
                &func(4, 2)
            ));
        }
        test(solution_dfs::Solution::combine);
    }
}
