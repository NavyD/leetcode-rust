//! 如何处理皇后不能被攻击问题

/// 总结
///
/// 回溯还是简单，但是如何处理问题才是关键。如何将不能被攻击转换为
/// 对应数组下标关系还是不行
pub mod solution_backtracking {
    /// # 思路
    ///
    /// 对每一个棋子不断的试错，找出所有的解法。对每一行放一个棋子在不同位置，递归下去判断是否在
    /// 列、45对角线上、135对角线上能被攻击。
    ///
    /// 回溯递归树：
    ///
    /// ![](https://pic.leetcode-cn.com/1598117469-RXhjxi-image.png)
    ///
    /// 如何判断在当前位置时可能被攻击：使用row递归下去，每一行只有一个棋子，行不可能被攻击；
    ///
    /// 对于维护一个cols, left right diagonals数组，表示列、左右对角线是否被攻击；
    ///
    /// 对于列，长度为n只要对应位置已棋子，left_diagonals[col]=true就可以判断，不管多少行，一列只有一个棋子；
    ///
    /// 对于对角线，一个n大小的棋盘有2*n - 1个对角线（左右都是）。假设有一个n=2的棋盘：如何判断在一条对角线上
    /// left45(row,col)：  [(1,0), (0,1)], [(0,0)], [(1,1)]
    /// right135(row,col): [(0,0), (1,1)], [(1,0)], [(0, 1)]
    ///
    /// 在left时：left_diagonals[row + col]=true表示在一条对角线上
    /// 在right时：right_diagonals[n - 1 + col - row]=true表示在一条角线上
    ///
    /// 一个n=4的对角线：
    ///
    /// ![](https://pic.leetcode-cn.com/1599142979-VEuEDb-image.png)
    ///
    /// 参考：
    ///
    /// - [回溯算法（转换成全排列问题 + 剪枝）- 题解后有相关问题](https://leetcode-cn.com/problems/n-queens/solution/gen-ju-di-46-ti-quan-pai-lie-de-hui-su-suan-fa-si-/)
    ///
    /// ### Submissions
    ///
    /// date=20201219, mem=2.2, mem_beats=63, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132179132/
    ///
    /// date=20201220, mem=2.3, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132332061/
    ///
    /// date=20201228, mem=2.2, mem_beats=92, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/134422730/
    ///
    /// date=20220613, mem=2.4, mem_beats=25, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
            const QUEEN: char = 'Q';
            const EMPTY: char = '.';

            fn backtrack(
                n: usize,
                row: usize,
                cols: &mut Vec<bool>,
                front_diagonals: &mut Vec<bool>,
                back_diagonals: &mut Vec<bool>,
                path: &mut Vec<Vec<char>>,
                res: &mut Vec<Vec<String>>,
            ) {
                if row == n {
                    let solution = path
                        .iter()
                        .map(|e| e.iter().collect())
                        .collect::<Vec<String>>();
                    res.push(solution);
                    return;
                }

                for col in 0..n {
                    // 对角线对应下标
                    let (front, back) = (col + row, n - 1 + row - col);
                    // 不能攻击
                    if !cols[col] && !front_diagonals[front] && !back_diagonals[back] {
                        cols[col] = true;
                        front_diagonals[front] = true;
                        back_diagonals[back] = true;
                        path[row][col] = QUEEN;

                        backtrack(n, row + 1, cols, front_diagonals, back_diagonals, path, res);

                        cols[col] = false;
                        front_diagonals[front] = false;
                        back_diagonals[back] = false;
                        path[row][col] = EMPTY;
                    }
                }
            }

            let n = n as usize;
            let mut res = vec![];

            backtrack(
                n,
                0,
                // 默认false表示对应位置没有占用 不能攻击到
                &mut vec![false; n],
                // 对角线个数为2*n - 1
                &mut vec![false; 2 * n - 1],
                &mut vec![false; 2 * n - 1],
                &mut vec![vec![EMPTY; n]; n],
                &mut res,
            );

            res
        }
    }
}

pub mod solution_backtracking_each_position {
    /// # 思路
    ///
    /// 对每个位置上迭代检查每个列、对角线
    ///
    /// 参考：
    ///
    /// [Accepted 4ms c++ solution use backtracking and bitmask, easy understand.](https://leetcode.com/problems/n-queens/discuss/19808/Accepted-4ms-c%2B%2B-solution-use-backtracking-and-bitmask-easy-understand.)
    ///
    /// ### Submissions
    ///
    /// date=20201219, mem=2.4, mem_beats=9, runtime=4, runtime_beats=69, url=https://leetcode-cn.com/submissions/detail/132198620/
    pub struct Solution;

    impl Solution {
        pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
            const QUEEN: char = 'Q';
            const EMPTY: char = '.';
            fn _backtrack(
                n: usize,
                row: usize,
                path: &mut Vec<Vec<char>>,
                res: &mut Vec<Vec<String>>,
            ) {
                if row >= n {
                    res.push(
                        path.clone()
                            .into_iter()
                            .map(|e| e.into_iter().collect())
                            .collect(),
                    );
                    return;
                }
                for col in 0..n {
                    if _is_valid(path, row, col, n) {
                        path[row][col] = QUEEN;
                        _backtrack(n, row + 1, path, res);
                        path[row][col] = EMPTY;
                    }
                }
            }

            fn _is_valid(path: &[Vec<char>], row: usize, col: usize, n: usize) -> bool {
                // check cols
                for i in 0..n {
                    if path[i][col] == QUEEN {
                        return false;
                    }
                }
                let (mut i, mut j) = (row, col);
                // check 45 diagonal
                while i > 0 && j > 0 {
                    i -= 1;
                    j -= 1;
                    if path[i][j] == QUEEN {
                        return false;
                    }
                }
                let (mut i, mut j) = (row, col);
                // check 135 diagonal
                let n = n - 1;
                while i > 0 && j < n {
                    i -= 1;
                    j += 1;
                    if path[i][j] == QUEEN {
                        return false;
                    }
                }
                true
            }
            let mut res = vec![];
            let n = n as usize;
            _backtrack(n, 0, &mut vec![vec![EMPTY; n]; n], &mut res);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::i32;

    #[test]
    fn basic() {
        test(solution_backtracking::Solution::solve_n_queens);
        test(solution_backtracking_each_position::Solution::solve_n_queens);
        test(solve_n_queens)
    }

    fn test<F: Fn(i32) -> Vec<Vec<String>>>(func: F) {
        fn arr<const N: usize>(a: &[[&str; N]]) -> Vec<Vec<String>> {
            a.iter()
                .map(|a| a.iter().map(ToString::to_string).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        }

        assert_eq!(func(1), arr(&[["Q"]]));
        assert_eq!(
            func(4),
            arr(&[
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."],
            ])
        );
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        const QUEEN: char = 'Q';
        const EMPTY: char = '.';

        fn backtrack(
            n: usize,
            row: usize,
            cols: &mut [bool],
            forward_diagonals: &mut [bool],
            back_diagonals: &mut [bool],
            path: &mut Vec<Vec<char>>,
            res: &mut Vec<Vec<String>>,
        ) {
            if n == row {
                let s = path
                    .clone()
                    .into_iter()
                    .map(|s| s.into_iter().collect::<String>())
                    .collect::<Vec<_>>();
                res.push(s);
                return;
            }

            for col in 0..n {
                let (left, right) = (row + col, n - 1 + col - row);
                if !cols[col] && !forward_diagonals[left] && !back_diagonals[right] {
                    cols[col] = true;
                    forward_diagonals[left] = true;
                    back_diagonals[right] = true;
                    path[row][col] = QUEEN;

                    backtrack(
                        n,
                        row + 1,
                        cols,
                        forward_diagonals,
                        back_diagonals,
                        path,
                        res,
                    );

                    path[row][col] = EMPTY;
                    cols[col] = false;
                    forward_diagonals[left] = false;
                    back_diagonals[right] = false;
                }
            }
        }

        let n = n as usize;
        let mut res = vec![];

        backtrack(
            n,
            0,
            &mut vec![false; n],
            &mut vec![false; 2 * n - 1],
            &mut vec![false; 2 * n - 1],
            &mut vec![vec![EMPTY; n]; n],
            &mut res,
        );

        res
    }
}
