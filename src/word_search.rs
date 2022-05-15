pub mod solution_backtracking {
    use embed_doc_image::embed_doc_image;

    /// # 思路
    ///
    /// 对任意字符 有四个可选点：当前点的上、下、左、右。
    ///
    /// ![][pic]
    ///
    /// 当前可选点不能继续递归的情况：
    ///
    /// * 越出矩阵边界。
    /// * 之前访问过，不满足「同一个单元格内的字母不允许被重复使用」。
    /// * 不是目标点，比如你想找 E，却来到了 D。
    ///
    /// ![][end]
    ///
    /// 注意：
    ///
    /// 记录访问过的点可以直接使用used[row][col]判断。但这里使用`mark=*`在回溯中记录，完成回溯
    /// 修改回去不需要数组
    ///
    /// 参考：
    ///
    /// * [Accepted very short Java solution. No additional space.](https://leetcode.com/problems/word-search/discuss/27658/Accepted-very-short-Java-solution.-No-additional-space.)
    /// * [「手画图解」回溯思路的形成与细节 | 79. 单词搜索](https://leetcode-cn.com/problems/word-search/solution/shou-hua-tu-jie-79-dan-ci-sou-suo-dfs-si-lu-de-cha/)
    ///
    /// ### Submissions
    ///
    /// date=20220330, mem=2, mem_beats=77, runtime=52, runtime_beats=93
    ///
    /// date=20220406, mem=2, mem_beats=86, runtime=52, runtime_beats=93
    ///
    /// date=20220514, mem=2.1, mem_beats=34, runtime=60, runtime_beats=66
    #[embed_doc_image("pic", "docs/images/2022-03-30-20-48-58.png")]
    #[embed_doc_image("end", "docs/images/2022-03-30-20-55-18.png")]
    pub struct Solution;

    impl Solution {
        pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
            const MARK: char = '*';

            fn find(board: &mut [Vec<char>], i: usize, j: usize, word: &[u8]) -> bool {
                let cur = word[0] as char;
                if cur == MARK || cur != board[i][j] {
                    return false;
                }
                // fixed issuse: board: [["a"]], word: "a"
                else if word.len() <= 1 {
                    return true;
                }

                let word = &word[1..];
                board[i][j] = MARK;
                let found = (i > 0 && find(board, i - 1, j, word))
                    || (i < board.len() - 1 && find(board, i + 1, j, word))
                    || (j > 0 && find(board, i, j - 1, word))
                    || (j < board[0].len() - 1 && find(board, i, j + 1, word));
                board[i][j] = cur;

                found
            }

            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    if find(&mut board, i, j, word.as_bytes()) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_backtracking::Solution::exist);
    }

    fn test<F: Fn(Vec<Vec<char>>, String) -> bool>(f: F) {
        assert!(f(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ));

        assert!(!f(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ))
    }
}
