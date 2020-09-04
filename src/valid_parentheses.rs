/// 总结
///
/// stack push 相对字符这个思路确实厉害，代码精简，
/// 相反，my_solution是一般的方法，且rust api运用不
/// 熟悉
pub mod solution_stack {

    /// # 思路
    ///
    /// 若遇到左括号入栈，遇到右括号时将对应栈顶左括号出栈，
    /// 则遍历完所有括号后 stack 仍然为空
    /// 
    /// stack保持出现的相对的括号，在遇到相对括号时，stack pop对比是否一样
    ///
    /// 参考：
    ///
    /// - [Short java solution](https://leetcode.com/problems/valid-parentheses/discuss/9178/Short-java-solution)
    /// - [有效的括号（辅助栈法，极简+图解）](https://leetcode-cn.com/problems/valid-parentheses/solution/valid-parentheses-fu-zhu-zhan-fa-by-jin407891080/)
    /// 
    /// ### Submissions
    ///
    /// date=20200903, mem=2.1, mem_beats=20.88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104330959/
    ///
    /// date=20200904, mem=2.2, mem_beats=7.69, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104667778/
    /// 
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn is_valid(s: String) -> bool {
            let mut stack = std::collections::VecDeque::new();
            for left in s.chars() {
                if left == '(' {
                    stack.push_back(')');
                } else if left == '[' {
                    stack.push_back(']');
                } else if left == '{' {
                    stack.push_back('}');
                } else if stack.is_empty() || left != stack.pop_back().unwrap() {
                    return false;
                }
            }
            stack.is_empty()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert!(Solution::is_valid("()".to_string()));
            assert!(Solution::is_valid("()[]{}".to_string()));
            assert!(!Solution::is_valid("(]".to_string()));
            assert!(!Solution::is_valid("([]".to_string()));
        }
    }
}

/// # 思路
///
/// stack保持出现的字符left，出现right时匹配stack当前是否一样
///
/// ### Submissions
///
/// date=20200903, mem=2.2, mem_beats=7.69, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104327176/
///
/// ### 复杂度
///
/// - 时间：O(n)
/// - 空间：O(n)
pub mod solution_my {
    pub struct Solution;

    impl Solution {
        pub fn is_valid(s: String) -> bool {
            let mut stack = std::collections::VecDeque::new();
            let bytes = s.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] != b')' && bytes[i] != b'}' && bytes[i] != b']' {
                    stack.push_back(bytes[i]);
                } else if let Some(left) = stack.pop_back() {
                    let right = bytes[i];
                    if (left == b'(' && right != b')')
                        || (left == b'{' && right != b'}')
                        || (left == b'[' && right != b']')
                    {
                        return false;
                    }
                } else {
                    return false;
                }
                i += 1;
            }
            stack.is_empty()
        }

        /// [Rust, 0ms,2MB, 100%, 81.25%](https://leetcode-cn.com/problems/valid-parentheses/solution/rust-0ms2mb-100-8125-by-shu-53/)
        pub fn is_valid1(s: String) -> bool {
            let mut stack = Vec::new();
            for ch in s.chars() {
                match ch {
                    '(' | '[' | '{' => stack.push(ch),
                    ')' | ']' | '}'
                        if ch
                            == match stack.pop() {
                                Some('(') => ')',
                                Some('[') => ']',
                                Some('{') => '}',
                                _ => '*',
                            } =>
                    {
                        continue
                    }
                    _ => return false,
                }
            }
            stack.is_empty()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert!(Solution::is_valid("()".to_string()));
            assert!(Solution::is_valid("()[]{}".to_string()));
            assert!(!Solution::is_valid("(]".to_string()));
            assert!(!Solution::is_valid("([]".to_string()));
        }
    }
}
