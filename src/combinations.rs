//! 如何找组合
//!
//! 如何剪枝

pub mod solution_dfs {
    /// # 思路
    ///
    /// 对每一层递归回溯
    ///
    /// ![](https://pic.leetcode-cn.com/1599488203-TzmCXb-image.png)
    /// 
    /// 优化：分析搜索起点的上界进行剪枝
    ///
    /// 当n = 6 ，k = 4：
    ///
    /// - path.size() == 1 的时候，接下来要选择 3 个数，搜索起点最大是 4，最后一个被选的组合是 [4, 5, 6]；
    /// - path.size() == 2 的时候，接下来要选择 2 个数，搜索起点最大是 5，最后一个被选的组合是 [5, 6]；
    /// - path.size() == 3 的时候，接下来要选择 1 个数，搜索起点最大是 6，最后一个被选的组合是 [6]；
    ///
    /// `搜索起点的上界 + 接下来要选择的元素个数 - 1 = n`  => `搜索起点的上界 = n - (k - path.size()) + 1`
    ///
    /// 参考：
    ///
    /// - [回溯算法 + 剪枝（Java）](https://leetcode-cn.com/problems/combinations/solution/hui-su-suan-fa-jian-zhi-python-dai-ma-java-dai-ma-/581687/)
    ///
    /// ### Submissions
    ///
    /// date=20201027, mem=2.9, mem_beats=5.51, runtime=12, runtime_beats=58.46, url=https://leetcode-cn.com/submissions/detail/118929760/
    /// 
    /// date=20201028, mem=2.7, mem_beats=98.43, runtime=8, runtime_beats=99, url=https://leetcode-cn.com/submissions/detail/119148220/
    pub struct Solution;

    impl Solution {
        pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
            let mut res = vec![];
            Self::helper(n, k, 1, &mut vec![], &mut res);
            res
        }

        fn helper(n: i32, k: i32, start: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if path.len() as i32 == k {
                res.push(path.to_vec());
                return;
            }
            let end = n - (k - path.len() as i32) + 1;
            for start in start..=end {
                path.push(start);
                Self::helper(n, k, start + 1, path, res);
                path.pop();
            }
        }
    }
}
