pub mod solution {

    /// # 思路
    ///
    /// 参考：
    ///
    /// * [36. 简单想法，最优思路：就按照从左往右、从上往下的顺序遍历一次 board，完成 3 个条件的检验](https://leetcode.cn/problems/valid-sudoku/solution/36-jiu-an-zhao-cong-zuo-wang-you-cong-shang-wang-x/)
    ///
    /// ### submissions
    ///
    /// date=20220529, mem=2, mem_beats=87, runtime=4, runtime_beats=85
    pub struct Solution;

    impl Solution {
        pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            const N: usize = 9;
            const EMPTY: char = '.';

            let mut used_rows = [[false; N]; N];
            let mut used_cols = [[false; N]; N];
            let mut used_areas = [[false; N]; N];

            for i in 0..N {
                for j in 0..N {
                    if board[i][j] != EMPTY {
                        let num = board[i][j] as usize - '1' as usize;
                        let k = (i / 3) * 3 + j / 3;

                        if used_rows[i][num] || used_cols[j][num] || used_areas[k][num] {
                            return false;
                        }

                        used_rows[i][num] = true;
                        used_cols[j][num] = true;
                        used_areas[k][num] = true;
                    }
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution::Solution::is_valid_sudoku);
    }

    fn test(f: impl Fn(Vec<Vec<char>>) -> bool) {
        fn arr<const N: usize>(a: [[&str; N]; N]) -> Vec<Vec<char>> {
            a.into_iter()
                .map(|a| {
                    a.into_iter()
                        .map(|s| s.chars().next().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect()
        }

        assert!(f(arr([
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"]
        ])));
        assert!(!f(arr([
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"]
        ])));
    }
}
