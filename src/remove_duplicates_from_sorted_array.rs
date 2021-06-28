/// 总结
///
/// 20200907
///
/// 双指针法运用还不熟练，想了半天，写了个bug
/// ```rust,ignore
/// if nums[i + 1] != nums[next] {
///     i += 1;
///     nums[i] = nums[next];
/// }
/// ```
///
/// 这是nums[i] == nums[j]没分析清楚
pub mod solution_double_pointer {
    /// # 思路
    ///
    /// 数组是有序的，重复的元素一定会相邻，双指针将不重复的元素移到数组的左侧。
    ///
    /// - 放置两个指针 i 和 j，其中 i 是慢指针，而 j 是快指针。只要 nums[i]=nums[j]，我们就增加 jj 以跳过重复项。
    /// - 当我们遇到 nums[j] != nums[i]时，跳过重复项的运行已经结束，因此我们必须把它nums[j]的值复制到 nums[i+1]。然后递增 i
    ///
    /// ```java
    /// @Submission(date = "20200830", memory = 41.7, memoryBeatRate = 30, runtime = 1, runtimeBeatRate = 94, url = "https://leetcode-cn.com/submissions/detail/102996457/")
    /// public int removeDuplicates(int[] nums) {
    ///     int i = 0;
    ///     for (int j = 1; j < nums.length; j++) {
    ///         if (nums[i] != nums[j]) {
    ///             nums[++i] = nums[j];
    ///         }
    ///     }
    ///     return i + 1;
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [【双指针】删除重复项-带优化思路](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/solution/shuang-zhi-zhen-shan-chu-zhong-fu-xiang-dai-you-hu/)
    ///
    /// ***不明白为何rust是4ms时间，而java是1ms，而且rust有0ms的方法不明白有什么不一样***
    ///
    /// rust std有个和这个相同功能的API：[Vec::dedup](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup)，
    /// 提交耗时也用了4ms。。。
    ///
    /// ```ignore
    /// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    ///     nums.dedup();
    ///     nums.len() as i32
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20200830, mem=2.3, mem_beats=53.49, runtime=4, runtime_beats=30, url=https://leetcode-cn.com/submissions/detail/103003211/
    ///
    /// 很奇怪，同样的代码提交后runtime不一样 艹
    ///
    /// date=20200831, mem=2.3, mem_beats=27.91, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103290602/
    ///
    /// date=20200907, mem=2.1, mem_beats=98.51, runtime=4, runtime_beats=40, url=https://leetcode-cn.com/submissions/detail/105525720/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn remove_duplicates_4ms(nums: &mut Vec<i32>) -> i32 {
            if nums.len() == 0 {
                return 0;
            }
            let mut i = 0;
            for j in 1..nums.len() {
                if nums[i] != nums[j] {
                    i += 1;
                    nums[i] = nums[j];
                }
            }
            return i as i32 + 1;
        }

        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.is_empty() {
                return 0;
            }
            let mut i = 0;
            for j in 1..nums.len() {
                if nums[i] != nums[j] {
                    i += 1;
                    nums[i] = nums[j];
                }
            }
            i as i32 + 1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert_eq!(
                5,
                Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
            );
            assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
            assert_eq!(0, Solution::remove_duplicates(&mut vec![]));
        }
    }
}
