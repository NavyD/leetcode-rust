pub mod solution_violent {
    /// # 思路
    ///
    /// 暴力解法：在s中用最大的子字符串长度从0找起，如果是合法的则返回子字符串的长度
    ///
    /// 合法子字符串一定是偶数：`(0..=if n % 2 == 0 { n } else { n - 1 }).rev().step_by(2)`
    ///
    /// `for c in s.chars() {`的解释：
    ///
    /// ```ignore
    /// if c == LEFT {
    ///     stack.push(c);
    /// }
    /// // 遇到RIGHT，如果 stack存在元素时表示一定是一对`()`取出
    /// else if !stack.is_empty() && *stack.last().unwrap() == LEFT {
    ///     stack.pop();
    /// } else {
    ///     return false;
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// ac超时：228 / 231 个通过测试用例
    pub struct Solution;

    impl Solution {
        pub fn longest_valid_parentheses(s: String) -> i32 {
            const LEFT: char = '(';
            fn is_valid(s: &str, stack: &mut Vec<char>) -> bool {
                stack.clear();
                for c in s.chars() {
                    if c == LEFT {
                        stack.push(c);
                    } else if stack.pop().is_none() {
                        // 右括号 未对应left
                        return false;
                    }
                }
                stack.is_empty()
            }

            let n = s.len();
            if n < 2 {
                return 0;
            }

            let mut stack = vec![];
            // 子字符串长度
            for i in (0..=if n % 2 == 0 { n } else { n - 1 }).rev().step_by(2) {
                // 在s中 从0开始遍历这个长度的字符串
                for j in 0..=n - i {
                    if is_valid(&s[j..j + i], &mut stack) {
                        return i as i32;
                    }
                }
            }
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_violent::Solution::longest_valid_parentheses);
    }

    fn test<F: Fn(String) -> i32>(f: F) {
        assert_eq!(f("(()".to_string()), 2);
        assert_eq!(f(")()())".to_string()), 4);
        assert_eq!(f("".to_string()), 0);
    }
}
