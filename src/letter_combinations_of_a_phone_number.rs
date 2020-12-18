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
    /// ### Submissions
    ///
    /// date=20201218, mem=2.2, mem_beats=5, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132095917/
    pub struct Solution;

    impl Solution {
        pub fn letter_combinations(digits: String) -> Vec<String> {
            use std::collections::HashMap;
            fn _backtrack(
                digits: &Vec<char>,
                cur_idx: usize,
                number_letters: &HashMap<char, String>,
                path: &mut String,
                res: &mut Vec<String>,
            ) {
                if cur_idx == digits.len() {
                    res.push(path.clone());
                    return;
                } else {
                    let letters = number_letters.get(&digits[cur_idx]).unwrap();
                    for letter in letters.chars() {
                        path.push(letter);
                        _backtrack(digits, cur_idx + 1, number_letters, path, res);
                        path.pop();
                    }
                }
            }

            let mut res = vec![];
            if digits.is_empty() {
                return res;
            }
            let number_letters = [
                ('2', "abc"),
                ('3', "def"),
                ('4', "ghi"),
                ('5', "jkl"),
                ('6', "mno"),
                ('7', "pqrs"),
                ('8', "tuv"),
                ('9', "wxyz"),
            ]
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_string()))
            .collect::<HashMap<char, String>>();
            _backtrack(
                &digits.chars().collect(),
                0,
                &number_letters,
                &mut String::new(),
                &mut res,
            );
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
