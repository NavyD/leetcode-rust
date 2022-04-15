pub mod solution_dfs {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [Neat DFS java solution](https://leetcode.com/problems/number-of-provinces/discuss/101338/Neat-DFS-java-solution)
    ///
    /// ### Submissions
    ///
    /// date=20220415, mem=2.2, mem_beats=100, runtime=0, runtime_beats=100
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

            let n = is_connected.len();
            let mut count = 0;
            let mut visited = vec![false; n];
            for i in 0..n {
                if !visited[i] {
                    dfs(&is_connected, i, &mut visited);
                    count += 1;
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
    fn basics() {
        test(solution_dfs::Solution::find_circle_num);
    }

    fn test<F: Fn(Vec<Vec<i32>>) -> i32>(f: F) {
        assert_eq!(f(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]), 2);
        assert_eq!(f(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
    }
}
