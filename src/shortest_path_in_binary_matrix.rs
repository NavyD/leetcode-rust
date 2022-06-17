pub mod solution_bfs {
    /// # 思路
    ///
    /// 注意：使用hashset保存要40ms
    ///
    /// `if i == n - 1 && j == n - 1 {`放在`for (x, y) in DIRS {`前以避免多余的判断。
    /// 当`grid=[[0]]`时无法进入for中，没有判断当前点
    ///
    /// 参考：
    ///
    /// * [BFS 最短路径问题（BFS，DFS 的思考）](https://leetcode.cn/problems/shortest-path-in-binary-matrix/solution/bfszui-duan-lu-jing-wen-ti-bfsdfsde-si-k-ngc5/)
    ///
    /// ### Submissions
    ///
    /// date=20220602, mem=2.1, mem_beats=85, runtime=8, runtime_beats=85
    ///
    /// date=20220607, mem=2.2, mem_beats=66, runtime=12, runtime_beats=41
    ///
    /// date=20220617, mem=2.1, mem_beats=92, runtime=8, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
            const DIRS: [(i32, i32); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
            ];
            const NOT_FOUND: i32 = -1;

            let n = grid.len();
            if grid[0][0] != 0 || grid[n - 1][n - 1] != 0 {
                return NOT_FOUND;
            }

            let mut visited = vec![false; n * n];
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((0, 0));

            let n = n as i32;
            let mut distance = 0;
            while !queue.is_empty() {
                distance += 1;
                for _ in 0..queue.len() {
                    let (i, j) = queue.pop_front().unwrap();

                    if i == n - 1 && j == n - 1 {
                        return distance;
                    }

                    for (x, y) in DIRS {
                        let (i, j) = (i + x, j + y);
                        if i >= 0
                            && j >= 0
                            && i < n
                            && j < n
                            && grid[i as usize][j as usize] == 0
                            && !visited[(i * n + j) as usize]
                        {
                            queue.push_back((i, j));
                            visited[(i * n + j) as usize] = true;
                        }
                    }
                }
            }

            NOT_FOUND
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_bfs::Solution::shortest_path_binary_matrix);
    }

    fn test(f: impl Fn(Vec<Vec<i32>>) -> i32) {
        fn arr<const N: usize>(a: [[i32; N]; N]) -> Vec<Vec<i32>> {
            a.into_iter().map(FromIterator::from_iter).collect()
        }

        assert_eq!(f(arr([[0, 0, 0], [1, 1, 0], [1, 1, 0]])), 4);
        assert_eq!(f(arr([[1, 0, 0], [1, 1, 0], [1, 1, 0]])), -1);
        assert_eq!(f(arr([[0]])), 1);
    }
}
