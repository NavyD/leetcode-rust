pub mod solution_dfs {
    /// # 思路
    ///
    /// 注意dfs时搜索到一个城市时，下一次从当前城市开始搜索下一个相连城市，
    /// 所以要传入i `fn dfs(is_connected: &[Vec<i32>], i: usize, visited: &mut [bool])`
    ///
    /// 参考：
    ///
    /// * [Neat DFS java solution](https://leetcode.com/problems/number-of-provinces/discuss/101338/Neat-DFS-java-solution)
    ///
    /// ### Submissions
    ///
    /// date=20220415, mem=2.2, mem_beats=100, runtime=0, runtime_beats=100
    ///
    /// date=20220508, mem=2.2, mem_beats=33, runtime=0, runtime_beats=100
    ///
    /// date=20220514, mem=2.4, mem_beats=22, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
            fn dfs(is_connected: &[Vec<i32>], i: usize, visited: &mut [bool]) {
                for j in 0..is_connected.len() {
                    if is_connected[i][j] == 1 && !visited[j] {
                        visited[j] = true;
                        dfs(is_connected, j, visited);
                    }
                }
            }

            let cities = is_connected.len();
            let mut count = 0;
            let mut visited = vec![false; cities];

            for i in 0..cities {
                if !visited[i] {
                    dfs(&is_connected, i, &mut visited);
                    count += 1;
                }
            }
            count
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [DFS + BFS + 并查集，3 种方法计算无向图连通域数量](https://leetcode-cn.com/problems/number-of-provinces/solution/dfs-bfs-bing-cha-ji-3-chong-fang-fa-ji-s-edkl/)
    ///
    /// ### Submissions
    ///
    /// date=20220416, mem=2.2, mem_beats=50, runtime=0, runtime_beats=100
    ///
    /// date=20220508, mem=2.3, mem_beats=33, runtime=4, runtime_beats=55
    pub struct Solution;

    impl Solution {
        pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
            let cities = is_connected.len();
            let mut provinces = 0;

            let mut visited = vec![false; cities];
            let mut queue = std::collections::VecDeque::default();

            for i in 0..cities {
                if !visited[i] {
                    queue.push_back(i);

                    while let Some(i) = queue.pop_front() {
                        visited[i] = true;

                        for j in 0..cities {
                            if is_connected[i][j] == 1 && !visited[j] {
                                queue.push_back(j);
                            }
                        }
                    }

                    provinces += 1;
                }
            }
            provinces
        }
    }
}

pub mod solution_union {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [DFS + BFS + 并查集，3 种方法计算无向图连通域数量](https://leetcode-cn.com/problems/number-of-provinces/solution/dfs-bfs-bing-cha-ji-3-chong-fang-fa-ji-s-edkl/)
    /// * [Java solution, Union Find](https://leetcode.com/problems/number-of-provinces/discuss/101336/Java-solution-Union-Find)
    /// * [方法三：并查集](https://leetcode.cn/problems/number-of-provinces/solution/sheng-fen-shu-liang-by-leetcode-solution-eyk0/)
    /// * [[Python/C++/Java] 多图详解并查集](https://leetcode.cn/problems/number-of-provinces/solution/python-duo-tu-xiang-jie-bing-cha-ji-by-m-vjdr/)
    ///
    /// ### Submissions
    ///
    /// date=20220508, mem=2.1, mem_beats=77, runtime=0, runtime_beats=100
    ///
    /// date=20220514, mem=2.1, mem_beats=74, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
            struct UnionFind {
                parent: Vec<usize>,
                count: i32,
            }

            impl UnionFind {
                fn new(n: usize) -> Self {
                    Self {
                        count: n as i32,
                        parent: (0..n).into_iter().collect(),
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
                    let (pp, pq) = (self.find(p), self.find(q));
                    if pp != pq {
                        self.parent[pp] = pq;
                        self.count -= 1;
                    }
                }
            }

            let n = is_connected.len();
            let mut uf = UnionFind::new(n);
            for i in 0..n {
                for j in 0..n {
                    if is_connected[i][j] == 1 {
                        uf.union(i, j);
                    }
                }
            }

            uf.count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dfs::Solution::find_circle_num);
        test(solution_bfs::Solution::find_circle_num);
        test(solution_union::Solution::find_circle_num);
    }

    fn test<F: Fn(Vec<Vec<i32>>) -> i32>(f: F) {
        assert_eq!(f(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1],]), 2);
        assert_eq!(f(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
        assert_eq!(
            f(vec![
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 1]
            ]),
            1
        );
    }
}
