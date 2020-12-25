//! 如何找到多数元素

pub mod solution_sort {
    /// # 思路
    ///
    /// 出现频率大于n/2，所以排序后的中间位置必然是众数
    ///
    /// ### Submissions
    ///
    /// date=20201217, mem=2.2, mem_beats=88, runtime=4, runtime_beats=44, url=https://leetcode-cn.com/submissions/detail/131706297/
    pub struct Solution;

    impl Solution {
        pub fn majority_element(mut nums: Vec<i32>) -> i32 {
            nums.sort_unstable();
            nums[nums.len() / 2]
        }
    }
}

pub mod solution_hash {
    /// # 思路
    /// 
    /// 下面的方法是由于众数的元素数量多于n/2，所以在到最后的元素时必然会找到count > n/2
    /// 
    /// ```ignore
    /// pub fn majority_element(nums: Vec<i32>) -> i32 {
    ///     let mut counts = std::collections::HashMap::with_capacity(nums.len());
    ///     let max_count = nums.len() / 2 + 1;
    ///     for num in &nums {
    ///         let count = counts
    ///             .entry(num)
    ///             .and_modify(|count| *count += 1)
    ///             .or_insert(1);
    ///         if *count >= max_count {
    ///             return *num;
    ///         }
    ///     }
    ///     panic!()
    /// }
    /// ```
    ///
    /// 多一次遍历
    /// 
    /// ```ignore
    /// pub fn majority_element(nums: Vec<i32>) -> i32 {
    ///     let mut counts = std::collections::HashMap::with_capacity(nums.len());
    ///     nums.iter().for_each(|num| {
    ///         counts
    ///             .entry(num)
    ///             .and_modify(|count| *count += 1)
    ///             .or_insert(1);
    ///     });
    ///     **counts.iter()
    ///         .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
    ///         .map(|(k, _)| k)
    ///         .unwrap()
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20201217, mem=2.4, mem_beats=18, runtime=4, runtime_beats=44, url=https://leetcode-cn.com/submissions/detail/131725247/
    /// 
    /// date=20201218, mem=2.3, mem_beats=86, runtime=4, runtime_beats=44, url=https://leetcode-cn.com/submissions/detail/132099536/
    /// 
    /// date=20201225, mem=2.3, mem_beats=29, runtime=4, runtime_beats=50, url=https://leetcode-cn.com/submissions/detail/133664081/
    pub struct Solution;

    impl Solution {
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            let mut counts = std::collections::HashMap::with_capacity(nums.len());
            let mut max_count = 1;
            let mut mode = nums[0];
            for num in &nums {
                let count = counts
                    .entry(num)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                if *count > max_count {
                    mode = *num;
                    max_count = *count;
                }
            }
            mode
        }
    }
}

pub mod solution_divide {
    /// # 思路
    ///
    /// 如果数 a 是数组 nums 的众数，如果我们将 nums 分成两部分，那么 a 必定是至少一部分的众数。
    ///
    /// 如何合并两个部分的众数。当两个部分的众数相同时，合并众数就是。如果不同时，需要比较对应的出现次数，
    /// 次数多的为合并众数，如果次数一样，表示当前层无法找出众数，分治上层的范围会再次统计合并众数
    ///
    /// 参考：
    ///
    /// [多数元素](https://leetcode-cn.com/problems/majority-element/solution/duo-shu-yuan-su-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20201217, mem=2.3, mem_beats=86, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/131741299/
    /// 
    /// date=20201218, mem=2.5, mem_beats=6, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132100515/
    /// 
    /// date=20201225, mem=2.3, mem_beats=62, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133666622/
    pub struct Solution;

    impl Solution {
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            fn _helper(nums: &[i32]) -> i32 {
                // 一个元素的众数
                if nums.len() == 1 {
                    return nums[0];
                }

                let mid = nums.len() / 2;
                let left_mode = _helper(&nums[..mid]);
                let right_mode = _helper(&nums[mid..]);

                // 众数不一样
                if left_mode != right_mode {
                    let (mut left_count, mut right_count) = (0, 0);
                    // 统计出现次数 在整个nums slice范围
                    nums.iter().for_each(|num| {
                        if left_mode == *num {
                            left_count += 1;
                        } else if right_mode == *num {
                            right_count += 1;
                        }
                    });
                    if left_count < right_count {
                        return right_mode;
                    }
                }
                // 众数一样 或 left_mode_count >= right_mode_count
                left_mode
            }
            _helper(&nums)
        }
    }
}

pub mod solution_moore {
    /// # 思路
    ///
    /// 如果我们把众数记为 +1，把其他数记为 -1，将它们全部加起来，显然和大于 0，从结果本身我们可以看出众数比其他数多。
    ///
    /// 投票算法证明：
    ///
    /// - 如果候选人不是maj 则 maj会和其他非候选人一起反对 会反对候选人,所以候选人一定会下台(maj==0时发生换届选举)
    /// - 如果候选人是maj , 则maj会支持自己，其他候选人会反对，同样因为maj 票数超过一半，所以maj 一定会成功当选
    ///
    /// 参考：
    ///
    /// [多数元素](https://leetcode-cn.com/problems/majority-element/solution/duo-shu-yuan-su-by-leetcode-solution/)
    ///
    /// [投票算法证明](https://leetcode-cn.com/problems/majority-element/solution/duo-shu-yuan-su-by-leetcode-solution/283251)
    ///
    /// 注意：不能连写else if，更换candidate后还要计票count=1
    ///
    /// ```ignore
    /// if count == 0 {
    ///     candidate = num;
    /// } else if candidate == num {
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20201217, mem=2.2, mem_beats=97, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/131754036/
    /// 
    /// date=20201218, mem=2.3, mem_beats=83, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132100983/
    /// 
    /// date=20201225, mem=2.3, mem_beats=37, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133667496/
    pub struct Solution;

    impl Solution {
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            let mut candidate = &nums[0];
            let mut count = 0;
            for num in &nums {
                // 换人
                if count == 0 {
                    candidate = num;
                }

                // 自己给自己投票
                if candidate == num {
                    count += 1;
                }
                // 其它人反对
                else {
                    count -= 1;
                }
            }
            *candidate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
            assert_eq!(func(vec![3, 2, 3]), 3);
            assert_eq!(func(vec![2, 2, 1, 1, 1, 2, 2]), 2);
            assert_eq!(func(vec![3, 3, 4]), 3);
        }
        test(solution_sort::Solution::majority_element);
        test(solution_hash::Solution::majority_element);
        test(solution_divide::Solution::majority_element);
        test(solution_moore::Solution::majority_element);
    }
}
