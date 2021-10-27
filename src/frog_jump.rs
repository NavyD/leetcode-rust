//! 题意为青蛙要在i位置上跳`next_k = k|+-1`距离后的位置要在石子stones列表中找到，
//! 即`stones[i] + next_k in stones`
//!
//! dp时要注意如何找出前后k关系

pub mod solution_backtracking {
    /// # 思路
    ///
    /// 题意为青蛙要在i位置上跳`next_k = k|+-1`距离后的位置要在石子stones列表中找到，
    /// 即`stones[i] + next_k in stones`
    ///
    /// 我们可以使用 DFS 来模拟/爆搜一遍，检查所有的可能性中是否有能到达最后一块石子的。
    /// 通常设计 DFS 函数时，我们只需要不失一般性的考虑完成第 i 块石子的跳跃需要些什么信息即可：
    ///
    /// - 需要知道当前所在位置在哪，也就是需要知道当前石子所在列表中的下标 u。
    /// - 需要知道当前所在位置是经过多少步而来的，也就是需要知道上一步的跳跃步长 k。
    ///
    /// ### Submissions
    ///
    /// date=20210902, mem=5.8, mem_beats=66, runtime=8, runtime_beats=100
    ///
    /// date=20210904, mem=5.9, mem_beats=25, runtime=12, runtime_beats=75
    ///
    /// date=20210915, mem=5.8, mem_beats=70, runtime=8, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn can_cross(stones: Vec<i32>) -> bool {
            fn backtrack(stones: &[i32], k: i32, crosses: &mut [Vec<Option<bool>>]) -> bool {
                if stones.len() == 1 {
                    return true;
                }
                if let Some(v) = crosses[0][k as usize] {
                    return v;
                }
                for i in -1..=1 {
                    let next_k = k + i;
                    // k=1,i=-1 skipped
                    if next_k != 0
                        && stones
                            // find next
                            .binary_search(&(stones[0] + next_k))
                            .map(|i| backtrack(&stones[i..], next_k, &mut crosses[i..]))
                            // not found next
                            .unwrap_or(false)
                    {
                        crosses[0][k as usize] = Some(true);
                        return true;
                    }
                }
                crosses[0][k as usize] = Some(false);
                false
            }

            let n = stones.len();
            if stones[1] > 1 {
                false
            } else {
                backtrack(&stones[1..], 1, &mut vec![vec![None; n]; n])
            }
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// 这样的「状态定义」所代表的含义：当前在第 i 个位置，并且是以步长 k 跳到位置 i 时，是否到达最后一块石子。
    ///
    /// 令 `dp[i][k]` 表示青蛙能否达到 现在所处的石子编号 为 i 且 上一次跳跃距离为 k 的状态，可以
    /// 由上一次所在的石子编号j与跳跃的距离k决定。对于第 i 个石子，我们首先枚举所有的 j表示在i上可跳的位置，
    /// 这里的上一次跳跃距离k可以在i的基础上由`j in 1..=i-1`跳过来的步长`k=stones[i]-stones[j]`。
    ///
    /// 优化
    ///
    /// 1. 第 j 个石子上我们至多只能跳出 j+1 的距离`k <= j+1`，因为每次跳跃，下标至少增加 1，而步长最多增加 1
    /// 从距离k小开始一旦遇到`k>j+1`，表示后面的是不可达退出，反向枚举j `for j in (0..i).rev()`。
    /// 1. 如果发现n-1可以到达时提前返回：`if i == n - 1 && dp[i][k]: return true`，否则需要主动在n-1中找
    /// 到达的k：`for k in 0..n if dp[n-1][k]: return true`
    ///
    /// - dp方程：`dp[i][k] = dp[j][k] || dp[j][k-1] || dp[j][k+1] for j in i..0, k=stones[i]-stones[j] if k > j+1 break`
    /// - 初始化：`dp[0][0]=true`则`dp[1][1] = dp[0][0] = true`
    ///
    /// 参考：
    ///
    /// - [青蛙过河 方法二：动态规划](https://leetcode-cn.com/problems/frog-jump/solution/qing-wa-guo-he-by-leetcode-solution-mbuo/)
    /// - [【宫水三叶】一题四解 : 降低确定「记忆化容器大小」的思维难度 & 利用「对偶性质」构造有效状态值 动态规划](https://leetcode-cn.com/problems/frog-jump/solution/gong-shui-san-xie-yi-ti-duo-jie-jiang-di-74fw/)
    ///
    /// ### Submissions
    ///
    /// date=20210904, mem=5.8, mem_beats=100, runtime=28, runtime_beats=25
    ///
    /// date=20210916, mem=5.8, mem_beats=100, runtime=12, runtime_beats=80
    ///
    /// date=20211027, mem=5.7, mem_beats=83, runtime=16, runtime_beats=41
    pub struct Solution;

    impl Solution {
        pub fn can_cross(stones: Vec<i32>) -> bool {
            let n = stones.len();
            let mut dp = vec![vec![false; n]; n];
            dp[0][0] = true;

            for i in 1..n {
                for j in (0..i).rev() {
                    let k = (stones[i] - stones[j]) as usize;
                    if k > j + 1 {
                        break;
                    }
                    dp[i][k] = dp[j][k] || dp[j][k - 1] || dp[j][k + 1];
                    if i == n - 1 && dp[i][k] {
                        return true;
                    }
                }
            }
            false
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// 从[super::solution_backtracking::Solution]修改
    ///
    /// 参考：
    ///
    /// - [【宫水三叶】一题四解 : 降低确定「记忆化容器大小」的思维难度 & 利用「对偶性质」构造有效状态值 BFS](https://leetcode-cn.com/problems/frog-jump/solution/gong-shui-san-xie-yi-ti-duo-jie-jiang-di-74fw/)
    ///
    /// ### Submissions
    ///
    /// date=20210904, mem=5.7, mem_beats=100, runtime=36, runtime_beats=25
    pub struct Solution;

    impl Solution {
        pub fn can_cross(stones: Vec<i32>) -> bool {
            let n = stones.len();
            if n < 2 || stones[1] > 1 {
                return false;
            }
            // 特判 stones[1]==1
            if n == 2 {
                return true;
            }

            let mut queue = std::collections::VecDeque::new();
            queue.push_back((1, 1));

            let mut visited = vec![vec![false; n]; n];
            visited[1][1] = true;

            // 不需要分层次了
            while let Some((cur, k)) = queue.pop_front() {
                for i in -1..2 {
                    let next_k = k + i;
                    if next_k == 0 {
                        continue;
                    }
                    if let Ok(next) = stones.binary_search(&(stones[cur] + next_k)) {
                        if next == n - 1 {
                            return true;
                        } else if !visited[next][next_k as usize] {
                            visited[next][next_k as usize] = true;
                            queue.push_back((next, next_k));
                        }
                    }
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_backtracking::Solution::can_cross);
        test(solution_dp::Solution::can_cross);
        test(solution_bfs::Solution::can_cross);
    }

    fn test<F: Fn(Vec<i32>) -> bool>(f: F) {
        assert!(f(vec![0, 1, 3, 5, 6, 8, 12, 17]));
        assert!(!f(vec![0, 1, 2, 3, 4, 8, 9, 11]));
        assert!(f(vec![0, 1]));
        assert!(!f(vec![0, 2]));
    }
}
