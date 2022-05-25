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

pub mod solution_union {
    /// # 思路
    ///
    /// 在边缘的O与dummy节点连通分为一组，所有与dummy连通的O都不会被X填充
    ///
    /// 注意：union只需要上下左右取2个方向就可以了，同时要保证边缘节点与dummy连接，否则
    /// 会导致无法连接到边缘节点的相邻O节点
    ///
    /// ```no_run
    /// if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
    ///     uf.union(p, dummy);
    /// // 无法连接边缘的相邻O节点
    /// } else {
    ///     if i > 0 && board[i - 1][j] == O {
    ///         uf.union(p, uf.index(i - 1, j));
    ///     }
    ///     if j > 0 && board[i][j - 1] == O {
    ///         uf.union(p, uf.index(i, j - 1));
    ///     }
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// * [bfs + 递归 dfs + 非递归 dfs + 并查集](https://leetcode.cn/problems/surrounded-regions/solution/bfsdi-gui-dfsfei-di-gui-dfsbing-cha-ji-by-ac_pipe/)
    ///
    /// ### Submissions
    ///
    /// date=20220525, mem=4.7, mem_beats=68, runtime=8, runtime_beats=78
    pub struct Solution;

    impl Solution {
        pub fn solve(board: &mut Vec<Vec<char>>) {
            const O: char = 'O';
            const X: char = 'X';

            struct UnionFind {
                parent: Vec<usize>,
                rank: Vec<u32>,
                cols: usize,
            }

            impl UnionFind {
                fn new(rows: usize, cols: usize) -> Self {
                    Self {
                        parent: (0..rows * cols + 1).collect::<Vec<_>>(),
                        cols,
                        rank: vec![0; rows * cols + 1],
                    }
                }
                #[inline(always)]
                fn index(&self, row: usize, col: usize) -> usize {
                    self.cols * row + col
                }
                fn find(&mut self, mut p: usize) -> usize {
                    while self.parent[p] != p {
                        self.parent[p] = self.parent[self.parent[p]];
                        p = self.parent[p];
                    }
                    p
                }
                fn union(&mut self, p: usize, q: usize) {
                    let (mut x, mut y) = (self.find(p), self.find(q));
                    if x == y {
                        return;
                    }

                    use std::cmp::Ordering::*;
                    match self.rank[x].cmp(&self.rank[y]) {
                        Equal => self.rank[y] += 1,
                        Greater => std::mem::swap(&mut x, &mut y),
                        _ => {}
                    }
                    self.parent[x] = y;
                }
                fn has_unit(&mut self, p: usize, q: usize) -> bool {
                    self.find(p) == self.find(q)
                }
            }

            let (rows, cols) = (board.len(), board[0].len());
            let dummy = rows * cols;
            let mut uf = UnionFind::new(rows, cols);

            for i in 0..rows {
                for j in 0..cols {
                    if board[i][j] == O {
                        let p = uf.index(i, j);
                        if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
                            uf.union(p, dummy);
                        } else {
                            if i > 0 && board[i - 1][j] == O {
                                uf.union(p, uf.index(i - 1, j));
                            }
                            if j > 0 && board[i][j - 1] == O {
                                uf.union(p, uf.index(i, j - 1));
                            }
                        }
                    }
                }
            }

            for i in 0..rows {
                for j in 0..cols {
                    if board[i][j] == O && !uf.has_unit(uf.index(i, j), dummy) {
                        board[i][j] = X;
                    }
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
        test(solution_union::Solution::solve);
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

        let mut input = arr([
            ["O", "X", "X", "O", "X"],
            ["X", "O", "O", "X", "O"],
            ["X", "O", "X", "O", "X"],
            ["O", "X", "O", "O", "O"],
            ["X", "X", "O", "X", "O"],
        ]);
        f(&mut input);
        assert_eq!(
            input,
            arr([
                ["O", "X", "X", "O", "X"],
                ["X", "X", "X", "X", "O"],
                ["X", "X", "X", "O", "X"],
                ["O", "X", "O", "O", "O"],
                ["X", "X", "O", "X", "O"]
            ])
        );
    }
}
