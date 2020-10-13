//! 如何排列出n对括号，且这个n对都要合法
//!
//! n对表示有n个`(`和n个`)`，如果使用递归，每一层在都可以放`(`,`)`两种，
//! 可以有2*n个括号，这样就是在一个递归树上解释

pub mod solution_dfs {
    /// # 思路
    ///
    /// [pic](https://pic.leetcode-cn.com/7ec04f84e936e95782aba26c4663c5fe7aaf94a2a80986a97d81574467b0c513-LeetCode%20%E7%AC%AC%2022%20%E9%A2%98%EF%BC%9A%E2%80%9C%E6%8B%AC%E5%8F%B7%E7%94%9F%E5%87%BA%E2%80%9D%E9%A2%98%E8%A7%A3%E9%85%8D%E5%9B%BE.png)
    /// 
    /// 当前左右括号都有大于 0 个可以使用的时候，才产生分支；
    /// 
    /// - 产生左分支的时候，只看当前是否还有左括号可以使用；
    /// - 产生右分支的时候，还受到左分支的限制，右边剩余可以使用的括号数量一定得在严格大于左边剩余的数量的时候，才可以产生分支；
    /// - 在左边和右边剩余的括号数都等于 0 的时候结算。
    /// 
    /// 参考：
    /// 
    /// - [回溯算法（深度优先遍历）+ 广度优先遍历 + 动态规划](https://leetcode-cn.com/problems/generate-parentheses/solution/hui-su-suan-fa-by-liweiwei1419/)
    /// 
    /// ### Submissions
    /// 
    /// date=20201013, mem=2.2, mem_beats=5.88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115365774/
    pub struct Solution;
    impl Solution {
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            let mut res = vec![];
            Self::generate(0, 0, "".to_string(), n, &mut res);
            res
        }

        fn generate(left: i32, right: i32, s: String, n: i32, res: &mut Vec<String>) {
            if left == n && right == n {
                res.push(s);
                return;
            }

            if left < n {
                Self::generate(left + 1, right, s.clone() + "(", n, res);
            }
            if right < left {
                Self::generate(left, right + 1, s.clone() + ")", n, res);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::generate_parenthesis);
    }

    fn test<F: Fn(i32) -> Vec<String>>(func: F) {
        assert!(is_equals(
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ],
            func(3)
        ))
    }

    /// 如果a与b长度一样且内容一样无序，则返回true
    fn is_equals(a: Vec<String>, b: Vec<String>) -> bool {
        if a.len() != b.len() {
            false
        } else {
            let mut set = std::collections::HashSet::new();
            a.into_iter().for_each(|s| {
                set.insert(s);
            });
            b.iter().for_each(|s| {
                set.remove(s);
            });
            set.is_empty()
        }
    }
}
