pub mod solution_backtracking {
    /// # 思路
    ///
    /// 删除被匹配的单词：
    ///
    /// 当board[i][j]全是a且单词列表 `["a", "aa", "aaa", "aaaa"]`时，每次i+1,j+1都会
    /// 找到a，需要去重。可以使用hashset对结果去重或回溯时移除树单词
    ///
    /// 匹配路径：
    ///
    /// 回溯时不需要考虑board[i][j]当前是否是trie中词的前缀，只需要看是否对应子结点word一样
    /// 参考：
    ///
    /// * [【宫水三叶】一题双解 :「回溯」&「Trie 」](https://leetcode-cn.com/problems/word-search-ii/solution/gong-shui-san-xie-yi-ti-shuang-jie-hui-s-am8f/)
    /// * [My simple and clean Java code using DFS and Trie](https://leetcode.com/problems/word-search-ii/discuss/59784/My-simple-and-clean-Java-code-using-DFS-and-Trie)
    ///
    /// ### Submissions
    ///
    /// date=20220409, mem=2.2, mem_beats=100, runtime=656, runtime_beats=10
    ///
    /// date=20220511, mem=2.1, mem_beats=100, runtime=924, runtime_beats=20
    ///
    /// date=20220528, mem=2.1, mem_beats=88, runtime=964, runtime_beats=11
    pub struct Solution;

    impl Solution {
        pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            use std::collections::HashSet;
            const MARK: char = '*';

            fn backtrack(
                board: &mut [Vec<char>],
                i: usize,
                j: usize,
                words: &mut HashSet<String>,
                cur_word: &mut String,
                res: &mut Vec<String>,
            ) {
                if cur_word.len() > 10 || board[i][j] == MARK {
                    return;
                }

                cur_word.push(board[i][j]);
                if words.remove(cur_word) {
                    res.push(cur_word.clone());
                }

                board[i][j] = MARK;

                if i < board.len() - 1 {
                    backtrack(board, i + 1, j, words, cur_word, res);
                }
                if j < board[0].len() - 1 {
                    backtrack(board, i, j + 1, words, cur_word, res);
                }
                if i > 0 {
                    backtrack(board, i - 1, j, words, cur_word, res);
                }
                if j > 0 {
                    backtrack(board, i, j - 1, words, cur_word, res);
                }

                board[i][j] = cur_word.pop().unwrap()
            }

            let (m, n) = (board.len(), board[0].len());
            let mut words = words.into_iter().collect::<HashSet<_>>();
            let mut cur_word = String::default();

            let mut res = vec![];
            for i in 0..m {
                for j in 0..n {
                    backtrack(&mut board, i, j, &mut words, &mut cur_word, &mut res);
                }
            }

            res
        }
    }
}

pub mod solution_trie {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [Java 15ms Easiest Solution (100.00%)](https://leetcode.com/problems/word-search-ii/discuss/59780/Java-15ms-Easiest-Solution-(100.00))
    /// * [单词搜索 II](https://leetcode-cn.com/problems/word-search-ii/solution/dan-ci-sou-suo-ii-by-leetcode-solution-7494/)
    ///
    /// ### Submissions
    ///
    /// date=20220412, mem=3.7, mem_beats=66, runtime=92, runtime_beats=83
    ///
    /// date=20220413, mem=3.7, mem_beats=66, runtime=96, runtime_beats=83
    ///
    /// date=20220511, mem=3.9, mem_beats=20, runtime=152, runtime_beats=40
    ///
    /// date=20220528, mem=3.7, mem_beats=33, runtime=124, runtime_beats=55
    pub struct Solution;

    impl Solution {
        pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            use std::{cell::RefCell, rc::Rc};

            #[derive(Clone, Default)]
            struct Node {
                word: Option<String>,
                children: [Option<Rc<RefCell<Node>>>; 26],
            }

            #[inline(always)]
            fn index(c: char) -> usize {
                c as usize - 'a' as usize
            }

            fn build_trie(words: Vec<String>) -> Rc<RefCell<Node>> {
                let root = Rc::new(RefCell::new(Node::default()));
                for word in words {
                    let mut node = root.clone();
                    for c in word.chars() {
                        let child = node.borrow_mut().children[index(c)]
                            .get_or_insert_with(Default::default)
                            .clone();
                        node = child;
                    }
                    node.borrow_mut().word.replace(word);
                }
                root
            }

            const MARK: char = '*';
            fn backtrack(
                board: &mut [Vec<char>],
                i: usize,
                j: usize,
                root: Rc<RefCell<Node>>,
                res: &mut Vec<String>,
            ) {
                let cur = board[i][j];
                if cur == MARK {
                    return;
                }

                let root = if let Some(child) = root.borrow().children[index(cur)].clone() {
                    child
                } else {
                    return;
                };

                if let Some(word) = root.borrow_mut().word.take() {
                    res.push(word);
                }
                board[i][j] = MARK;

                if i > 0 {
                    backtrack(board, i - 1, j, root.clone(), res);
                }
                if j > 0 {
                    backtrack(board, i, j - 1, root.clone(), res);
                }
                if i < board.len() - 1 {
                    backtrack(board, i + 1, j, root.clone(), res);
                }
                if j < board[0].len() - 1 {
                    backtrack(board, i, j + 1, root, res);
                }

                board[i][j] = cur;
            }

            let mut res = vec![];
            let root = build_trie(words);
            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    backtrack(&mut board, i, j, root.clone(), &mut res);
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
        test(solution_backtracking::Solution::find_words);
        test(solution_trie::Solution::find_words);
    }

    fn test<F: Fn(Vec<Vec<char>>, Vec<String>) -> Vec<String>>(f: F) {
        use std::collections::HashSet;
        fn two_arr<const M: usize, const N: usize>(a: [[&str; N]; M]) -> Vec<Vec<char>> {
            a.into_iter()
                .map(|a| {
                    a.into_iter()
                        .map(|s| s.chars().next().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect()
        }
        fn one_arr<const N: usize>(a: [&str; N]) -> Vec<String> {
            a.into_iter().map(ToOwned::to_owned).collect()
        }

        let res = f(
            two_arr([
                ["o", "a", "a", "n"],
                ["e", "t", "a", "e"],
                ["i", "h", "k", "r"],
                ["i", "f", "l", "v"],
            ]),
            one_arr(["oath", "pea", "eat", "rain"]),
        );
        let expected = one_arr(["eat", "oath"]);
        assert_eq!(res.len(), expected.len(),);
        assert_eq!(
            res.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect()
        );

        let res = f(two_arr([["a", "b"], ["c", "d"]]), one_arr(["abcb"]));
        let expected = vec![];
        assert_eq!(res.len(), expected.len());
        assert_eq!(
            res.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect()
        );

        let res = f(two_arr([["a", "a"]]), one_arr(["aaa"]));
        let expected = vec![];
        assert_eq!(res.len(), expected.len());
        assert_eq!(
            res.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect()
        );
    }
}
