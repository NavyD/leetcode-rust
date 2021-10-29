pub mod solution_backtracking {
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
    /// 参考：
    ///
    /// * [戳气球](https://leetcode-cn.com/problems/burst-balloons/solution/chuo-qi-qiu-by-leetcode-solution/)
    /// * [超详细回溯到分治到DP](https://leetcode-cn.com/problems/burst-balloons/solution/chao-xiang-xi-hui-su-dao-fen-zhi-dao-dp-by-niu-you/)
    ///
    /// ### Submissions
    ///
    /// date=20211028, mem=3, mem_beats=10, runtime=192, runtime_beats=10
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

                    max_coins[left][right] = coins.max(max_coins[left][right])
                }
                max_coins[left][right]
            }

            let n = nums.len() + 2;
            nums.insert(0, 1);
            nums.push(1);

            solve(0, n - 1, &mut nums, &mut vec![vec![MARKED; n]; n])
        }
    }
}

pub mod solution_dp {

    /// # 思路
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
    pub struct Solution;

    impl Solution {
        pub fn max_coins(mut nums: Vec<i32>) -> i32 {
            nums.insert(0, 1);
            nums.push(1);
            let n = nums.len();

            let mut dp = vec![vec![0; n]; n];
            for len in 3..=n {
                for left in 0..=n - len {
                    let right = left + len - 1;
                    let mut max_coins = 0;
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
    }

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![3, 1, 5, 8]), 167);
        assert_eq!(f(vec![1, 5]), 10);
    }
}
