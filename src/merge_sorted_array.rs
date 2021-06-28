/// 总结
///
/// 正常用的是solution_forward解法，归并排序就是这种，
/// solution_backward没有nums1的额外空间也不可能放下nums2
///
/// 20200903
///
/// 注意backward解法的nums1不需要对m检查是否有m>=1
pub mod solution_forward {
    /// # 思路
    ///
    /// 双指针向前合并到临时数组中
    ///
    /// ### Submissions
    ///
    /// date=20200902, mem=2.1, mem_beats=0, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103891745/
    ///
    /// date=20200903, mem=2.1, mem_beats=7.69, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104340610/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n+m)
    /// - 空间：O(n+m)
    pub struct Solution;

    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let (m, n) = (m as usize, n as usize);
            let mut temp = Vec::with_capacity(m + n);
            let (mut i, mut j) = (0, 0);
            while i < m && j < n {
                temp.push(if nums1[i] < nums2[j] {
                    let val = nums1[i];
                    i += 1;
                    val
                } else {
                    let val = nums2[j];
                    j += 1;
                    val
                });
            }

            while i < m {
                temp.push(nums1[i]);
                i += 1;
            }
            while j < n {
                temp.push(nums2[j]);
                j += 1;
            }
            std::mem::swap(nums1, &mut temp);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let (mut nums1, mut nums2) = (vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]);
            let (m, n) = (3, 3);
            // nums1.len() >= m+n
            for i in nums1.len() - 1..m + n {
                nums1[i] = 0;
            }
            Solution::merge(&mut nums1, m as i32, &mut nums2, n as i32);
            assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
        }
    }
}

pub mod solution_backward {
    /// # 思路
    ///
    /// 在从头改变nums1的值时，需要把nums1中的元素存放在其他位置。
    ///
    /// 从结尾开始改写 nums1 的值 `i = m+n-1`，由于`nums1.len >= m+n`确保nums1可push nums2
    /// 所有值，对nums1 m个值也只要移动就不需要额外空间
    ///
    /// 注意当nums1 m个值先排完即都大于nums2前面某些值，nums2剩下的要放到nums1中
    /// `nums1[i]=nums2[n]`
    ///
    /// 如果nums2 n都大于nums1，则nums2先移到nums1后面n个，不对前m个影响，也就是说
    /// 只要nums2 n个排到了nums1中就完成了`while n >= 1 {`
    ///
    /// rust的数组下标是usize类型，在i-=1时可能有判断<0出现usize<0运行错误，
    /// 可在`while m >= 1 { m -= 1; nums[m-1]}`访问
    ///
    /// 参考：
    ///
    /// - [合并两个有序数组](https://leetcode-cn.com/problems/merge-sorted-array/solution/he-bing-liang-ge-you-xu-shu-zu-by-leetcode/)
    ///
    /// ### Submissions
    ///
    /// date=20200902, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103908664/
    ///
    /// date=20200903, mem=2.1, mem_beats=23.08, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104338847/
    ///
    /// date=202009012, mem=2, mem_beats=57.14, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/107179774/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n + m)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let (mut m, mut n) = (m as usize, n as usize);
            let mut i = m + n;
            while m >= 1 && n >= 1 {
                i -= 1;
                nums1[i] = if nums1[m - 1] < nums2[n - 1] {
                    n -= 1;
                    nums2[n]
                } else {
                    m -= 1;
                    nums1[m]
                };
            }
            while n >= 1 {
                i -= 1;
                n -= 1;
                nums1[i] = nums2[n];
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let (mut nums1, mut nums2) = (vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]);
            let (m, n) = (3, 3);
            // nums1.len() >= m+n
            for i in nums1.len() - 1..m + n {
                nums1[i] = 0;
            }
            Solution::merge(&mut nums1, m as i32, &mut nums2, n as i32);
            assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
        }
    }
}

pub mod solution_sort {
    /// # 思路
    ///
    /// 合并后排序
    ///
    /// - [合并两个有序数组](https://leetcode-cn.com/problems/merge-sorted-array/solution/he-bing-liang-ge-you-xu-shu-zu-by-leetcode/)
    ///
    /// ### Submissions
    ///
    /// date=20200902, mem=2.1, mem_beats=25, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103923302/
    ///
    /// date=20200903, mem=2.1, mem_beats=7.69, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104336208/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n log n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let (mut m, n) = (m as usize, n as usize);
            for i in 0..n {
                nums1[m] = nums2[i];
                m += 1;
            }
            nums1.sort_unstable();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let (mut nums1, mut nums2) = (vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]);
            let (m, n) = (3, 3);
            // nums1.len() >= m+n
            for i in nums1.len() - 1..m + n {
                nums1[i] = 0;
            }
            Solution::merge(&mut nums1, m as i32, &mut nums2, n as i32);
            assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
        }
    }
}
