//! 如何排列出n对括号，且这个n对都要合法
//!
//! n对表示有n个`(`和n个`)`，如果使用递归，每一层在都可以放`(`,`)`两种，
//! 可以有2*n个括号，这样就是在一个递归树上解释

pub mod solution_dfs {
    /// # 思路
    ///
    /// 在一个添加左括号后添加对应的右括号。利用dfs递归时添加括号时回溯找出
    /// 所有括号
    ///
    /// ![](https://pic.leetcode-cn.com/7ec04f84e936e95782aba26c4663c5fe7aaf94a2a80986a97d81574467b0c513-LeetCode%20%E7%AC%AC%2022%20%E9%A2%98%EF%BC%9A%E2%80%9C%E6%8B%AC%E5%8F%B7%E7%94%9F%E5%87%BA%E2%80%9D%E9%A2%98%E8%A7%A3%E9%85%8D%E5%9B%BE.png)
    ///
    /// 当前左右括号都有大于 0 个可以使用的时候，才产生分支；
    ///
    /// - 产生左分支的时候，只受n个左括号的限制
    /// - 产生右分支的时候，还受到左分支的限制，右边剩余可以使用的括号数量一定得在严格大于左边剩余的数量的时候，才可以产生分支；
    /// - 在左边和右边剩余的括号数用完时generate完成
    ///
    /// `if right < left`可限制`right<left<n`
    ///
    /// 可以有对树加法或减法，上面是用减法，下面是加法
    ///
    /// 注意：谨慎使用`s: &mut String`作为generate参数，可能导致下层递归被上层的环境被影响`s.push('(')`，在同层时
    /// s.push前应该clone: s.clone().push('(')，不能影响同层后面的right写为`()`导致错误
    ///
    /// `fn generate(left: i32, right: i32, n: i32, res: &mut Vec<String>, s: &mut String) {`
    ///
    /// ![](https://pic.leetcode-cn.com/efbe574e5e6addcd1c9dc5c13a50c6f162a2b14a95d6aed2c394e18287a067fa-image.png)
    ///
    /// 参考：
    ///
    /// - [回溯算法（深度优先遍历）+ 广度优先遍历 + 动态规划](https://leetcode-cn.com/problems/generate-parentheses/solution/hui-su-suan-fa-by-liweiwei1419/)
    ///
    /// ### Submissions
    ///
    /// date=20201013, mem=2.2, mem_beats=5.88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115365774/
    ///
    /// date=20201014, mem=2.1, mem_beats=61.76, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/115655272/
    ///
    /// date=20201025, mem=2.2, mem_beats=47.5, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/118434839/
    ///
    /// date=20201222, mem=2.2, mem_beats=25, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132817432/
    ///
    /// date=20201223, mem=2.1, mem_beats=85, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133181240/
    ///
    /// ### 复杂度
    ///
    /// 参考：[括号生成 复杂度](https://leetcode-cn.com/problems/generate-parentheses/solution/gua-hao-sheng-cheng-by-leetcode-solution/)
    pub struct Solution;

    impl Solution {
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            const LEFT_PARENTHESIS: char = '(';
            const RIGHT_PARENTHESIS: char = ')';

            fn _backtrack(
                left_count: i32,
                right_count: i32,
                path: &mut String,
                res: &mut Vec<String>,
            ) {
                if left_count == 0 && right_count == 0 {
                    res.push(path.clone());
                    return;
                }

                if left_count > 0 {
                    path.push(LEFT_PARENTHESIS);
                    _backtrack(left_count - 1, right_count, path, res);
                    path.pop();
                }
                if right_count > left_count {
                    path.push(RIGHT_PARENTHESIS);
                    _backtrack(left_count, right_count - 1, path, res);
                    path.pop();
                }
            }

            let mut res = vec![];
            _backtrack(n, n, &mut String::new(), &mut res);
            res
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// 下面是n=3的过程中queue结果
    ///
    /// ```ignore
    /// 0:
    /// (
    /// ((
    /// ()
    /// 1:
    /// (((
    /// (()
    /// ()(
    /// 2:
    /// ((()
    /// (()(
    /// (())
    /// ()((
    /// ()()
    /// 3:
    /// ((())
    /// (()()
    /// (())(
    /// ()(()
    /// ()()(
    /// 4: result=
    /// ((()))
    /// (()())
    /// (())()
    /// ()(())
    /// ()()()
    /// ```
    ///
    /// 下面这个部分更容易理解
    ///
    /// ```ignore
    /// let mut n = n * 2;
    /// while n > 0 {
    ///     n -= 1;
    ///     for _ in 0..queue.len() {
    ///         let node = queue.pop_front().unwrap();
    ///         if node.left_count > 0 {
    ///             let mut next = node.clone();
    ///             next.parentheses.push(LEFT_PARENTHESIS);
    ///             next.left_count -= 1;
    ///             queue.push_back(next);
    ///         }
    ///         if node.right_count > node.left_count {
    ///             let mut next = node;
    ///             next.parentheses.push(RIGHT_PARENTHESIS);
    ///             next.right_count -= 1;
    ///             queue.push_back(next);
    ///         }
    ///     }
    /// }
    /// // 最后一层就是题目要求的结果集
    /// while let Some(node) = queue.pop_front() {
    ///     res.push(String::from_utf8(node.parentheses).unwrap());
    /// }
    /// ```
    ///
    /// 注意：在while中的node.parentheses使用任何顺序都是可以的，如下面的方式。
    /// 只要node的count计数正确
    ///
    /// ```rust,ignore
    /// while !stack.is_empty() {
    //      for _ in 0..stack.len() {
    //          let node = stack.pop().unwrap();
    //          if node.left_count == 0 && node.right_count == 0 {
    //              res.push(node.parentheses);
    //              continue;
    //          }
    //          if node.left_count > 0 {
    //              let mut next = node.clone();
    //              next.parentheses.push(LEFT_PARENTHESIS);
    //              next.left_count -= 1;
    //              stack.push(next);
    //          }
    //          if node.right_count > node.left_count {
    //              let mut next = node;
    //              next.parentheses.push(RIGHT_PARENTHESIS);
    //              next.right_count -= 1;
    //              stack.push(next);
    //          }
    //      }
    //  }
    /// ```
    ///
    /// 参考：
    ///
    /// - [回溯算法（深度优先遍历）+ 广度优先遍历 + 动态规划](https://leetcode-cn.com/problems/generate-parentheses/solution/hui-su-suan-fa-by-liweiwei1419/)
    ///
    /// ### Submissions
    ///
    /// date=20201222, mem=2.2, mem_beats=22, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132839666/
    ///
    /// date=20201223, mem=2.2, mem_beats=53, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133191328/
    pub struct Solution;

    impl Solution {
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            const LEFT_PARENTHESIS: char = '(';
            const RIGHT_PARENTHESIS: char = ')';
            // 表示一个生成的括号
            #[derive(Clone)]
            struct Node {
                parentheses: String,
                left_count: i32,
                right_count: i32,
            }
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(Node {
                parentheses: String::new(),
                left_count: n,
                right_count: n,
            });
            let mut res = vec![];
            while let Some(node) = queue.pop_front() {
                // node结果完成
                if node.left_count == 0 && node.right_count == 0 {
                    res.push(node.parentheses.clone());
                    continue;
                }
                // 左括号
                if node.left_count > 0 {
                    let mut next = node.clone();
                    next.parentheses.push(LEFT_PARENTHESIS);
                    next.left_count -= 1;
                    queue.push_back(next);
                }
                // 右括号
                if node.right_count > node.left_count {
                    let mut next = node;
                    next.parentheses.push(RIGHT_PARENTHESIS);
                    next.right_count -= 1;
                    queue.push_back(next);
                }
            }
            res
        }
    }
}

#[allow(unused)]
pub mod solution_dp {
    pub struct Solution;

    impl Solution {
        pub fn generate_parenthesis(n: i32) -> Vec<String> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::generate_parenthesis);
        test(solution_bfs::Solution::generate_parenthesis);
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
            func(3),
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
