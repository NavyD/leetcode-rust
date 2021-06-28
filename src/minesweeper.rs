//! 如何找相邻雷的数量：8个方向，只要在对应下标+-1就行了

pub mod solution_dfs {
    /// # 思路
    ///
    /// - 如果当前节点为雷，修改为x结束
    /// - 如果当前不为雷，递归找相邻节点雷数量
    ///   - 如果雷不存在，则修改为B并找相邻节点，
    ///   - 雷存在则修改节点数量并返回
    ///
    /// 嵌套循环下标变成一个iterator
    ///
    /// ```ignore
    /// OFFSET_RANGE.flat_map(|i| OFFSET_RANGE.map(move |j| (i, j))).for_each(|(i, j)| panic!())
    /// ```
    ///
    /// 注意：
    ///
    /// checked_indexes中存在一个坑`filter(|(i, j)| !(*i == 0 && *j == 0))`中，很容易写成
    /// `filter(|(i, j)| *i != 0 && *j != 0))`这导致出现0的
    /// 元素都被过滤，调试了许久才发现的。。。
    ///
    /// 在dfs中checked_indexes被放在了_dfs函数中递归也可能不断的创建，可能改成方法参数传递好点，
    /// 但是这个不太熟悉
    ///
    /// ### Submissions
    ///
    /// date=20210110, mem=2.5, mem_beats=100, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137457156/
    ///
    /// date=20210111, mem=2.7, mem_beats=100, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137614072/
    ///
    /// date=20210120, mem=2.4, mem_beats=60, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139747473/
    pub struct Solution;

    impl Solution {
        pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
            const REVEALED_MINE: char = 'X';
            const UNREVEALED_MINE: char = 'M';
            const REVEALED_BLANK: char = 'B';
            const UNREVEALED: char = 'E';

            fn _dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
                let (row_len, col_len) = (board.len(), board[0].len());
                let checked_indexes = |row: usize, col: usize| {
                    (-1..2)
                        .flat_map(|i| (-1..2).map(move |j| (i, j)))
                        // 过滤(0, 0)
                        .filter(|(i, j)| !(*i == 0 && *j == 0))
                        // offset转换board下标
                        .map(move |(i, j)| (i + row as i32, j + col as i32))
                        // 过滤非法下标
                        .filter(move |(row, col)| {
                            *row >= 0 && *row < row_len as i32 && *col >= 0 && *col < col_len as i32
                        })
                        .map(|(row, col)| (row as usize, col as usize))
                };

                let mine_count = checked_indexes(row, col)
                    .filter(|(row, col)| board[*row][*col] == UNREVEALED_MINE)
                    .count() as u8;
                if mine_count > 0 {
                    board[row][col] = (b'0' + mine_count) as char;
                    return;
                }
                board[row][col] = REVEALED_BLANK;
                for (row, col) in checked_indexes(row, col) {
                    if board[row][col] == UNREVEALED {
                        _dfs(board, row, col);
                    }
                }
            }

            let (row, col) = (click[0] as usize, click[1] as usize);
            if board[row][col] == UNREVEALED_MINE {
                board[row][col] = REVEALED_MINE;
            } else {
                _dfs(&mut board, row, col);
            }
            board
        }
    }
}

/// 要注意去重的问题
pub mod solution_bfs {
    /// # 思路
    ///
    /// BFS 借助队列，当前层的节点带出下一层节点入列，一层层地遍历。
    ///
    /// - 如果当前位置相邻有雷M则修改为对应数量，并访问下个queue位置元素
    /// - 如果访问到 E，且它周围没有雷，则标记为 B，让邻居点入列。
    ///
    /// 怎么避免节点的重复入列：由于queue在pop当前位置时可能访问到上个已访问的位置，
    /// 导致重复入队，虽然会对位置修改非 E 不会出现无限循环，但出现了超时问题。有下面
    /// 两种方法解决
    ///
    /// - 使用visited数组，对应位置在入队时置为true
    ///
    /// ```ignore
    /// checked_indexes(row, col)
    /// .filter(|(row, col)| board[*row][*col] == 'E')
    /// // 不能将visited放入filter中 出现借用问题：immutable borrow later used by call
    /// .for_each(|(row, col)| {
    ///     if !visited[row][col] {
    ///         visited[row][col] = true;
    ///     }
    ///     queue.push_back((row, col));
    /// });
    /// ```
    ///
    /// - 入列的 E 改掉它的标记。并在入列时，卡掉非 E 的点，是 E 才能入列，就能避免重复遍历。
    ///
    /// 参考：
    ///
    /// - [DFS 和 BFS 两种解法](https://leetcode-cn.com/problems/minesweeper/solution/dfs-he-bfs-jie-fa-bu-xu-yao-e-wai-kai-pi-kong-jian/)
    /// - [从起点开始 DFS / BFS 遍历一遍即可](https://leetcode-cn.com/problems/minesweeper/solution/cong-qi-dian-kai-shi-dfs-bfs-bian-li-yi-bian-ji-ke/)
    /// - [Java Solution, DFS + BFS](https://leetcode.com/problems/minesweeper/discuss/99826/Java-Solution-DFS-%2B-BFS)
    ///
    /// ### Submissions
    ///
    /// date=20210111, mem=2.2, mem_beats=100, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137607120/
    ///
    /// date=20210120, mem=2.3, mem_beats=80, runtime=20, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139752875/
    pub struct Solution;

    impl Solution {
        pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
            // 非B就行
            const MARKED: char = '#';
            const REVEALED_MINE: char = 'X';
            const UNREVEALED_MINE: char = 'M';
            const REVEALED_BLANK: char = 'B';
            const UNREVEALED: char = 'E';

            let (row, col) = (click[0] as usize, click[1] as usize);
            if board[row][col] == UNREVEALED_MINE {
                board[row][col] = REVEALED_MINE;
                return board;
            }

            let (row_len, col_len) = (board.len(), board[0].len());
            let checked_indexes = |row: usize, col: usize| {
                (-1..2)
                    .flat_map(|i| (-1..2).map(move |j| (i, j)))
                    // 过滤(0, 0)
                    .filter(|(i, j)| !(*i == 0 && *j == 0))
                    // offset转换board下标
                    .map(move |(i, j)| (i + row as i32, j + col as i32))
                    // 过滤非法下标
                    .filter(move |(row, col)| {
                        *row >= 0 && *row < row_len as i32 && *col >= 0 && *col < col_len as i32
                    })
                    .map(|(row, col)| (row as usize, col as usize))
            };

            let mut queue = std::collections::VecDeque::new();
            queue.push_back((row, col));
            while let Some((row, col)) = queue.pop_front() {
                // 相邻雷的数量
                let mine_count = checked_indexes(row, col)
                    .filter(|(row, col)| board[*row][*col] == UNREVEALED_MINE)
                    .count() as u8;
                // 修改为雷数
                if mine_count > 0 {
                    board[row][col] = (b'0' + mine_count) as char;
                } else {
                    // 置为空白
                    board[row][col] = REVEALED_BLANK;
                    for (row, col) in checked_indexes(row, col) {
                        if board[row][col] == UNREVEALED {
                            // 不重复入队标记 可在出队时重置
                            board[row][col] = MARKED;
                            queue.push_back((row, col));
                        }
                    }
                }
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
        test(solution_bfs::Solution::update_board);
    }

    fn test<F: Fn(Vec<Vec<char>>, Vec<i32>) -> Vec<Vec<char>>>(func: F) {
        let board: Vec<Vec<char>> = [
            ['E', 'E', 'E', 'E', 'E'],
            ['E', 'E', 'M', 'E', 'E'],
            ['E', 'E', 'E', 'E', 'E'],
            ['E', 'E', 'E', 'E', 'E'],
        ]
        .iter()
        .map(|a| a.iter().copied().collect())
        .collect();

        let expected: Vec<Vec<char>> = [
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'M', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B'],
        ]
        .iter()
        .map(|a| a.iter().copied().collect())
        .collect();
        let res = func(board, vec![3, 0]);
        assert_eq!(res.len(), expected.len());
        assert!(is_contains_vec2(&res, &expected));
    }
}
