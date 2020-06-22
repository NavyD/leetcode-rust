//! Given an integer array nums, find the sum of the elements between indices i and j (i ≤ j), inclusive.
//! 
//! Example:
//! Given nums = [-2, 0, 3, -5, 2, -1]
//! 
//! sumRange(0, 2) -> 1
//! sumRange(2, 5) -> -1
//! sumRange(0, 5) -> -3
//! Note:
//! You may assume that the array does not change.
//! There are many calls to sumRange function.

#![allow(dead_code)]
pub mod solution_dp {
    /// # 思路
    /// 
    /// 用sums[n]表示nums[0..=n]的和，取范围时可用sum_range(i, j) => sums[j + 1] - sums[i]，
    /// 注意在sums是n+1个，保证range(0, 0)时有sums[1]-sums[0]=nums[0]
    /// 
    /// `range(i, j) = sums[j + 1] - sums[i]`
    /// 
    /// ## Submissions
    /// 
    /// date=20200622, mem=7.7, mem_beats=20.51, runtime=4, runtime_beats=95.35, url=https://leetcode.com/submissions/detail/356787183/,
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    struct NumArray {
        sums: Vec<i32>,
    }
    
    impl NumArray {
    
        fn new(nums: Vec<i32>) -> Self {
            let n = nums.len();
            let mut sums = vec![0; n + 1];
            for i in 1..=n {
                sums[i] = sums[i - 1] + nums[i - 1];
            }
            NumArray { sums }
        }
        
        fn sum_range(&self, i: i32, j: i32) -> i32 {
            self.sums[j as usize + 1] - self.sums[i as usize]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let na = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
            assert_eq!(na.sum_range(0, 2), 1);
            assert_eq!(na.sum_range(2, 5), -1);
            assert_eq!(na.sum_range(0, 5), -3);
            assert_eq!(na.sum_range(5, 5), -1);
        }
    }
}

pub mod solution_dp_less {
    /// # 思路
    /// 
    /// 由于`range(i, j) = sums[j] - sums[i - 1]`，可用nums作为sums减少空间使用，
    /// 当i==0时直接返加sums[j]而不是sums[j+1] - sums[0]
    /// 
    /// ## Submissions
    /// 
    /// date=20200622, mem=7.6, mem_beats=71.79, runtime=4, runtime_beats=95.35, url=https://leetcode.com/submissions/detail/356790921/,
    /// 
    /// author=0, url=https://leetcode.com/submissions/api/detail/303/rust/0/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    struct NumArray {
        sums: Vec<i32>,
    }
    
    impl NumArray {
    
        fn new(nums: Vec<i32>) -> Self {
            let mut nums = nums;
            let mut sum = 0;
            for num in &mut nums {
                sum += *num;
                *num = sum;
            }
            NumArray { sums: nums }
        }
        
        fn sum_range(&self, i: i32, j: i32) -> i32 {
            if i == 0 {
                self.sums[j as usize]
            } else {
                self.sums[j as usize] - self.sums[i as usize - 1]
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let na = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
            assert_eq!(na.sum_range(0, 2), 1);
            assert_eq!(na.sum_range(2, 5), -1);
            assert_eq!(na.sum_range(0, 5), -3);
            assert_eq!(na.sum_range(5, 5), -1);
        }
    }
}