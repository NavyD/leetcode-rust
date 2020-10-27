pub mod solution_dfs {
    /// # 思路
    /// 
    /// 优化：分析搜索起点的上界进行剪枝
    /// 
    /// 参考：
    /// 
    /// - [回溯算法 + 剪枝（Java）](https://leetcode-cn.com/problems/combinations/solution/hui-su-suan-fa-jian-zhi-python-dai-ma-java-dai-ma-/581687/)
    /// 
    /// ### Submissions
    /// 
    /// date=20201027, mem=2.9, mem_beats=5.51, runtime=12, runtime_beats=58.46, url=https://leetcode-cn.com/submissions/detail/118929760/
    pub struct Solution;

    impl Solution {
        pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
            let mut res = vec![];
            if n >= k && k > 0 {
                Self::helper(n, k, 1, &mut vec![], &mut res);
            }
            res
        }

        fn helper(n: i32, k: i32, start: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if path.len() as i32 == k {
                res.push(path.to_vec());
                return;
            }
            let end = n - (k - path.len() as i32) + 1;
            for i in start..=end {
                path.push(i);
                Self::helper(n, k, i + 1, path, res);
                path.pop();
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn basic() {
//         let data = [[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]];
//         assert_eq!(solution_dfs::Solution::combine(4, 2), data);
//     }
// }
