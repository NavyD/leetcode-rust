pub mod solution_dfs {

    /// 如果当前节点为雷，修改为x结束
    /// 如果当前不为雷，递归找相邻节点雷数量
    ///     如果雷不存在，则修改为B并找相邻节点，
    ///     雷存在则修改节点数量并返回
    ///
    /// 嵌套循环下标变成一个iterator
    ///
    /// ```ignore
    /// OFFSET_RANGE.flat_map(|i| OFFSET_RANGE.map(move |j| (i, j))).for_each(|(i, j)| panic!())
    /// ```
    /// 
    /// ### Submissions
    /// 
    /// date=20210110, mem=2.5, mem_beats=100, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137457156/
    pub struct Solution;

    impl Solution {
        pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
            const OFFSETS: std::ops::Range<i32> = -1..2;

            fn _dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
                let mut mine_count: u8 = 0;
                for i in OFFSETS {
                    for j in OFFSETS {
                        let row = i + row as i32;
                        let col = j + col as i32;
                        if row >= 0
                            && row < board.len() as i32
                            && col >= 0
                            && col < board[0].len() as i32
                            && board[row as usize][col as usize] == 'M'
                        {
                            mine_count += 1;
                        }
                    }
                }
                if mine_count > 0 {
                    board[row][col] = ('0' as u8 + mine_count) as char;
                    return;
                }
                board[row][col] = 'B';
                for i in OFFSETS {
                    for j in OFFSETS {
                        let row = i + row as i32;
                        let col = j + col as i32;
                        if row >= 0
                            && row < board.len() as i32
                            && col >= 0
                            && col < board[0].len() as i32
                            && board[row as usize][col as usize] == 'E'
                        {
                            _dfs(board, row as usize, col as usize);
                        }
                    }
                }
            }

            let (row, col) = (click[0] as usize, click[1] as usize);
            if board[row][col] == 'M' {
                board[row][col] = 'X';
            } else {
                _dfs(&mut board, row, col);
            }
            board
        }
    }
}

// todo
pub mod solution_bfs {
    pub struct Solution;

    impl Solution {
        pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
            let (row, col) = (click[0] as usize, click[1] as usize);
            if board[row][col] == 'M' {
                board[row][col] = 'X';
                return board;
            }

            let (row_len, col_len) = (board.len(), board[0].len());
            let checked_indexes = |row: usize, col: usize| {
                (-1..2)
                    .flat_map(|i| (-1..2).map(move |j| (i, j)))
                    .map(move |(i, j)| (i + row as i32, j + col as i32))
                    .filter(move |(i, j)| {
                        let row = i + row as i32;
                        let col = j + col as i32;
                        row >= 0 && row < row_len as i32 && col >= 0 && col < col_len as i32
                    })
                    .map(|(i, j)| (i as usize, j as usize))
            };

            let mut visited = vec![vec![false; board[0].len()]; board.len()];
            visited[row][col] = true;

            let mut queue = std::collections::VecDeque::new();
            queue.push_back((row, col));

            while let Some((row, col)) = queue.pop_front() {
                let mine_count = checked_indexes(row, col)
                    .filter(|(i, j)| board[*i][*j] == 'M')
                    .count() as u8;
                if mine_count > 0 {
                    board[row][col] = ('0' as u8 + mine_count) as char;
                    continue;
                }
                board[row][col] = 'B';
                checked_indexes(row, col)
                    .for_each(|(i, j)| {
                        if board[i][j] == 'E' && !visited[i][j] {
                            visited[i][j] = true;
                            queue.push_back((i, j));
                        }
                    });
            }

            board
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn basic() {
        test(solution_dfs::Solution::update_board);
        // test(solution_bfs::Solution::update_board);
    }

    fn test<F: Fn(Vec<Vec<char>>, Vec<i32>) -> Vec<Vec<char>>>(func: F) {
        let board: Vec<Vec<char>> = [
            ['E', 'E', 'E', 'E', 'E'],
            ['E', 'E', 'M', 'E', 'E'],
            ['E', 'E', 'E', 'E', 'E'],
            ['E', 'E', 'E', 'E', 'E'],
        ]
        .iter()
        .map(|a| a.iter().map(|e| e.clone()).collect())
        .collect();

        let expected: Vec<Vec<char>> = [
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'M', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B'],
        ]
        .iter()
        .map(|a| a.iter().map(|e| e.clone()).collect())
        .collect();
        let res = func(board, vec![3, 0]);
        assert_eq!(res.len(), expected.len());
        assert!(is_contains_vec2(&res, &expected));
    }
}
