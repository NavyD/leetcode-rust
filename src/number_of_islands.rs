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
    ///         && 0 <= c && c < grid[0].length;
    /// }
    /// ```
    ///
    /// ## 如何避免重复遍历
    ///
    /// 标记已经遍历过的格子。以岛屿问题为例，我们需要在所有值为 1 的陆地格子上做 DFS 遍历。
    /// 每走过一个陆地格子，就把格子的值改为 2，这样当我们遇到 2 的时候，就知道这是遍历过的格子了。
    ///
    /// ## 不修改原数组
    ///
    /// ```no_run
    /// // 8ms
    /// pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    ///     const LAND: char = '1';
    ///     fn dfs(grid: &[Vec<char>], i: usize, j: usize, visited: &mut [Vec<bool>]) {
    ///         if i < grid.len() && j < grid[0].len() && grid[i][j] == LAND && !visited[i][j] {
    ///             visited[i][j] = true;
    ///             dfs(grid, i + 1, j, visited);
    ///             dfs(grid, i, j + 1, visited);
    ///             if i > 0 {
    ///                 dfs(grid, i - 1, j, visited);
    ///             }
    ///             if j > 0 {
    ///                 dfs(grid, i, j - 1, visited);
    ///             }
    ///         }
    ///     }
    ///     let (m, n) = (grid.len(), grid[0].len());
    ///     let mut visited = vec![vec![false; n]; m];
    ///     let mut count = 0;
    ///     for i in 0..m {
    ///         for j in 0..n {
    ///             if grid[i][j] == LAND && !visited[i][j] {
    ///                 count += 1;
    ///                 dfs(&grid, i, j, &mut visited);
    ///             }
    ///         }
    ///     }
    ///     count
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [岛屿类问题的通用解法、DFS 遍历框架](https://leetcode-cn.com/problems/number-of-islands/solution/dao-yu-lei-wen-ti-de-tong-yong-jie-fa-dfs-bian-li-/)
    ///
    /// ### Submissions
    ///
    /// date=20210109, mem=5.2, mem_beats=50, runtime=8, runtime_beats=30, url=https://leetcode-cn.com/submissions/detail/137091727/
    ///
    /// date=20210110, mem=5.2, mem_beats=40, runtime=8, runtime_beats=54, url=https://leetcode-cn.com/submissions/detail/137401861/
    ///
    /// date=20210117, mem=5.2, mem_beats=57, runtime=8, runtime_beats=60, url=https://leetcode-cn.com/submissions/detail/139014563/
    ///
    /// date=20220515, mem=8.8, mem_beats=72, runtime=12, runtime_beats=72
    pub struct Solution;

    impl Solution {
        pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
            const LAND: char = '1';
            const NON_LAND: char = '2';

            fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
                if row < grid.len() && col < grid[0].len() && grid[row][col] == LAND {
                    grid[row][col] = NON_LAND;
                    dfs(grid, row + 1, col);
                    dfs(grid, row, col + 1);
                    if row > 0 {
                        dfs(grid, row - 1, col);
                    }
                    if col > 0 {
                        dfs(grid, row, col - 1);
                    }
                }
            }

            let mut count = 0;
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if grid[row][col] == LAND {
                        dfs(&mut grid, row, col);
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
    ///
    /// date=20210110, mem=5.2, mem_beats=81, runtime=8, runtime_beats=54, url=https://leetcode-cn.com/submissions/detail/137403628/
    ///
    /// date=20210117, mem=5.4, mem_beats=7, runtime=4, runtime_beats=92, url=https://leetcode-cn.com/submissions/detail/139015792/
    pub struct Solution;

    impl Solution {
        pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
            use std::collections::VecDeque;
            const LAND: char = '1';
            const DELETED_LAND: char = '2';

            fn _bfs_delete(grid: &mut Vec<Vec<char>>, queue: &mut VecDeque<(usize, usize)>) {
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
                        queue.push_back((row, col));
                        _bfs_delete(&mut grid, &mut queue);
                        // queue在出来时已清空
                        // queue.clear();
                        count += 1;
                    }
                }
            }
            count
        }
    }
}

pub mod solution_union {
    /// # 思路
    ///
    /// 注意：
    ///
    /// * 只有两个方向`[(1, 0), (0, 1)]`
    ///
    /// 对于并查集的做法，第一行向下尝试合并，与第二行向上尝试合并是同一个操作。
    /// 左边格子和右边格子尝试合并，与右边格子和左边格子尝试合并是同一个操作。
    /// 所以只用考虑两个方向。
    ///
    /// 参考：
    ///
    /// - [1D Union Find Java solution, easily generalized to other problems](https://leetcode.com/problems/number-of-islands/discuss/56354/1D-Union-Find-Java-solution-easily-generalized-to-other-problems)
    /// - [方法三：并查集](https://leetcode.cn/problems/number-of-islands/solution/dfs-bfs-bing-cha-ji-python-dai-ma-java-dai-ma-by-l/)
    /// - [岛屿数量 方法三：并查集](https://leetcode.cn/problems/number-of-islands/solution/dao-yu-shu-liang-by-leetcode/)
    ///
    /// ### Submissions
    ///
    /// date=20220515, mem=9, mem_beats=12, runtime=8, runtime_beats=100
    ///
    /// date=20220524, mem=8.8, mem_beats=65, runtime=12, runtime_beats=68
    pub struct Solution;

    impl Solution {
        pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
            const LAND: char = '1';

            struct UnionFind {
                count: i32,
                parent: Vec<usize>,
                cols: usize,
                rank: Vec<u32>,
            }
            impl UnionFind {
                fn new(rows: usize, cols: usize) -> Self {
                    let count = rows * cols;
                    Self {
                        count: count as i32,
                        parent: (0..count).collect::<Vec<_>>(),
                        cols,
                        rank: vec![0; count],
                    }
                }

                fn find(&mut self, mut p: usize) -> usize {
                    while p != self.parent[p] {
                        self.parent[p] = self.parent[self.parent[p]];
                        p = self.parent[p];
                    }
                    p
                }

                fn union(&mut self, p: usize, q: usize) {
                    let (mut p, mut q) = (self.find(p), self.find(q));
                    if p == q {
                        return;
                    }
                    use std::cmp::Ordering::*;
                    match self.rank[p].cmp(&self.rank[q]) {
                        Equal => self.rank[q] += 1,
                        Greater => std::mem::swap(&mut p, &mut q),
                        _ => {}
                    }
                    self.parent[p] = q;
                    self.count -= 1;
                }

                #[inline(always)]
                fn index(&self, row: usize, col: usize) -> usize {
                    self.cols * row + col
                }
            }

            let (rows, cols) = (grid.len(), grid[0].len());
            let mut uf = UnionFind::new(rows, cols);
            let mut spaces = 0;

            for i in 0..rows {
                for j in 0..cols {
                    if grid[i][j] == LAND {
                        if i > 0 && grid[i - 1][j] == LAND {
                            uf.union(uf.index(i, j), uf.index(i - 1, j));
                        }
                        if j > 0 && grid[i][j - 1] == LAND {
                            uf.union(uf.index(i, j), uf.index(i, j - 1));
                        }
                    } else {
                        spaces += 1;
                    }
                }
            }
            uf.count - spaces
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
        test(solution_union::Solution::num_islands);
    }

    fn test<F: Fn(Vec<Vec<char>>) -> i32>(func: F) {
        fn arr<const M: usize, const N: usize>(a: [[&str; N]; M]) -> Vec<Vec<char>> {
            a.into_iter()
                .map(|a| {
                    a.into_iter()
                        .map(|s| s.chars().next().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        assert_eq!(
            func(arr([
                ["1", "1", "1", "1", "0"],
                ["1", "1", "0", "1", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "0", "0", "0"],
            ])),
            1
        );
        assert_eq!(
            func(arr([
                ["1", "1", "0", "0", "0"],
                ["1", "1", "0", "0", "0"],
                ["0", "0", "1", "0", "0"],
                ["0", "0", "0", "1", "1"],
            ])),
            3
        );
    }
}
