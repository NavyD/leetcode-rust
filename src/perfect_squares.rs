pub mod solution_backtracking {
    /// # 思路
    ///
    /// 先把 n 减去一个平方数，然后求剩下的数分解成平方数和所需的最小个数.
    /// 只需要从 (n-square1 + 1), (n-square2 + 1), (n-square.. + 1) 多种方案中选择最小的个数
    ///
    /// 参考：
    ///
    /// * [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/perfect-squares/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by--51/)
    ///
    /// ### Submissions
    ///
    /// date=20210701, mem=2.6, mem_beats=5, runtime=64, runtime_beats=14, url=https://leetcode-cn.com/submissions/detail/191269383/
    ///
    /// date=20210716, mem=2.4, mem_beats=8, runtime=52, runtime_beats=17, url=https://leetcode-cn.com/submissions/detail/196327001/
    pub struct Solution;

    impl Solution {
        pub fn num_squares(n: i32) -> i32 {
            const MARK: i32 = -1;
            fn backtrack(n: usize, cache: &mut Vec<i32>) -> i32 {
                // 剪枝 重复计算
                if cache[n] != MARK {
                    return cache[n];
                }
                if n == 0 {
                    return 0;
                }
                let mut min_count = i32::MAX;
                for i in 1..=n {
                    let rest_sum = n as i32 - (i * i) as i32;
                    if rest_sum < 0 {
                        break;
                    }
                    // 计算剩下的 sum. 统计+1
                    min_count = min_count.min(backtrack(rest_sum as usize, cache) + 1);
                }
                cache[n] = min_count;
                min_count
            }
            let n = n as usize;
            backtrack(n, &mut vec![MARK; n + 1])
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// 定义problem(i)表示和为i的完全平方数的数量。如何计算，
    /// 当从1开始计算square数遍历，每计算一个square，可以由 `problem(i-square) + 1`
    /// 计算出problem(i)，表示由i-square作为剩下的和。
    /// 最小数量：`min(problem(i), problem(i-square)+1)`
    ///
    /// dp方程：`dp[i] = min(dp[i], dp[i - square] + 1) for square in 1..n`
    ///
    /// 初始化：len=n+1. 当i=1时，`dp[1] = min(dp[1], dp[0] + 1) = 1`，dp[0]应该初始化为0，`dp[1]=i32::MAX`
    /// 即`dp[1..]=i32::MAX`
    ///
    /// 参考：
    ///
    /// * [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/perfect-squares/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by--51/)
    ///
    /// ### Submissions
    ///
    /// date=20210701, mem=2.2, mem_beats=18, runtime=40, runtime_beats=45, url=https://leetcode-cn.com/submissions/detail/191283031/
    ///
    /// date=20210706, mem=2.1, mem_beats=59, runtime=40, runtime_beats=45, url=https://leetcode-cn.com/submissions/detail/192796308/
    ///
    /// date=20210716, mem=2.2, mem_beats=35, runtime=36, runtime_beats=58, url=https://leetcode-cn.com/submissions/detail/196317183/
    pub struct Solution;

    impl Solution {
        pub fn num_squares(n: i32) -> i32 {
            let n = n as usize;
            let mut dp = vec![i32::MAX; n + 1];
            dp[0] = 0;

            for i in 1..=n {
                for j in 1..=i {
                    let rest_sum = i as i32 - (j * j) as i32;
                    if rest_sum < 0 {
                        break;
                    }
                    dp[i] = dp[i].min(dp[rest_sum as usize] + 1);
                }
            }
            dp[n]
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/perfect-squares/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by--51/)
    ///
    /// ### Submissions
    ///
    /// date=20210701, mem=2.3, mem_beats=15, runtime=164, runtime_beats=10, url=https://leetcode-cn.com/submissions/detail/191286428/
    ///
    /// date=20210706, mem=2.1, mem_beats=48, runtime=168, runtime_beats=7, url=https://leetcode-cn.com/submissions/detail/192797893/
    ///
    /// 优化了`else if rest_sum > 0 && !visited[rest_sum as usize] {`为break提前退出
    ///
    /// date=20210716, mem=1.9, mem_beats=96, runtime=4, runtime_beats=81, url=https://leetcode-cn.com/submissions/detail/196332621/
    pub struct Solution;

    impl Solution {
        pub fn num_squares(n: i32) -> i32 {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(n);

            let n = n as usize;
            let mut visited = vec![false; n + 1];
            visited[n] = true;

            let mut steps = 0;
            while !queue.is_empty() {
                steps += 1;
                for _ in 0..queue.len() {
                    let sum = queue.pop_front().unwrap();
                    for i in 1..=sum {
                        let rest_sum = sum - i * i;
                        use std::cmp::Ordering::*;
                        match rest_sum.cmp(&0) {
                            Greater => {
                                let rest_sum = rest_sum as usize;
                                if !visited[rest_sum] {
                                    visited[rest_sum] = true;
                                    queue.push_back(rest_sum as i32);
                                }
                            }
                            Equal => return steps,
                            Less => break,
                        }
                    }
                }
            }
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_backtracking::Solution::num_squares);
        test(solution_dp::Solution::num_squares);
        test(solution_bfs::Solution::num_squares);
    }

    fn test<F: Fn(i32) -> i32>(f: F) {
        assert_eq!(f(12), 3);
        assert_eq!(f(13), 2);
        assert_eq!(f(54), 3);
    }
}
