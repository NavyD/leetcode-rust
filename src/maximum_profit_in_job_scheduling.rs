//! We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].
//!
//! You're given the startTime , endTime and profit arrays, you need to output the maximum profit you can take such that there are no 2 jobs in the subset with overlapping time range.
//!
//! If you choose a job that ends at time X you will be able to start another job that starts at time X.
//!
//!  
//!
//! Example 1:
//!
//! ![](https://assets.leetcode.com/uploads/2019/10/10/sample1_1584.png)
//!
//! Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
//! Output: 120
//! Explanation: The subset chosen is the first and fourth job.
//! Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
//!
//! Example 2:
//!
//! ![](https://assets.leetcode.com/uploads/2019/10/10/sample22_1584.png)
//!
//!
//! Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
//! Output: 150
//! Explanation: The subset chosen is the first, fourth and fifth job.
//! Profit obtained 150 = 20 + 70 + 60.
//!
//! Example 3:
//!
//! ![](https://assets.leetcode.com/uploads/2019/10/10/sample3_1584.png)
//!
//! Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
//! Output: 6
//!  
//!
//! Constraints:
//!
//! 1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4
//! 1 <= startTime[i] < endTime[i] <= 10^9
//! 1 <= profit[i] <= 10^4

pub mod solution_dp {
    pub struct Solution;

    struct Job {
        start_time: i32,
        end_time: i32,
        profit: i32,
    }
    struct Dp {
        end_time: i32,
        profit: i32,
    }

    impl Solution {
        /// # 思路
        ///
        /// 用dp的方式解决，设`dp[time] = max_profit`表示在[0,time]的最大的profit，当time=n时
        /// dp[n]=max_profit.
        ///
        /// 由于dp[n+1]要与dp[n]的profit比较，如果dp[n+1]的max profit比dp[n]
        /// 的profit大时则更新为dp[n+1]，否则还是dp[n]的profit，
        /// 即`dp[n]=max(dp[n-1].profit, dp[n].profit)`
        ///
        /// ## 问题
        ///
        /// - 如何解决P[end_time]与P[end_time+1]时可能出现Sum更大或不变的情况
        ///
        ///   比较dp.end_time与当前end_time(+1)的profit，更大的加入dp，小则不变
        ///
        /// - 如何找出计算dp[n].max_profit
        ///
        ///  由于dp[end_time]保存的是end_time与max profit的关系，如果要找当前n=(start_time, end_time)
        /// 的max_profit，不用重新计算，可用dp[start_time+1]找到这个n之前的max profit.
        /// 实际不用time作为下标时，由于time的增大特性，可用binary search找到time对应下标
        ///
        /// - 如何用binary search找到profit，为何要用start_time而不是start_time+1
        ///
        ///   在job.start_time找dp中的end_time是很直接的，如果当前job.start_time刚好在dp中存在，这个start_time刚好
        /// 完成双可以开始一个job，则表示cur_profit = dp[job.start_time].profit + job.profit。
        /// 如果不存在，则说明要在job.start_time 前找完成的job
        ///
        /// ## submissions
        ///
        /// date=20200615, mem=3.4, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode.com/submissions/detail/353836184/,
        ///
        /// author=lee215, references=https://leetcode.com/problems/maximum-profit-in-job-scheduling/discuss/409009/JavaC%2B%2BPython-DP-Solution
        /// 
        /// author=fengyunzz8, references=https://leetcode.com/problems/maximum-profit-in-job-scheduling/discuss/409009/JavaC++Python-DP-Solution/368208
        ///
        /// ## 复杂度
        ///
        /// - 时间
        ///
        /// unstable sort用quicksort O(N log N)，在构建dp时binary search中共为O(N log N)
        ///
        /// - 空间
        ///
        /// jobs与dp数组为O(N)
        pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
            // create jobs
            let mut jobs: Vec<Job> = Vec::with_capacity(profit.len());
            start_time
                .into_iter()
                .zip(end_time.into_iter())
                .zip(profit.into_iter())
                .for_each(|item| {
                    jobs.push(Job {
                        start_time: (item.0).0,
                        end_time: (item.0).1,
                        profit: item.1,
                    });
                });
            // sort jobs with end_time
            jobs.sort_unstable_by(|a, b| a.end_time.cmp(&b.end_time));
            let mut dp: Vec<Dp> = Vec::new();
            // init dp[0]
            dp.push(Dp {
                end_time: 0,
                profit: 0,
            });
            for job in &jobs {
                let idx = match dp.binary_search_by(|a| a.end_time.cmp(&(job.start_time))) {
                    // found
                    Ok(idx) => idx,
                    // get largest if not found
                    Err(idx) => idx - 1,
                };

                let cur_profit = dp.get(idx).unwrap().profit + job.profit;
                let prev_profit = dp.last().unwrap().profit;
                if cur_profit > prev_profit {
                    dp.push(Dp {
                        end_time: job.end_time,
                        profit: cur_profit,
                    })
                }
            }
            dp.last().unwrap().profit
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let start_time = [1, 2, 3, 4, 6];
            let end_time = [3, 5, 10, 6, 9];
            let profit = [20, 20, 100, 70, 60];
            assert_eq!(
                Solution::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
                150
            );

            let start_time = [1, 2, 3, 3];
            let end_time = [3, 4, 5, 6];
            let profit = [50, 10, 40, 70];
            assert_eq!(
                Solution::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
                120
            );

            let start_time = [1, 1, 1];
            let end_time = [2, 3, 4];
            let profit = [5, 6, 4];
            assert_eq!(
                Solution::job_scheduling(start_time.to_vec(), end_time.to_vec(), profit.to_vec()),
                6
            );
        }
    }
}
