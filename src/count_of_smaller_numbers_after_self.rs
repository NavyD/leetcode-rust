//! You are given an integer array nums and you have to return a new counts array. The counts array has the property where counts[i] is the number of smaller elements to the right of nums[i].
//!
//! Example:
//!
//! ```ignore
//! Input: [5,2,6,1]
//! Output: [2,1,1,0]
//! Explanation:
//! To the right of 5 there are 2 smaller elements (2 and 1).
//! To the right of 2 there is only 1 smaller element (1).
//! To the right of 6 there is 1 smaller element (1).
//! To the right of 1 there is 0 smaller element.
//! ```
pub struct SolutionByViolent;

impl SolutionByViolent {
    /// # 思路
    ///
    /// 用双重for对`i<j && nums[i] > nums[j]`计算counts[i]，暴力破解
    ///
    /// ## Submissions
    ///
    /// date=2020519, mem=2.3, mem_beats=100, runtime=828, runtime_beats=5.09, url=https://leetcode.com/submissions/detail/341486647/
    ///
    /// author=navyd
    ///
    /// ## 复杂度
    ///
    /// - 时间：双重for为O(N^2)
    /// - 空间：返回结果count为O(N)
    ///
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut counts = vec![0; n];
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] > nums[j] {
                    counts[i] += 1;
                }
            }
        }
        counts
    }
}

pub struct SolutionByMergeSort;

impl SolutionByMergeSort {
    /// # 思路
    ///
    /// 如果两个数组已有序，合并时right移动到left前面即是left当前元素右边小的元素数量。
    /// 利用merge sort可完成计数
    ///
    /// ## 问题
    ///
    /// - 如何排序对nums
    ///
    /// 直接对nums归并会导致nums相对位置发生改变，我们需要用nums的位置计数，也不能用
    /// copy and find的方式复制nums,对重复元素无法用。
    ///
    /// 如果只对nums的index归并排序，即用index可以排序nums并能保持原来的nums：
    /// `nums[indexes[i]]`多了一个indexes映射层
    ///
    /// ```ignore
    /// nums=[5,2,6,1]
    /// indexes=[0,1,2,3]
    ///
    /// // 排序后
    /// indexes=[3,1,0,2]
    /// nums[indexes[0]]=1
    /// nums[indexes[1]]=2
    /// nums[indexes[2]]=5
    /// nums[indexes[3]]=6
    /// ```
    ///
    /// - 如何在归并中对left计数
    ///
    /// 用indexes映射排序，操作的是index，比较的是nums[index]。当right移动时，对right移动计数为count，
    /// 当left移动时，对count[indexes[left]] += count，保证counts[i]之前的统计和left后面的也能+count，
    /// 不至于count清0
    ///
    /// ```ignore
    /// nums=[5,2,6,1]
    /// indexes=[0,1,2,3]
    ///
    /// round0: indexes=[0] | indexes=[1]
    /// loop0:  left=nums[indexes[0]] 5 > right=nums[indexes[1]] 2
    ///         // right move
    ///         count = 1
    ///         right++
    /// loop1:  // left move
    ///         counts[indexes[0]=0]+=count=1
    ///
    /// round1: indexes=[2] | indexes=[3]
    /// loop0:  left=nums[indexes[2]=2] 6 > right=nums[indexes[3]=3] 1
    ///         // right move
    ///         count = 1
    ///         right++
    /// loop1:  // left move
    ///         counts[indexes[2]=2]+=count=1
    ///
    /// round2: indexes=[1,0] | indexes=[3,2] => [2,5] | [1,6]
    /// loop0:  left=nums[indexes[0]=1] 2 > right=nums[indexes[2]=3] 1
    ///         // right move
    ///         count = 1
    ///         right++
    /// loop1: left=nums[left=indexes[0]=1] 2 < right=nums[indexes[3]=2] 6
    ///         // left move, next loop
    ///         counts[indexes[0]=1] += count=1
    ///         left++
    /// loop2:  left=nums[indexes[1]=0] 5 < right=nums[indexes[3]=2] 6
    ///         // counts[0]=2  [5,2]=>[2,5], [2,5]=>[1,25] 两次
    ///         counts[indexes[1]=0] += count=1 2
    ///         left++
    /// loop3:  right move
    ///
    /// counts=[2,1,1,0]
    /// ```
    /// 
    /// - 为何是逆序 indexes
    /// 
    /// 使用`aux_indexes[index] = right`导致在个别测试用例中存在逆序情况，可能在4个元素以下的数组都
    /// 不能由于直接用下标导致逆序。可以用工`aux_indexes[index] = indexes[right]`修复
    /// 
    /// ```rust,ignore
    /// for left in lo..=mid {
    ///     while right <= hi && nums[indexes[left]] > nums[indexes[right]] {
    ///         // aux_indexes[index] = right;
    ///         aux_indexes[index] = indexes[right];
    ///         count += 1;
    ///         right += 1;
    ///         index += 1;
    ///     }
    ///     counts[indexes[left]] += count;
    ///     // aux_indexes[index] = right;
    ///     aux_indexes[index] = indexes[left];
    ///     index += 1;
    /// }
    /// 
    /// while right <= hi {
    ///     // aux_indexes[index] = right;
    ///     aux_indexes[index] = indexes[right];
    ///     index += 1;
    ///     right += 1;
    /// }
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200521, mem=2.5, mem_beats=100, runtime=8, runtime_beats=57.38, url=https://leetcode.com/submissions/detail/342473041/
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：归并排序最大O(N log N)
    /// - 空间：counts, indexes, aux_indexes都是O(N)
    /// 
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        // create indexes, counts
        let n = nums.len();
        let (mut indexes, mut aux_indexes, mut counts) = (vec![0; n], vec![0; n], vec![0; n]);
        if n == 0 {
            return counts;
        }
        for i in 0..n {
            indexes[i] = i;
        }
        // merge sort for indexes
        Self::merge_sort(&nums, &mut indexes, &mut aux_indexes, &mut counts, 0, n - 1);
        counts
    }

    fn merge_sort(
        nums: &Vec<i32>,
        indexes: &mut Vec<usize>,
        aux_indexes: &mut Vec<usize>,
        counts: &mut Vec<i32>,
        lo: usize,
        hi: usize,
    ) {
        if lo >= hi {
            return;
        }
        let mid = lo + (hi - lo) / 2;
        Self::merge_sort(nums, indexes, aux_indexes, counts, lo, mid);
        Self::merge_sort(nums, indexes, aux_indexes, counts, mid + 1, hi);
        Self::merge(nums, indexes, aux_indexes, counts, lo, mid, hi);
    }

    fn merge(
        nums: &Vec<i32>,
        indexes: &mut Vec<usize>,
        aux_indexes: &mut Vec<usize>,
        counts: &mut Vec<i32>,
        lo: usize,
        mid: usize,
        hi: usize,
    ) {
        let (mut index, mut count, mut right) = (lo, 0, mid + 1);
        for left in lo..=mid {
            while right <= hi && nums[indexes[left]] > nums[indexes[right]] {
                aux_indexes[index] = indexes[right];
                count += 1;
                right += 1;
                index += 1;
            }
            counts[indexes[left]] += count;
            aux_indexes[index] = indexes[left];
            index += 1;
        }

        while right <= hi {
            aux_indexes[index] = indexes[right];
            index += 1;
            right += 1;
        }

        for i in lo..=hi {
            indexes[i] = aux_indexes[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_by_violent() {
        test_data(SolutionByViolent::count_smaller);
    }

    #[test]
    fn test_solution_by_merge_sort() {
        test_data(SolutionByMergeSort::count_smaller);
    }

    fn test_data<F: Fn(Vec<i32>) -> Vec<i32>>(f: F) {
        assert_eq!(f(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(f(vec![2, 0, 1]), vec![2, 0, 0]);
        assert_eq!(f(vec![]), vec![]);
        assert_eq!(
            f(vec![
                26, 78, 27, 100, 33, 67, 90, 23, 66, 5, 38, 7, 35, 23, 52, 22, 83, 51, 98, 69, 81,
                32, 78, 28, 94, 13, 2, 97, 3, 76, 99, 51, 9, 21, 84, 66, 65, 36, 100, 41
            ],),
            vec![
                10, 27, 10, 35, 12, 22, 28, 8, 19, 2, 12, 2, 9, 6, 12, 5, 17, 9, 19, 12, 14, 6, 12,
                5, 12, 3, 0, 10, 0, 7, 8, 4, 0, 0, 4, 3, 2, 0, 1, 0
            ]
        );
    }
}
