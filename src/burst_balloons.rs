pub mod solution_backtracking {
    /// 参考：
    ///
    /// * [超详细回溯到分治到DP](https://leetcode-cn.com/problems/burst-balloons/solution/chao-xiang-xi-hui-su-dao-fen-zhi-dao-dp-by-niu-you/)
    pub struct Solution;

    impl Solution {
        pub fn max_coins(mut nums: Vec<i32>) -> i32 {
            const MARKED: i32 = -1;

            fn backtrack(nums: &mut [i32], k: usize, last_coins: i32, max_coins: &mut i32) {
                // 回溯结束条件
                if k == nums.len() {
                    // 更新最大数量在完成一层后
                    if last_coins > *max_coins {
                        *max_coins = last_coins;
                    }
                    return;
                }

                // 枚举戳破所有
                for i in 0..nums.len() {
                    let cur_num = nums[i];
                    // 标记
                    nums[i] = MARKED;

                    // 找前后未戳破的
                    let last_num = {
                        let mut i = i;
                        loop {
                            if i == 0 {
                                break 1;
                            } else if nums[i] != MARKED {
                                break nums[i];
                            } else {
                                i -= 1;
                            }
                        }
                    };
                    let next_num = {
                        let mut i = i;
                        loop {
                            if i >= nums.len() {
                                break 1;
                            } else if nums[i] != MARKED {
                                break nums[i];
                            } else {
                                i += 1;
                            }
                        }
                    };

                    // 计算当前硬币数量
                    let cur_coins = last_num * cur_num * next_num;
                    // 下一层
                    backtrack(nums, k + 1, cur_coins + last_coins, max_coins);
                    // 恢复
                    nums[i] = cur_num;
                }
            }
            let mut max_coins = 0;
            backtrack(&mut nums, 0, 0, &mut max_coins);
            max_coins
        }
    }
}

pub mod solution_recursive {
    ///
    /// 设戳破区间 i 到 j 间的气球我们得到的最大金币数为 coin。及 coin = def (i , j)。
    /// 当我们戳破气球 k 时，两边区间的最大值分别是 def (i , k-1) 与 def ( k+1 , j )。
    ///
    /// 我们发现了问题，因为戳破了气球 k ，气球数组的相邻关系发生了改变，k-1 与 k+1 原本都与 k 相邻，而 k 戳破后他们两个直接相邻了。
    /// 而且先戳破 k+1 与先戳破 k-1 得到的结果将完全不同，也就是说两个子问题间发生了依赖。如果先戳破 k-1 ，
    /// 则 k+1 左边的相邻气球变成了 k-2；反之 k-1 右边相邻的气球变成了 k+2 。
    ///
    /// 子问题的处理顺序将影响到每个子问题的解，这将使我们的状态转移方程极为复杂和低效，我们应当换一种划分子问题的方式，使每个子问题都是独立的。
    ///
    /// 既然两个子问题都依赖 k 和两个边界，那么我们划分子问题时，k 与两个边界的气球我们都不戳破，求出 i+1 到 k-1 与 k+1 到 j-1 之间的解。
    /// 这样两个子问题间的依赖便被消除了，两个边界及气球 k 不被戳破，两个子问题的依赖都不会越过 k 到另一个子问题上，子问题间是相互独立的
    ///
    /// 在两个子问题解决后，气球序列还剩下 k 与两个边界的气球没有戳破，那么我们用两个子问题的解与戳破 k 与两个边界的最大值即可求出原问题的解。
    ///
    /// def (i , j) 函数的定义则为，不戳破 i 与 j ，仅戳破 i 与 j 之间的气球我们能得到的最大金币数。
    /// 如此划分，状态转移方程为： def (i, j) = def ( i , k ) + def ( k , j )+nums [ i ][ j ][ k ]
    ///
    /// 边界1不影响 `nums[i - 1] * nums[i] * nums[i + 1]`
    ///
    /// 参考：
    ///
    /// * [戳气球](https://leetcode-cn.com/problems/burst-balloons/solution/chuo-qi-qiu-by-leetcode-solution/)
    /// * [超详细回溯到分治到DP](https://leetcode-cn.com/problems/burst-balloons/solution/chao-xiang-xi-hui-su-dao-fen-zhi-dao-dp-by-niu-you/)
    ///
    /// ### Submissions
    ///
    /// date=20211028, mem=3, mem_beats=10, runtime=192, runtime_beats=10
    ///
    /// date=20220314, mem=2.5, mem_beats=50, runtime=100, runtime_beats=12
    pub struct Solution;

    impl Solution {
        pub fn max_coins(mut nums: Vec<i32>) -> i32 {
            const MARKED: i32 = -1;
            fn solve(
                left: usize,
                right: usize,
                nums: &mut [i32],
                max_coins: &mut [Vec<i32>],
            ) -> i32 {
                if left >= right - 1 {
                    return 0;
                }
                if max_coins[left][right] != MARKED {
                    return max_coins[left][right];
                }

                for i in left + 1..right {
                    let coins = nums[left] * nums[i] * nums[right]
                        + solve(left, i, nums, max_coins)
                        + solve(i, right, nums, max_coins);
                    if coins > max_coins[left][right] {
                        max_coins[left][right] = coins;
                    }
                }
                max_coins[left][right]
            }

            // 添加边界
            nums.insert(0, 1);
            nums.push(1);

            let n = nums.len();
            solve(0, n - 1, &mut nums, &mut vec![vec![MARKED; n]; n])
        }
    }
}

pub mod solution_dp {

    /// # 思路
    ///
    /// 假设这个区间是个开区间，最左边索引 i，最右边索引 j，只能戳爆 i 和 j 之间的气球，i 和 j 不要戳，
    /// k 是这个区间   最后一个   被戳爆的气球，在 (i,j) 开区间得到的金币可以由 dp[i][k] 和 dp[k][j] 进行转移
    ///
    /// ![](https://pic.leetcode-cn.com/e994d9928ff254477dab117275d0e0e4ed3b81fadd637c3203728d4fe7066eac-gaitubao_%E5%B1%8F%E5%B9%95%E5%BF%AB%E7%85%A7%202020-07-19%20%E4%B8%8B%E5%8D%886.29.13.png)
    ///
    /// dp方程：`dp[i][j] = dp[i][k] + dp[k][j] + val[i] * val[k] * val[j]`
    ///
    ///
    /// 参考：
    ///
    /// * [[这个菜谱, 自己在家也能做] 关键思路解释](https://leetcode-cn.com/problems/burst-balloons/solution/zhe-ge-cai-pu-zi-ji-zai-jia-ye-neng-zuo-guan-jian-/)
    /// * [戳气球](https://leetcode-cn.com/problems/burst-balloons/solution/chuo-qi-qiu-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20211029, mem=2.9, mem_beats=60, runtime=44, runtime_beats=100
    ///
    /// date=20220314, mem=2.4, mem_beats=62, runtime=28, runtime_beats=87
    ///
    /// date=20220324, mem=2.5, mem_beats=40, runtime=40, runtime_beats=70
    pub struct Solution;

    impl Solution {
        pub fn max_coins(mut nums: Vec<i32>) -> i32 {
            // nums首尾添加1，方便处理边界情况
            nums.insert(0, 1);
            nums.push(1);

            let n = nums.len();
            let mut dp = vec![vec![0; n]; n];

            // 对每一个区间长度进行循环，最少3个元素
            for len in 3..=n {
                // 区间左开点
                for left in 0..=n - len {
                    // 区间右开点
                    let right = left + len - 1;

                    let mut max_coins = 0;
                    // k 最后被戳爆点
                    for k in left + 1..right {
                        max_coins = max_coins
                            .max(dp[left][k] + dp[k][right] + nums[left] * nums[right] * nums[k]);
                    }
                    dp[left][right] = max_coins;
                }
            }
            dp[0][n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_recursive::Solution::max_coins);
        test(solution_dp::Solution::max_coins);
        // test(solution_backtracking::Solution::max_coins);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![3, 1, 5, 8]), 167);
        assert_eq!(f(vec![1, 5]), 10);
    }
}
