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
    ///
    /// date=20220628, mem=2.4, mem_beats=43, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
            const EMPTY: u8 = b'.';
            const QUEEN: u8 = b'Q';

            fn backtrack(
                n: usize,
                row: usize,
                cols: &mut [bool],
                front_diagonals: &mut [bool],
                back_diagonals: &mut [bool],
                path: &mut [Vec<u8>],
                solutions: &mut Vec<Vec<String>>,
            ) {
                if row >= n {
                    let solution = path
                        .iter()
                        .map(|a| unsafe { String::from_utf8_unchecked(a.to_vec()) })
                        .collect::<Vec<_>>();
                    solutions.push(solution);
                    return;
                }

                for col in 0..n {
                    // 对角线对应下标
                    let (front, back) = (n - 1 + row - col, row + col);
                    // 不能攻击
                    if !cols[col] && !front_diagonals[front] && !back_diagonals[back] {
                        cols[col] = true;
                        front_diagonals[front] = true;
                        back_diagonals[back] = true;
                        path[row][col] = QUEEN;

                        backtrack(
                            n,
                            row + 1,
                            cols,
                            front_diagonals,
                            back_diagonals,
                            path,
                            solutions,
                        );

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

pub mod solution_bit {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [N 皇后 方法二：基于位运算的回溯](https://leetcode.cn/problems/n-queens/solution/nhuang-hou-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20220614, mem=2.2, mem_beats=75, runtime=0, runtime_beats=100
    ///
    /// date=20220615, mem=2.5, mem_beats=6, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
            const QUEEN: char = 'Q';
            const EMPTY: char = '.';
            const INIT: i16 = -1;

            fn backtrack(
                n: usize,
                row: usize,
                // 表示在每行中queen放的列位置，用于定位queen在board的位置
                queen_cols: &mut [i16],
                // 当前列是否存在queen
                cols: i16,
                front_diagonals: i16,
                back_diagonals: i16,
                solutions: &mut Vec<Vec<String>>,
            ) {
                if n == row {
                    let solution = (0..n)
                        .map(|row| {
                            let mut board_row = vec![EMPTY; n];
                            board_row[queen_cols[row] as usize] = QUEEN;
                            board_row.into_iter().collect()
                        })
                        .collect::<Vec<String>>();

                    solutions.push(solution);
                    return;
                }

                // 可以放置皇后的位置（该结果的值为 1 的位置表示可以放置皇后的位置）
                let mut available_positions =
                    ((1 << n) - 1) & (!(cols | front_diagonals | back_diagonals));
                while available_positions != 0 {
                    // 获取二进制表示中的最低位的 1 的位置；
                    let position = available_positions & -available_positions;
                    // 移除最低位的 1
                    available_positions &= available_positions - 1;
                    // 放在row中的queen的列位置。postion=8,0b1000 => 0b0111 => 3
                    queen_cols[row] = (position - 1).count_ones() as i16;
                    backtrack(
                        n,
                        row + 1,
                        queen_cols,
                        cols | position,
                        // 下一行正对角线往左移一位，表示为右移一位（左列对应最低二进制位）
                        (front_diagonals | position) >> 1,
                        (back_diagonals | position) << 1,
                        solutions,
                    );
                    queen_cols[row] = INIT;
                }
            }

            let n = n as usize;
            let mut solutions = vec![];
            backtrack(n, 0, &mut vec![INIT; n], 0, 0, 0, &mut solutions);
            solutions
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
        test(solution_bit::Solution::solve_n_queens);
        // TODO
        // test(solve_n_queens)
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
        const EMPTY: u8 = b'.';
        const QUEEN: u8 = b'Q';
        const INIT: i16 = -1;

        fn backtrack(
            n: usize,
            row: usize,
            queen_cols: &mut [i16],
            cols: i16,
            front_diagoals: i16,
            back_diagonals: i16,
            solutions: &mut Vec<Vec<String>>,
        ) {
            if n >= row {
                let solution = (0..n)
                    .map(|row| {
                        let mut board_row = vec![EMPTY; n];
                        board_row[queen_cols[row] as usize] = QUEEN;
                        unsafe { String::from_utf8_unchecked(board_row) }
                    })
                    .collect::<Vec<_>>();
                solutions.push(solution);
                return;
            }

            let mut available_positions =
                ((1 << n) - 1) & (!(cols | front_diagoals | back_diagonals));
            while available_positions != 0 {
                let position = available_positions & -available_positions;
                available_positions &= available_positions - 1;
                queen_cols[row] = (position - 1).count_ones() as i16;
                backtrack(
                    n,
                    row + 1,
                    queen_cols,
                    cols | position,
                    (front_diagoals | position) >> 1,
                    (back_diagonals | position) << 1,
                    solutions,
                );
                queen_cols[row] = INIT;
            }
        }

        let n = n as usize;
        let mut res = vec![];
        backtrack(n, 0, &mut vec![INIT; n], 0, 0, 0, &mut res);
        res
    }
}
