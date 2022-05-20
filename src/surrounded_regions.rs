pub mod solution_dfs {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [被围绕的区域 方法一：深度优先搜索](https://leetcode.cn/problems/surrounded-regions/solution/bei-wei-rao-de-qu-yu-by-leetcode-solution/)
    /// * [bfs + 递归 dfs + 非递归 dfs + 并查集](https://leetcode.cn/problems/surrounded-regions/solution/bfsdi-gui-dfsfei-di-gui-dfsbing-cha-ji-by-ac_pipe/)
    ///
    /// ### Submissions
    ///
    /// date=20220520, mem=4.7, mem_beats=64, runtime=8, runtime_beats=76
    pub struct Solution;

    impl Solution {
        pub fn solve(board: &mut Vec<Vec<char>>) {
            const O: char = 'O';
            const X: char = 'X';
            const MARKED: char = '*';

            fn dfs(board: &mut [Vec<char>], i: usize, j: usize) {
                if i >= board.len() || j >= board[0].len() || board[i][j] != O {
                    return;
                }
                board[i][j] = MARKED;

                if i > 0 {
                    dfs(board, i - 1, j);
                }
                if j > 0 {
                    dfs(board, i, j - 1);
                }
                dfs(board, i + 1, j);
                dfs(board, i, j + 1);
            }

            // let board = &board;
            let (m, n) = (board.len(), board[0].len());
            for i in 0..m {
                dfs(board, i, 0);
                dfs(board, i, n - 1);
            }

            for i in 1..n {
                dfs(board, 0, i);
                dfs(board, m - 1, i);
            }

            for i in 0..m {
                for j in 0..n {
                    board[i][j] = match board[i][j] {
                        O => X,
                        MARKED => O,
                        _ => continue,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::solve);
    }

    fn test<F: Fn(&mut Vec<Vec<char>>)>(f: F) {
        fn arr<const M: usize, const N: usize>(a: [[&str; N]; M]) -> Vec<Vec<char>> {
            a.into_iter()
                .map(|a| {
                    a.into_iter()
                        .map(|s| s.chars().next().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        let mut input = arr([
            ["X", "X", "X", "X"],
            ["X", "O", "O", "X"],
            ["X", "X", "O", "X"],
            ["X", "O", "X", "X"],
        ]);
        f(&mut input);
        assert_eq!(
            input,
            arr([
                ["X", "X", "X", "X"],
                ["X", "X", "X", "X"],
                ["X", "X", "X", "X"],
                ["X", "O", "X", "X"]
            ])
        );

        let mut input = arr([["X"]]);
        f(&mut input);
        assert_eq!(input, arr([["X"]]));
    }
}
