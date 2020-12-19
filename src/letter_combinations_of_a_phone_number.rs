/// 总结
///
/// 类似于全排列中的回溯法
///
/// 在第一次写的时候，考虑到了使用backtracking的方式对digit递归，
/// 对每个digit到最后达到len==cur_idx时对path中迭代找出各个排列。
///
/// 问题出在最后不能对path中的digits进行迭代次数找出排列，如累计了234要
/// 找出letters要怎么组合，这个234是不定长度的。
/// 必须在递归时一起排列了
pub mod solution_backtracking {
    /// # 思路
    ///
    /// 递归回溯树：
    ///
    /// ![](https://pic.leetcode-cn.com/1598406872-BTOpqw-image.png)
    ///
    /// 参考：
    ///
    /// [「手画图解」两种解法：DFS回溯、BFS。我理解的回溯](https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/solution/shou-hua-tu-jie-liang-chong-jie-fa-dfshui-su-bfsya/)
    ///
    /// 可以使用macro对literal简化map.insert方式
    ///
    /// ```ignore
    /// #[macro_export]
    /// macro_rules! map {
    ///     ( $( $k:tt:  $v:tt),* ) => {
    ///         {
    ///             let mut map = std::collections::HashMap::new();
    ///             $(
    ///                 map.insert($k, $v);
    ///             )*
    ///             map
    ///         }
    ///     };
    /// }
    /// // use macro
    /// map!{ '2': "abc", '3': "def" }
    /// ```
    /// 
    /// rust中string取出char没有直接Index的方式，对于ascii直接用byte数组
    ///
    /// ### Submissions
    ///
    /// date=20201218, mem=2.2, mem_beats=5, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132095917/
    /// 
    /// date=20201219, mem=1.9, mem_beats=78, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132222590/
    pub struct Solution;

    impl Solution {
        pub fn letter_combinations(digits: String) -> Vec<String> {
            fn _backtrack(
                number_letters: &Vec<&str>,
                digits: &[u8],
                cur_idx: usize,
                path: &mut Vec<u8>,
                res: &mut Vec<String>,
            ) {
                if cur_idx == digits.len() {
                    res.push(String::from_utf8(path.clone()).unwrap());
                    return;
                }
                let digit = digits[cur_idx];
                // 重建索引
                let letters: &str = number_letters[digit as usize - '2' as usize];
                letters.bytes().for_each(|letter| {
                    path.push(letter);
                    _backtrack(number_letters, digits, cur_idx + 1, path, res);
                    path.pop();
                });
            }
            let mut res = vec![];
            if !digits.is_empty() {
                // number-letter: 2-abc, 3-def => 重建索引
                let number_letters = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
                _backtrack(&number_letters, digits.as_bytes(), 0, &mut vec![], &mut res);
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        fn test<F: Fn(String) -> Vec<String>>(func: F) {
            assert_eq!(func("".to_string()), vec![] as Vec<String>);
        };
        test(solution_backtracking::Solution::letter_combinations);
    }
}
