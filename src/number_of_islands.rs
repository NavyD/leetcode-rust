//! 如何计算岛屿的数量
//!
//! 如果用n皇后类似的方式dfs对row,col遍历，怎么处理grid中出现的0不同的岛屿，如何处理相邻
//! 的1是一个岛屿的问题
//!
//! 遍历所有元素，如果是一个LAND则删除（标记）相邻的LAND，使得不会重复找一个island
//!

pub mod solution_dfs {
    /// # 思路
    ///
    /// 遍历所有元素，如果是一个LAND则删除相邻的LAND，使得不会重复找一个island
    ///
    /// ## 网格问题的基本概念
    ///
    /// 岛屿问题是这类网格 DFS 问题的典型代表。二叉树的 DFS 有两个要素：「访问相邻结点」和「判断 base case」
    /// 在网格DFS中：
    ///
    /// - 访问相邻结点
    ///
    /// 上下左右四个。对于格子 (r, c) 来说（r 和 c 分别代表行坐标和列坐标），
    /// 四个相邻的格子分别是 (r-1, c)、(r+1, c)、(r, c-1)、(r, c+1)。换句话说，网格结构是「四叉」的
    ///
    /// ```ignore
    /// // 访问上、下、左、右四个相邻结点
    /// dfs(grid, r - 1, c);
    /// dfs(grid, r + 1, c);
    /// dfs(grid, r, c - 1);
    /// dfs(grid, r, c + 1);
    /// ```
    ///
    /// - 判断 base case
    ///
    /// 网格中不需要继续遍历、grid[r][c] 会出现数组下标越界异常的格子
    ///
    /// ```java, ignore
    /// // 判断坐标 (r, c) 是否在网格中
    /// boolean inArea(int[][] grid, int r, int c) {
    ///     return 0 <= r && r < grid.length
    ///         	&& 0 <= c && c < grid[0].length;
    /// }
    /// ```
    ///
    /// ## 如何避免重复遍历
    ///
    /// 标记已经遍历过的格子。以岛屿问题为例，我们需要在所有值为 1 的陆地格子上做 DFS 遍历。
    /// 每走过一个陆地格子，就把格子的值改为 2，这样当我们遇到 2 的时候，就知道这是遍历过的格子了。
    /// 
    /// 参考：
    /// 
    /// - [岛屿类问题的通用解法、DFS 遍历框架](https://leetcode-cn.com/problems/number-of-islands/solution/dao-yu-lei-wen-ti-de-tong-yong-jie-fa-dfs-bian-li-/)
    /// 
    /// ### Submissions
    /// 
    /// date=20210109, mem=5.2, mem_beats=50, runtime=8, runtime_beats=30, url=https://leetcode-cn.com/submissions/detail/137091727/
    pub struct Solution;

    impl Solution {
        pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
            const LAND: char = '1';
            const DELETED_LAND: char = '2';
            fn _dfs_delete(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
                if row >= grid.len() || col >= grid[0].len() || grid[row][col] != LAND {
                    return;
                }
                grid[row][col] = DELETED_LAND;
                _dfs_delete(grid, row + 1, col);
                _dfs_delete(grid, row, col + 1);
                if row >= 1 {
                    _dfs_delete(grid, row - 1, col);
                }
                if col >= 1 {
                    _dfs_delete(grid, row, col - 1);
                }
            }
            let mut count = 0;
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if grid[row][col] == LAND {
                        _dfs_delete(&mut grid, row, col);
                        count += 1;
                    }
                }
            }
            count
        }
    }
}

pub mod solution_bfs {

    /// # 思路
    /// 
    /// 用queue代替dfs，queue保存row,col下标去搜索对应的相邻节点并删除
    /// 
    /// 参考：
    /// 
    /// - [200. 岛屿数量（DFS / BFS）](https://leetcode-cn.com/problems/number-of-islands/solution/number-of-islands-shen-du-you-xian-bian-li-dfs-or-/)
    /// 
    /// ### Submissions
    /// 
    /// date=20210109, mem=5.1, mem_beats=100, runtime=4, runtime_beats=90, url=https://leetcode-cn.com/submissions/detail/137095630/
    pub struct Solution;

    impl Solution {
        pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
            use std::collections::VecDeque;
            const LAND: char = '1';
            const DELETED_LAND: char = '2';

            fn _bfs_delete(
                grid: &mut Vec<Vec<char>>,
                row: usize,
                col: usize,
                queue: &mut VecDeque<(usize, usize)>,
            ) {
                queue.push_back((row, col));
                while let Some((row, col)) = queue.pop_front() {
                    if row < grid.len() && col < grid[0].len() && grid[row][col] == LAND {
                        grid[row][col] = DELETED_LAND;
                        queue.push_back((row + 1, col));
                        queue.push_back((row, col + 1));
                        if row > 0 {
                            queue.push_back((row - 1, col));
                        }
                        if col > 0 {
                            queue.push_back((row, col - 1));
                        }
                    }
                }
            }
            let mut count = 0;
            let mut queue = VecDeque::new();
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if grid[row][col] == LAND {
                        _bfs_delete(&mut grid, row, col, &mut queue);
                        queue.clear();
                        count += 1;
                    }
                }
            }
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dfs::Solution::num_islands);
        test(solution_bfs::Solution::num_islands);
    }

    fn test<F: Fn(Vec<Vec<char>>) -> i32>(func: F) {
        let grid = [
            ["1", "1", "1", "1", "0"],
            ["1", "1", "0", "1", "0"],
            ["1", "1", "0", "0", "0"],
            ["0", "0", "0", "0", "0"],
        ]
        .iter()
        .map(|a| a.iter().map(|e| e.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(func(grid), 1);
    }
}
