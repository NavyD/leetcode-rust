pub mod solution_binarysearch {
    /// # 思路
    ///
    /// 直接使用二分法，需要在比较时判断如何收缩。当target在`[mid + 1,high]`时向后，在`[lo,mid]`时向前，
    /// 下面是用在`[mid + 1,high]`的情况：
    ///
    /// * 0..=mid增序 target在mid后面 两种情况 比mid大在旋转位前 或比0小在旋转后    
    /// * 0..=mid中存在旋转位 target在后面需要>0，且比mid大
    ///
    /// 注意：不能用`nums[0] > nums[mid]`替代`nums[0] > target`，前者在if中走到时是默认成立的，如果
    /// 在当target=nums[0]时nums[0]为旋转位，此时mid比target小，前者会走到`lo=mid+1`导致错误
    ///
    /// 参考：
    ///
    /// - [简洁+容易理解](https://leetcode-cn.com/problems/search-in-rotated-sorted-array/solution/jian-ji-rong-yi-li-jie-java-er-fen-fa-by-breezean/)
    ///
    /// ### Submissions
    ///
    /// date=20210117, mem=2.1, mem_beats=25, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139004026/
    ///
    /// date=20210119, mem=2.1, mem_beats=29, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139512673/
    ///
    /// date=20210309, mem=2, mem_beats=62, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152918436/
    ///
    /// date=20210520, mem=2, mem_beats=70, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/179164611/
    pub struct Solution;

    impl Solution {
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let (mut lo, mut hi) = (0, nums.len() - 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                // 0..=mid增序 target在mid后面 两种情况 比mid大在旋转位前 或比0小在旋转后
                if nums[0] <= nums[mid] && (target > nums[mid] || target < nums[0])
                    // 0..=mid中存在旋转位 target在后面需要 < 0，且比mid大
                    || (nums[0] > target && nums[mid] < target)
                {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            // 判断最后一个元素
            if nums[lo] == target {
                lo as i32
            } else {
                -1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_binarysearch::Solution::search);
    }

    fn test<F: Fn(Vec<i32>, i32) -> i32>(f: F) {
        assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(f(vec![1], 0), -1);
        assert_eq!(f(vec![1, 3], 3), 1);
        assert_eq!(f(vec![5, 1, 3], 5), 0);
        assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], 1), 5);
    }
}
