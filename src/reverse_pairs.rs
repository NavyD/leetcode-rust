//! Given an array nums, we call (i, j) an important reverse pair if i < j and nums[i] > 2*nums[j].
//!
//! You need to return the number of important reverse pairs in the given array.
//!
//! Example1:
//!
//! ```ignore
//! Input: [1,3,2,3,1]
//! Output: 2
//! ```
//!
//! Example2:
//!
//! ```ignore
//! Input: [2,4,3,5,1]
//! Output: 3
//! ```
//!
//! Note:
//!
//! 1. The length of the given array will not exceed 50,000.
//! 2. All the numbers in the input array are in the range of 32-bit integer.
//!

pub struct SolutionByViolent;

impl SolutionByViolent {
    /// # 思路
    ///
    /// 直接双重for对每个i判断j有`nums[i] as i64 > 2 * nums[j] as i64`
    ///
    /// Time Limit Exceeded
    ///
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] as i64 > 2 * nums[j] as i64 {
                    count += 1;
                }
            }
        }
        count
    }
}

pub struct SolutionByMergeSort;

impl SolutionByMergeSort {
    /// # 思路
    ///
    /// 对nums indexes排序merge时统计nums[i]>2*nums[j]数量
    /// 
    /// # Submissions
    /// 
    /// date=20200522, mem=3.2, mem_beats=100, runtime=96, runtime_beats=50, url=https://leetcode.com/submissions/detail/342936543/
    /// 
    /// author=navyd
    ///
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let (mut indexes, mut aux_indexes) = (vec![0; n], vec![0; n]);
        for i in 0..n {
            indexes[i] = i;
        }
        Self::merge_sort(&nums, &mut indexes, &mut aux_indexes, 0, n - 1) as i32
    }

    fn merge_sort(
        nums: &Vec<i32>,
        indexes: &mut Vec<usize>,
        aux_indexes: &mut Vec<usize>,
        lo: usize,
        hi: usize,
    ) -> usize {
        if lo >= hi {
            return 0;
        }
        let mid = lo + (hi - lo) / 2;
        let mut count = 0;
        count += Self::merge_sort(nums, indexes, aux_indexes, lo, mid);
        count += Self::merge_sort(nums, indexes, aux_indexes, mid + 1, hi);
        count += Self::merge(nums, indexes, aux_indexes, lo, mid, hi);
        count
    }

    fn merge(
        nums: &Vec<i32>,
        indexes: &mut Vec<usize>,
        aux_indexes: &mut Vec<usize>,
        lo: usize,
        mid: usize,
        hi: usize,
    ) -> usize {
        // count = idx_upper-mid-1
        let (mut count, mut idx_upper, mut index, mut right) = (0, mid + 1, lo, mid + 1);
        for left in lo..=mid {
            // get first index with left > 2*right
            while idx_upper <= hi
                && nums[indexes[left]] as i64 > 2 * nums[indexes[idx_upper]] as i64
            {
                idx_upper += 1;
            }
            // move right
            while right <= hi && nums[indexes[left]] > nums[indexes[right]] {
                aux_indexes[index] = indexes[right];
                index += 1;
                right += 1;
            }
            // move left
            aux_indexes[index] = indexes[left];
            index += 1;
            // offset 小的left>idx_upper, 大的left在这个基础上
            count += idx_upper - mid - 1;
        }
        // move left for right
        while right <= hi {
            aux_indexes[index] = indexes[right];
            right += 1;
            index += 1;
        }

        for i in lo..=hi {
            indexes[i] = aux_indexes[i];
        }
        count
    }
}

pub struct SolutionByBIT;

impl SolutionByBIT {
    ///
    /// # 思路
    /// 
    /// 用binary indexed tree解决
    /// 
    /// ## 问题
    /// 
    /// - 如何工作
    /// 
    ///   将BIT反过来处理。对于i<j先将i放入BIT，再对下一个j在前面找i是否存在`nums[i] >= 2 * nums[j]+1`。
    /// 
    ///   BIT中存储着离散化后nums的关系，只要用离散化后`2* nums[j]+1`在BIT中找到对应位置i，就可以快速统计出
    /// 当前存在的BIT中j-1个离散化的有多少满足
    /// 
    /// - 如何用nums[]处理BIT数据
    /// 
    ///   这个BIT类型一般不关心数据nums的值，而是对nums数据的关系敏感。我们可以用有序的nums使离散化，
    /// bits中保存的就是每个nums的位置i与数量val: `bits[i]=val`
    /// 
    /// - 如何处理nums的等值情况：bits中必须处理到相等值`bits[i]=val > 1`。
    /// 
    ///   在对nums离散化时，怎样使update bits val>1。当对BIT初始时会遍历nums，这就包含着重复值，
    /// 只要对重复的num离散化同一个值i，update时就在原来基础上+1
    /// 
    /// - 如何离散化nums并构造BIT
    /// 
    ///   对排序后的nums用binary search离散: index()。在遍历nums时先查询`index(2 * num + 1)`在BIT中
    /// 的数量`get_sum`，`index(num)`再update BIT
    /// 
    /// - 为何index()中用`lo+1`, `if mid == 0`
    /// 
    ///   由于bits.len = nums.len+1，如果直接用binary search中的下标mid返回，将无法对应上bits，则在原下标上+1。
    ///   
    ///   当mid=0时，`hi=mid-1`对于usize类型可导致overflow panic
    /// 
    /// ## Submissions
    /// 
    /// date=20200523, mem=2.9, mem_beats=100, runtime=64, runtime_beats=66.67, url=https://leetcode.com/submissions/detail/343406129/
    /// 
    /// author=fun4LeetCode, references=https://leetcode.com/problems/reverse-pairs/discuss/97268/General-principles-behind-problems-similar-to-%22Reverse-Pairs%22
    /// 
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        // bits
        let (mut bits, mut sorted_nums) = (vec![0; n + 1], nums.clone());
        sorted_nums.sort_unstable();
        // 离散化 sorted_nums
        let mut count = 0;
        for num in nums {
            // 查询当前num的2*num+1的数量
            count += Self::get_sum(&bits, Self::index(&sorted_nums, 2 * num as i64 + 1));
            // update BIT用num
            Self::increase(&mut bits, Self::index(&sorted_nums, num as i64));
        }
        count as i32
    }

    fn index(sorted_nums: &Vec<i32>, val: i64) -> usize {
        let (mut lo, mut hi) = (0, sorted_nums.len() - 1);
        while lo <= hi {
            let mid = (hi + lo) / 2;
            let num = sorted_nums[mid] as i64;
            if num < val {
                lo = mid + 1;
            } else if num > val {
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            } else {
                lo = mid;
                break;
            }
        }
        lo + 1
    }

    fn get_sum(bits: &Vec<usize>, index: usize) -> usize {
        let mut sum = 0;
        let mut index = index;
        while index < bits.len() {
            sum += bits[index];
            index += Self::lowbit(index);
        }
        sum
    }

    fn increase(bits: &mut Vec<usize>, index: usize) {
        let mut index = index;
        while index > 0 {
            bits[index] += 1;
            index -= Self::lowbit(index);
        }
    }

    fn lowbit(index: usize) -> usize {
        let i = index as isize;
        (i & -i) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data<T: Fn(Vec<i32>) -> i32>(f: T) {
        assert_eq!(f(vec![]), 0);
        assert_eq!(f(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(f(vec![2, 4, 3, 5, 1]), 3);
        assert_eq!(f(vec![-5, -5]), 1);
        assert_eq!(
            f(vec![
                2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
            ]),
            0
        );
        assert_eq!(f(vec![5,4,3,2,1]), 4);
    }

    #[test]
    fn solution_by_violent() {
        test_data(SolutionByViolent::reverse_pairs);
    }

    #[test]
    fn solution_by_merge_sort() {
        test_data(SolutionByMergeSort::reverse_pairs);
    }

    #[test]
    fn solution_by_bit() {
        test_data(SolutionByBIT::reverse_pairs);
    }
}
