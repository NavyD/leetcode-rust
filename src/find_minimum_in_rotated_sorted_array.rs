//! 要如何找最小值。
//!
//! 使用nums[hi]右边界与mid比较二分

/// 第一次不知道如何与mid比较，如何缩进
///
/// 注意不能使用`if nums[mid] > nums[0] {lo = mid + 1;}`缩进，对于存在旋转点的可以，
/// 但是对于正常的增序数组则不能使用了
pub mod solution_binarysearch {
    /// # 思路
    ///
    /// 使用右边界不断缩进，找到第一个nums[mid] <= nums[hi]的mid。当`nums[mid] > nums[hi]`时说明
    /// [mid,hi]存在旋转点 左缩进，否则表示[lo,mid]存在旋转点右缩进
    ///
    /// 参考：
    ///
    /// * [Beat 100%: Very Simple (Python), Very Detailed Explanation](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/discuss/158940/Beat-100%3A-Very-Simple-(Python)-Very-Detailed-Explanation)
    ///
    /// ### Submissions
    ///
    /// date=20210120, mem=2, mem_beats=40, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139732885/
    ///
    /// date=20210124, mem=2.1, mem_beats=33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140728925/
    ///
    /// date=20210310, mem=2.1, mem_beats=22, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153333847/
    pub struct Solution;

    impl Solution {
        pub fn find_min(nums: Vec<i32>) -> i32 {
            let (mut lo, mut hi) = (0, nums.len() - 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if nums[mid] > nums[hi] {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            nums[lo]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_binarysearch::Solution::find_min);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(f(vec![11, 13, 15, 17]), 11);
    }
}
