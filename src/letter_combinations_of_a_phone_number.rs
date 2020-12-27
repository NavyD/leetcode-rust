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
    /// rust中string取出char没有直接Index的方式，对于ascii直接用byte数组。
    ///
    /// 使用path.len()可以替换cur_idx作为索引下个digit
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
                digits: &[u8],
                number_letters: &Vec<&str>,
                path: &mut String,
                res: &mut Vec<String>,
            ) {
                if path.len() == digits.len() {
                    res.push(path.clone());
                    return;
                }
                // 重建索引
                let letters = number_letters[digits[path.len()] as usize - '2' as usize];
                for letter in letters.chars() {
                    path.push(letter);
                    _backtrack(digits, number_letters, path, res);
                    path.pop();
                }
            }

            let mut res = vec![];
            if !digits.is_empty() {
                _backtrack(
                    digits.as_bytes(),
                    // number-letter: 2-abc, 3-def => 重建索引
                    &vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"],
                    &mut String::new(),
                    &mut res,
                );
            }
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
        fn test<F: Fn(String) -> Vec<String>>(func: F) {
            assert_eq!(func("".to_string()), vec![] as Vec<String>);
            let res = func("23".to_string());
            let expected = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>();
            assert_eq!(res.len(), expected.len());
            assert!(is_contains(&res, &expected));
        };
        test(solution_backtracking::Solution::letter_combinations);
    }
}
