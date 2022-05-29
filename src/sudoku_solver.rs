pub mod solution_dfs {
    /// # 思路
    ///
    /// 注意：
    ///
    /// * 在`let (i, j) = (count / N, count % N);`通过一个参数取出了对应的两个下标，而不用直接传入(i,j)，
    /// 这样非常优雅，不用过多处理边界。如当count=8时，(i=0,j=8)，当count=9时，(i=1,j=0)到了第二行
    ///
    ///     ```rust,no_run
    ///     if col == board[0].length {
    ///         col = 0;
    ///         row += 1;
    ///         if(row == board.length){
    ///             return true;
    ///         }
    ///     }
    ///     ```
    ///
    /// * 定义`let mut used_areas = [[false; N]; N];`，而不是三维数组，取下标时使用`let k = (i / 3) * 3 + j / 3`，
    /// 这里是参考了[36. 有效的数独](https://leetcode.cn/problems/valid-sudoku/)
    ///
    /// 另外，还有一种使用bitset节省空间的方式，但算法思想是一致的，当前的解法更可读
    ///
    /// 参考：
    ///
    /// * [回溯法解数独](https://leetcode.cn/problems/sudoku-solver/solution/hui-su-fa-jie-shu-du-by-i_use_python/240847)
    /// * [【解数独】回溯 + 状态压缩（使用 bitset）](https://leetcode.cn/problems/sudoku-solver/solution/37-by-ikaruga/)
    ///
    /// ### Submissions
    ///
    /// date=20220527, mem=2.1, mem_beats=42, runtime=0, runtime_beats=100
    ///
    /// date=20220529, mem=2.1, mem_beats=66, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
            const EMPTY: char = '.';
            const N: usize = 9;

            fn validate(
                board: &mut [Vec<char>],
                count: usize,
                used_rows: &mut [[bool; N]],
                used_cols: &mut [[bool; N]],
                used_areas: &mut [[bool; N]],
            ) -> bool {
                if count >= 81 {
                    return true;
                }
                let (i, j) = (count / N, count % N);
                if board[i][j] != EMPTY {
                    return validate(board, count + 1, used_rows, used_cols, used_areas);
                }

                let k = (i / 3) * 3 + j / 3;
                for num in 0..N {
                    if !used_rows[i][num] && !used_cols[j][num] && !used_areas[k][num] {
                        board[i][j] = (num as u8 + b'1') as char;
                        used_rows[i][num] = true;
                        used_cols[j][num] = true;
                        used_areas[k][num] = true;

                        if validate(board, count + 1, used_rows, used_cols, used_areas) {
                            return true;
                        }

                        used_rows[i][num] = false;
                        used_cols[j][num] = false;
                        used_areas[k][num] = false;
                    }
                }

                board[i][j] = EMPTY;
                false
            }

            let mut used_rows = [[false; N]; N];
            let mut used_cols = [[false; N]; N];
            let mut used_areas = [[false; N]; N];

            for i in 0..N {
                for j in 0..N {
                    let c = board[i][j];
                    if c != EMPTY {
                        let num = c as usize - '1' as usize;
                        let k = (i / 3) * 3 + j / 3;

                        used_rows[i][num] = true;
                        used_cols[j][num] = true;
                        used_areas[k][num] = true;
                    }
                }
            }

            validate(board, 0, &mut used_rows, &mut used_cols, &mut used_areas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::solve_sudoku);
    }

    fn test(f: impl Fn(&mut Vec<Vec<char>>)) {
        fn arr<const N: usize>(a: [[&str; N]; N]) -> Vec<Vec<char>> {
            a.into_iter()
                .map(|a| {
                    a.into_iter()
                        .map(|s| s.chars().next().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect()
        }

        let mut input = arr([
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]);
        f(&mut input);
        assert_eq!(
            arr([
                ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
                ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
                ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
                ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
                ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
                ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
                ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
                ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
                ["3", "4", "5", "2", "8", "6", "1", "7", "9"]
            ]),
            input
        );
    }
}
