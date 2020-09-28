pub mod solution_heap {
    /// # 思路
    ///
    /// 堆排序
    ///
    /// 在java有使用non-leaf half作为while条件，移动child减少swap，但是暂时没找到方法
    ///
    /// ```java
    /// private static void siftDown(int[] a, int start, int end) {
    ///   int half = (end - start + 1) / 2 + start;
    ///   int val = a[start];
    ///   while (start < half) {
    ///     int child = 2 * start + 1, rightChild = child + 1;
    ///     if (rightChild <= end && a[rightChild] > a[child]) {
    ///       child = rightChild;
    ///     }
    ///     if (a[child] <= val) {
    ///       break;
    ///     }
    ///     a[start] = a[child];
    ///     start = child;
    ///   }
    ///   a[start] = val;
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20200928, mem=2.5, mem_beats=100, runtime=12, runtime_beats=62.5, url=https://leetcode-cn.com/submissions/detail/112075740/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n log n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
            let len = nums.len();
            if len <= 1 {
                return nums;
            }
            // non-leaf nodes from len/2-1
            for i in (0..=len / 2 - 1).rev() {
                Self::sift_down(&mut nums, i, len - 1);
            }
            for i in (1..len).rev() {
                // max to back
                Self::swap(&mut nums, 0, i);
                Self::sift_down(&mut nums, 0, i - 1);
            }
            nums
        }

        fn sift_down(nums: &mut Vec<i32>, mut k: usize, end: usize) {
            loop {
                let mut child = 2 * k + 1;
                if child > end {
                    break;
                }
                let right_child = child + 1;
                if right_child <= end && nums[right_child] > nums[child] {
                    child = right_child;
                }
                if nums[child] <= nums[k] {
                    break;
                }
                Self::swap(nums, k, child);
                k = child;
            }
        }

        fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
            let temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_heap::Solution::sort_array);
    }

    fn test<F: Fn(Vec<i32>) -> Vec<i32>>(func: F) {
        assert_eq!(func(vec![0]), vec![0]);
        assert_eq!(func(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            func(vec![
                -74, 48, -20, 2, 10, -84, -5, -9, 11, -24, -91, 2, -71, 64, 63, 80, 28, -30, -58,
                -11, -44, -87, -22, 54, -74, -10, -55, -28, -46, 29, 10, 50, -72, 34, 26, 25, 8,
                51, 13, 30, 35, -8, 50, 65, -6, 16, -2, 21, -78, 35, -13, 14, 23, -3, 26, -90, 86,
                25, -56, 91, -13, 92, -25, 37, 57, -20, -69, 98, 95, 45, 47, 29, 86, -28, 73, -44,
                -46, 65, -84, -96, -24, -12, 72, -68, 93, 57, 92, 52, -45, -2, 85, -63, 56, 55, 12,
                -85, 77, -39,
            ]),
            vec![
                -96, -91, -90, -87, -85, -84, -84, -78, -74, -74, -72, -71, -69, -68, -63, -58,
                -56, -55, -46, -46, -45, -44, -44, -39, -30, -28, -28, -25, -24, -24, -22, -20,
                -20, -13, -13, -12, -11, -10, -9, -8, -6, -5, -3, -2, -2, 2, 2, 8, 10, 10, 11, 12,
                13, 14, 16, 21, 23, 25, 25, 26, 26, 28, 29, 29, 30, 34, 35, 35, 37, 45, 47, 48, 50,
                50, 51, 52, 54, 55, 56, 57, 57, 63, 64, 65, 65, 72, 73, 77, 80, 85, 86, 86, 91, 92,
                92, 93, 95, 98
            ]
        );
    }
}
