/// 总结
/// 
/// my_solution比solution_simple要繁琐一点，但
/// 思路是一样的，首次能想出来还是不错的
/// 
/// leetcode还有两个解法[移动零](https://leetcode-cn.com/problems/move-zeroes/solution/yi-dong-ling-by-leetcode/)
pub mod my_solution {
    /// # 思路
    /// 
    /// - find cur_idx 0
    /// - 从后找非0移动next到cur_idx上
    /// - 遇到0时跳过
    /// - 最后用next-cur_idx表示有多少0补上
    /// 
    /// ### Submissions
    /// 
    /// date=20200815, mem=2.1, mem_beats=87, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98262973/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut cur_idx = nums.len();
            // find cur_idx
            for i in 0..nums.len() {
                if nums[i] == 0 {
                    cur_idx = i;
                    break;
                }
            }
            let mut next_idx = cur_idx + 1;
            while next_idx < nums.len() {
                // move not 0
                if nums[next_idx] != 0 {
                    nums[cur_idx] = nums[next_idx];
                    cur_idx += 1;
                }
                // skip if is 0
                next_idx += 1;
            }
            // put 0
            for i in cur_idx..nums.len() {
                nums[i] = 0;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let mut nums = vec![0,1,0,3,12];
            Solution::move_zeroes(&mut nums);
            assert_eq!(vec![1,3,12,0,0], nums);

            let mut nums = vec![12];
            Solution::move_zeroes(&mut nums);
            assert_eq!(vec![12], nums);

            let mut nums = vec![1,2];
            Solution::move_zeroes(&mut nums);
            assert_eq!(vec![1,2], nums);
        }
    }
}

pub mod solution_simple {
    /// # 思路
    /// 
    /// 在一次for中移动所有非0的元素到前面，遇到0时skip
    /// 
    /// 参考
    /// 
    /// - [My simple C++ solution](https://leetcode.com/problems/move-zeroes/discuss/72005/My-simple-C%2B%2B-solution)
    /// 
    /// ### Submissions
    /// 
    /// date=20200815, mem=2.2, mem_beats=28.57, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98268704/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut cur = 0;
            for next in 0..nums.len() {
                if nums[next] != 0 {
                    nums[cur] = nums[next];
                    cur += 1;
                }
            }
            for i in cur..nums.len() {
                nums[i] = 0;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let mut nums = vec![0,1,0,3,12];
            Solution::move_zeroes(&mut nums);
            assert_eq!(vec![1,3,12,0,0], nums);

            let mut nums = vec![12];
            Solution::move_zeroes(&mut nums);
            assert_eq!(vec![12], nums);

            let mut nums = vec![1,2];
            Solution::move_zeroes(&mut nums);
            assert_eq!(vec![1,2], nums);
        }
    }
}
