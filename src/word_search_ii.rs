pub mod solution_backtracking {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [【宫水三叶】一题双解 :「回溯」&「Trie 」](https://leetcode-cn.com/problems/word-search-ii/solution/gong-shui-san-xie-yi-ti-shuang-jie-hui-s-am8f/)
    ///
    /// ### Submissions
    ///
    /// date=20220409, mem=2.2, mem_beats=100, runtime=656, runtime_beats=10
    pub struct Solution;

    impl Solution {
        pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            const DIR: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            fn found(
                board: &[Vec<char>],
                i: usize,
                j: usize,
                words: &mut std::collections::HashSet<String>,
                cur_word: &mut String,
                vis: &mut [Vec<bool>],
                res: &mut Vec<String>,
            ) {
                if cur_word.len() > 10 {
                    return;
                }
                if words.remove(cur_word) {
                    res.push(cur_word.to_string());
                }

                for (x, y) in DIR {
                    let (i, j) = (i as isize + x, j as isize + y);
                    if i >= 0 && j >= 0 {
                        let (i, j) = (i as usize, j as usize);
                        if i < board.len() && j < board[0].len() && !vis[i][j] {
                            vis[i][j] = true;
                            cur_word.push(board[i][j]);
                            found(board, i, j, words, cur_word, vis, res);
                            vis[i][j] = false;
                            cur_word.pop();
                        }
                    }
                }
            }

            let mut words = words.into_iter().collect::<std::collections::HashSet<_>>();
            let (m, n) = (board.len(), board[0].len());
            let mut vis = vec![vec![false; n]; m];
            let mut res = vec![];
            let mut cur_word = String::new();
            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    cur_word.push(board[i][j]);
                    vis[i][j] = true;
                    found(&board, i, j, &mut words, &mut cur_word, &mut vis, &mut res);
                    vis[i][j] = false;
                    cur_word.pop();
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
    ///
    /// ### Submissions
    ///
    /// date=20220412, mem=3.7, mem_beats=66, runtime=92, runtime_beats=83
    ///
    /// date=20220413, mem=3.7, mem_beats=66, runtime=96, runtime_beats=83
    pub struct Solution;

    impl Solution {
        pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
            // RC and refcell can be used
            #[derive(Debug, Clone)]
            struct Node {
                word: Option<String>,
                children: Vec<Option<Box<Node>>>,
            }

            impl Default for Node {
                fn default() -> Self {
                    Self {
                        word: None,
                        children: vec![None::<Box<Node>>; 26],
                    }
                }
            }

            fn build_trie(words: Vec<String>) -> Node {
                let mut root: Node = Default::default();
                for word in words {
                    let mut next = &mut root;
                    for cur in word.chars() {
                        next = next.children[index(cur)]
                            .get_or_insert_with(Default::default)
                            .as_mut();
                    }
                    next.word = Some(word);
                }
                root
            }

            const MARK: char = '#';

            #[inline(always)]
            fn index(i: char) -> usize {
                i as usize - 'a' as usize
            }

            fn backtrack(
                board: &mut [Vec<char>],
                i: usize,
                j: usize,
                mut root: &mut Node,
                res: &mut Vec<String>,
            ) {
                let cur = board[i][j];
                if cur == MARK || root.children[index(cur)].is_none() {
                    return;
                }

                root = root.children[index(cur)].as_deref_mut().unwrap();
                if let Some(word) = root.word.take() {
                    res.push(word);
                }
                board[i][j] = MARK;

                if i < board.len() - 1 {
                    backtrack(board, i + 1, j, root, res);
                }
                if j < board[0].len() - 1 {
                    backtrack(board, i, j + 1, root, res);
                }
                if i > 0 {
                    backtrack(board, i - 1, j, root, res);
                }
                if j > 0 {
                    backtrack(board, i, j - 1, root, res);
                }

                board[i][j] = cur;
            }

            let mut root = build_trie(words);
            let mut res = vec![];

            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    backtrack(&mut board, i, j, &mut root, &mut res);
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn basics() {
        test(solution_backtracking::Solution::find_words);
        test(solution_trie::Solution::find_words);
    }

    fn test<F: Fn(Vec<Vec<char>>, Vec<String>) -> Vec<String>>(f: F) {
        let res = f(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                "oath".to_string(),
                "pea".to_string(),
                "eat".to_string(),
                "rain".to_string(),
            ],
        );
        let expected = vec!["eat".to_string(), "oath".to_string()];
        assert_eq!(res.len(), expected.len(),);
        assert_eq!(
            res.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect()
        );

        let res = f(
            vec![vec!['a', 'b'], vec!['c', 'd']],
            vec!["abcb".to_string()],
        );
        let expected = vec![];
        assert_eq!(res.len(), expected.len(),);
        assert_eq!(
            res.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect()
        );
    }
}
