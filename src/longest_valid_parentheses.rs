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
            fn is_valid(s: &str, stack: &mut Vec<()>) -> bool {
                stack.clear();
                for c in s.chars() {
                    if c == LEFT {
                        stack.push(());
                    } else if stack.pop().is_none() {
                        // 右括号 未对应left
                        return false;
                    }
                }
                stack.is_empty()
            }

            let n = s.len();
            let mut stack = vec![];

            // the length. from large to small, step = 2 and len is even
            for len in (2..=if n & 1 == 0 { n } else { n - 1 }).rev().step_by(2) {
                // get the slice of the length
                for i in 0..=n - len {
                    // check if the slice is valid
                    if is_valid(&s[i..i + len], &mut stack) {
                        return len as i32;
                    }
                }
            }
            0
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// 设dp[i]表示在以下标为 i 的字符结尾的最长有效子括号的长度。对于第i位置的元素：
    ///
    /// - s[i]==`(`：无法找到对应的`)`括号，dp[i]=0
    /// - s[i]==`)`：需要前面对元素来判断是否有有效括号对。
    ///     - s[i-1]==`(`，则i-1与i组成一对括号，
    ///     i位置对最长有效括号长度为 其之前2个位置的最长括号长度
    ///     加上 当前位置新增的2，dp[i] = dp[i-2]+2.
    ///     ![](https://pic.leetcode-cn.com/6f176074b305e1571da1ab74839d22436be5fba22b592d618d531ac79dae8a7a-%E6%88%AA%E5%B1%8F2020-04-17%E4%B8%8B%E5%8D%884.30.46.png)
    ///     - s[i-1]==`)`，要形成有效的括号对`((..))`要求i对应位置`i-dp[i-1]-1`必须是`(`，
    ///     i位置对最长有效括号长度为 i-1位置的最长括号长度 加上 当前位置新增的2。dp[i]=dp[i-1]+2
    ///     *这里不要求对应i-1位置是有效的，由于dp[i-1]可以不是有效的？如果i-1不是有效子括号，则
    ///     dp[i-1]=0, i-dp[i-1]-1=i-1，而i-1就是`)`无法与i `)`形成括号*。
    ///     如果i位置是有效括号，可能会与之前`i-dp[i-1]-2`形成连续括号dp[i]=dp[i-1]+2+dp[i-dp[i-1]-2]
    ///     ![](https://pic.leetcode-cn.com/6e07ddaac3b703cba03a9ea8438caf1407c4834b7b1e4c8ec648c34f2833a3b9-%E6%88%AA%E5%B1%8F2020-04-17%E4%B8%8B%E5%8D%884.26.34.png)
    ///
    /// dp方程：
    ///
    /// ```ignore
    /// if s[i] == '(' :
    ///     dp[i] = 0
    /// else:
    ///     if s[i - 1] == '(' :
    ///         dp[i] = dp[i - 2] + 2 #要保证i - 2 >= 0
    ///
    ///     if s[i - 1] == ')' and s[i - dp[i - 1] - 1] == '(' :
    ///         dp[i] = dp[i - 1] + dp[i - dp[i - 1] - 2] + 2 #要保证i - dp[i - 1] - 2 >= 0
    /// ```
    ///
    /// 初始化：dp.len = s.len, dp[i] = 0, 需要保证计算过程中：i−2>=0 和 i - dp[i - 1] - 2 >= 0
    ///
    /// 参考：
    ///
    /// - [动态规划思路详解（c++）——32.最长有效括号](https://leetcode-cn.com/problems/longest-valid-parentheses/solution/dong-tai-gui-hua-si-lu-xiang-jie-c-by-zhanganan042/)
    /// - [最长有效括号](https://leetcode-cn.com/problems/longest-valid-parentheses/solution/zui-chang-you-xiao-gua-hao-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20210731, mem=2, mem_beats=80, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/201642981/
    ///
    /// date=20210807, mem=2.3, mem_beats=17, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204266408/
    pub struct Solution;

    impl Solution {
        pub fn longest_valid_parentheses(s: String) -> i32 {
            const LEFT: u8 = b'(';
            let s = s.as_bytes();
            let mut dp = vec![0; s.len()];
            let mut res = 0;

            for i in 1..s.len() {
                if s[i] != LEFT {
                    if s[i - 1] == LEFT {
                        dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
                    } else if i > dp[i - 1] {
                        let last = i - dp[i - 1] - 1;
                        if s[last] == LEFT {
                            dp[i] = dp[i - 1] + 2 + if last > 0 { dp[last - 1] } else { 0 }
                        }
                    }
                }
                res = res.max(dp[i]);
            }
            res as i32
        }
    }
}

pub mod solution_stack {
    /// # 思路
    ///
    /// 在栈中预置 -1 作为“参照物”，并改变计算方式：当前索引 - 出栈后新的栈顶索引。
    ///
    /// 两种索引会入栈
    ///
    ///![](https://pic.leetcode-cn.com/5d7c8630b67841475a97775c870fdb63cdfa317ce236a3335667700c5ac5f99f-image.png)
    ///
    /// - 等待被匹配的左括号索引。当i=0时，stack:[-1], push(i=0)。当i=1时pop=0，cur_len=i-stack.last()=1-(-1)=2，
    /// 当i=4时，stack:[-1,2,3]，pop=3，cur_len=i-stack.last()=4-2=2，当i=5时pop=2,stack:[-1]，cur_len=5-(-1)=6
    /// - 充当「参照物」的右括号索引。因为：当左括号匹配光时，栈需要留一个垫底的参照物，用于计算一段连续的有效长度。
    /// 当i=6时，stack为空，push(i=6)，当i=8时pop，stack.len=1:[6]，cur_len=i-stack.last()=8-6=2
    ///
    /// 参考：
    ///
    /// - [「手画图解」剖析两种解法：利用栈、动态规划 | 32. 最长有效括号](https://leetcode-cn.com/problems/longest-valid-parentheses/solution/shou-hua-tu-jie-zhan-de-xiang-xi-si-lu-by-hyj8/)
    /// - [最长有效括号](https://leetcode-cn.com/problems/longest-valid-parentheses/solution/zui-chang-you-xiao-gua-hao-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20210731, mem=2.2, mem_beats=46, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/201648180/
    ///
    /// date=20210807, mem=2, mem_beats=85, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204272959/
    pub struct Solution;

    impl Solution {
        pub fn longest_valid_parentheses(s: String) -> i32 {
            let (mut res, mut stack) = (0, vec![-1]);
            let s = s.as_bytes();
            for i in 0..s.len() {
                if s[i] == b'(' {
                    stack.push(i as i32);
                } else {
                    stack.pop();
                    let i = i as i32;
                    if stack.is_empty() {
                        stack.push(i);
                    } else {
                        res = res.max(i - stack.last().unwrap());
                    }
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_violent::Solution::longest_valid_parentheses);
        test(solution_dp::Solution::longest_valid_parentheses);
        test(solution_stack::Solution::longest_valid_parentheses);
    }

    fn test<F: Fn(String) -> i32>(f: F) {
        assert_eq!(f("(()".to_string()), 2);
        assert_eq!(f("())".to_string()), 2);
        assert_eq!(f(")()())".to_string()), 4);
        assert_eq!(f("".to_string()), 0);
    }
}
