//! Given an integer array nums, return the number of range sums that lie in [lower, upper] inclusive.
//! Range sum S(i, j) is defined as the sum of the elements in nums between indices i and j (i ≤ j), inclusive.
//! Note:
//! A naive algorithm of O(n2) is trivial. You MUST do better than that.
//!
//! Example
//!
//! ```ignore
//! Input: nums = [-2,5,-1], lower = -2, upper = 2,
//! Output: 3
//! Explanation: The three ranges are : [0,0], [2,2], [0,2] and their respective sums are: -2, -1, 2.
//! ```

/// # 思路
///
/// 利用暴力求解时SolutionByViolentSum中的sums，由于题目要求`lower <= S[j]-S[i] <= upper`的数
/// 量count，可以用merge sort对sums排序时统计sums[j]-sums[i]。
///
/// 这个题本质和[Count of Smaller Numbers After Self](https://leetcode.com/problems/count-of-smaller-numbers-after-self/)
/// 一样，都是求解逆序对的数量。
///
/// ## 问题
///
/// - 如何统计`lower <= sums[j]-sums[i] <= upper`的数量？
///
/// 由于归并是有序的，对每个sums[left],找sums[right]中第1个满足`sums[right]-sums[left]<=lower`的
/// 下标idx_lower，和满足`sums[right]-sums[left]>upper`的下标idx_upper，count=idx_upper-idx_lower
///
/// ```rust,ignore
/// for left in lo..mid + 1 {
///     // find min lower index
///     while idx_lower <= hi && sums[idx_lower] - sums[left] < lower {
///         idx_lower += 1;
///     }
///     // find max uppper index
///     while idx_upper <= hi && sums[idx_upper] - sums[left] <= upper {
///         idx_upper += 1;
///     }
///     count += idx_upper - idx_lower;
/// }
/// ```
/// - 为何是right-left, 如何保证sums[j]-sums[i]不会多或少算？
///
/// 由于归并排序的性质递归的保证了`sums[j]-sums[i]`只会算一次，由于计算后的j-i都会
/// 到下次归并的left or right一个组，一个组不会再被分开归并
///
/// ```ignore
/// nums=[-2,5,-1], lower = -2, upper = 2
/// sums=[0,-2,3,2]
/// s[0..=1]: s[1]-s[0]
/// s[2..=3]: s[3]-s[2]
/// s[0..=3]: s[2]-s[0]
///         : s[3]-s[0]
///         : s[2]-s[1]
///         : s[3]-s[1]
/// ```
///
/// # submissions
///
/// date=20200517, mem=2.3, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode.com/submissions/detail/340509441/
///
/// author=dietpepsi, references=https://leetcode.com/problems/count-of-range-sum/discuss/77990/Share-my-solution
///
/// # 复杂度分析
///
/// 设nums长度为N
///
/// 时间：merge sort为O(N log N)，虽然merge中有双重循环，但是类似于double pointer，总复杂度
/// 还是线性的
///
/// 空间：merger sort需要两个sums,aux_sums数组排序用O(N)，返回一个count递归用空间是O(log N)，
/// 即总空间是O(N)
///
pub struct SolutionByMergeSortSum;

impl SolutionByMergeSortSum {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // caculate sums of all S(i+1) = n0+..+ni
        let n = nums.len();
        let mut sums = vec![0; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i] as i64;
        }
        let mut aux_sums = vec![0; n + 1];
        Self::merge_sort(lower as i64, upper as i64, &mut sums, 0, n, &mut aux_sums) as i32
    }

    fn merge_sort(
        lower: i64,
        upper: i64,
        sums: &mut Vec<i64>,
        lo: usize,
        hi: usize,
        aux_sums: &mut Vec<i64>,
    ) -> usize {
        if lo >= hi {
            return 0;
        }
        let mid = lo + (hi - lo) / 2;
        let mut count = 0;
        count += Self::merge_sort(lower, upper, sums, lo, mid, aux_sums);
        count += Self::merge_sort(lower, upper, sums, mid + 1, hi, aux_sums);
        count += Self::merge(lower, upper, sums, lo, mid, hi, aux_sums);
        count
    }

    fn merge(
        lower: i64,
        upper: i64,
        sums: &mut Vec<i64>,
        lo: usize,
        mid: usize,
        hi: usize,
        aux_sums: &mut Vec<i64>,
    ) -> usize {
        let (mut right, mut index, mut idx_upper, mut idx_lower) = (mid + 1, lo, mid + 1, mid + 1);
        let mut count = 0;
        // left as outer loop
        for left in lo..mid + 1 {
            // find min lower index
            while idx_lower <= hi && sums[idx_lower] - sums[left] < lower {
                idx_lower += 1;
            }
            // find max uppper index
            while idx_upper <= hi && sums[idx_upper] - sums[left] <= upper {
                idx_upper += 1;
            }
            // merge right to aux
            while right <= hi && sums[right] < sums[left] {
                aux_sums[index] = sums[right];
                index += 1;
                right += 1;
            }
            aux_sums[index] = sums[left];
            index += 1;
            // get count by upper index - lower index
            count += idx_upper - idx_lower;
        }
        while right <= hi {
            aux_sums[index] = sums[right];
            right += 1;
            index += 1;
        }
        sums[lo..(hi + 1)].clone_from_slice(&aux_sums[lo..(hi + 1)]);
        count
    }
}

/// # 思路
///
/// 题目要求在[i,j]内num之和sum[i,j]满足`lower <= sum[i,j] <= upper`，即每个i与
/// 后面所有元素之间的和sum[i,j]都可能满足条件。
///
/// 用sums数组保存每个num的和，避免重复计算。sums[j]-sums[i]表示sum[i,j]的和。
///
/// ## 问题
///
/// - 如何解决`sum[i,i]=sums[i]-sums[i]=0`的问题？
///
/// 在迭代时用`sum[i,i]=sums[i+1]-sums[i] = nums[i]`，就是nums[i]这个元素，
/// 如果用`sums[0] = 0, sums[0,0] = sums[1] - sums[0] = nums[0]`，保证nums[i]
/// 被包括在结果中
///
/// ## Examples
///
/// ```ignore
/// [-2,5,-1], lower = -2, upper = 2
/// sums[0] = nums[0]                   = -2
/// sums[1] = nums[1] + sums[0] = 5-2   = 3
/// sums[2] = nums[2] + sums[1] = 3-1   = 2
/// // 如果sum[i,j] = sums[j] - sums[j]
/// sum[0,0]    = sums[0]-sums[0] = 0
/// sum[0,2]    = sums[2]-sums[0] = 2-(-2) = 4
/// sum[2,2]    = sums[2]-sums[2] = 0
///
/// // 如果sum[i,j] = sums[j+1] - sums[i], sums[0] = 0, sums[1] = nums[0]+sums[0]
/// sum[0,0]    = sums[1]-sums[0] = -2
/// sum[0,2]    = sums[3]-sums[0] = 2-0 = 2
/// ```
/// ## Submissions
///
/// date=20200515, mem=2.2, mem_beats=100, runtime=140, runtime_beats=40, url=https://leetcode.com/submissions/detail/339532332/
///
/// author=dietpepsi, references=https://leetcode.com/problems/count-of-range-sum/discuss/77990/Share-my-solution
///
/// ## 复杂度
///
/// - 时间：对sums双重for是O(N^2)
/// - 空间：sums用的是O(N)
///
pub struct SolutionByViolentSum;

impl SolutionByViolentSum {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // get sums of all
        let (lower, upper, n) = (lower as i64, upper as i64, nums.len());
        let mut sums: Vec<i64> = vec![0; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i] as i64;
        }
        // compare
        let mut count = 0;
        for i in 0..n {
            for j in i + 1..n + 1 {
                let sum = sums[j] - sums[i];
                if sum >= lower && sum <= upper {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution_by_merge_sort_sum() {
        test(SolutionByMergeSortSum::count_range_sum);
    }

    #[test]
    fn test_solution_by_violent_sum() {
        test(SolutionByViolentSum::count_range_sum);
    }

    fn test<T>(f: T)
    where
        T: FnOnce(Vec<i32>, i32, i32) -> i32,
    {
        let (nums, lower, upper) = (vec![-2, 5, -1], -2, 2);
        let count = f(nums, lower, upper);
        assert_eq!(count, 3);
    }
}
