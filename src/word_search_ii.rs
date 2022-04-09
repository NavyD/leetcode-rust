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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn basics() {
        test(solution_backtracking::Solution::find_words);
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
