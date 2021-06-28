/// 总结
///
/// 没有思路
///
/// 20200818
///
/// 注意在left,right时不能重复`while left < right && nums[left + 1] == nums[left] {`
///
/// 20200825
///
/// 注意在去重时`while left < right && nums[right - 1] == nums[right] {`后一定要用
/// `right -= 1;left += 1;`使left,right不处于重复的位置
pub mod solution_sort {
    /// # 思路
    ///
    /// 用3次循环找出
    ///
    /// 如何去除重复解
    ///
    /// - 对nums排序
    /// - 对于连续出现的元素跳过`cur == nums[i - 1]`
    /// - 双指针left,right有`cur + nums[left] + nums[right]==0`时，
    /// 判断左界和右界是否和下一位置重复，去除重复解。并同时将 L,RL,R 移到下一位置，寻找新的解
    ///
    /// 一点优化：有序的nums[i]>0时后面的数必定有sum>0
    ///
    /// ```ignore
    /// for i in 0..len {
    ///     if nums[i] > 0 {
    ///         break;
    ///     } else if i != 0 && nums[i] == nums[i - 1] {
    ///         continue;
    ///     }
    /// ```
    ///
    /// 可用set去重而不是指针移动
    ///
    /// ```java
    /// while(j<k){
    ///     int sum = nums[i]+nums[j]+nums[k];
    ///     if(sum==0)res.add(Arrays.asList(nums[i],nums[j++],nums[k--]));
    ///     else if ( sum >0) k--;
    ///     else if (sum<0) j++;
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [排序 + 双指针，逐行解释](https://leetcode-cn.com/problems/3sum/solution/pai-xu-shuang-zhi-zhen-zhu-xing-jie-shi-python3-by/)
    /// - [三数之和](https://leetcode-cn.com/problems/3sum/solution/san-shu-zhi-he-by-leetcode-solution/)
    /// - [Concise O(N^2) Java solution](https://leetcode.com/problems/3sum/discuss/7380/Concise-O(N2)-Java-solution)
    /// - [Java with set](https://leetcode.com/problems/3sum/discuss/143636/Java-with-set)
    ///
    /// ### Submissions
    ///
    /// date=20200817, mem=3.4, mem_beats=75, runtime=28, runtime_beats=77.7, url=https://leetcode-cn.com/submissions/detail/98864502/
    ///
    /// date=20200818, mem=3.4, mem_beats=57.14, runtime=24, runtime_beats=95.68, url=https://leetcode-cn.com/submissions/detail/99437998/
    ///
    /// date=20200825, mem=3.3, mem_beats=93.75, runtime=32, runtime_beats=49.59, url=https://leetcode-cn.com/submissions/detail/101728221/
    ///
    /// ### 复杂度
    ///
    /// - 时间复杂度：O(N^2)，其中 NN 是数组 nums 的长度。
    /// - 空间复杂度：O(N)。我们忽略存储答案的空间，额外的排序的空间复杂度为 O(logN)。
    /// 然而我们修改了输入的数组 nums，在实际情况下不一定允许，因此也可以看成使用了一个
    /// 额外的数组存储了nums 的副本并进行排序，空间复杂度为O(N)。
    pub struct Solution;

    impl Solution {
        pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut nums = nums;
            let len = nums.len();
            let mut res = vec![];
            if len >= 3 {
                nums.sort();
                for i in 0..len - 2 {
                    // 有序
                    if nums[i] > 0 {
                        break;
                    }
                    // 去重
                    if i != 0 && nums[i] == nums[i - 1] {
                        continue;
                    }
                    let (mut left, mut right) = (i + 1, len - 1);
                    while left < right {
                        let sum = nums[i] + nums[left] + nums[right];
                        if sum == 0 {
                            res.push(vec![nums[i], nums[left], nums[right]]);
                            // 去重
                            while left < right && nums[left + 1] == nums[left] {
                                left += 1;
                            }
                            while left < right && nums[right - 1] == nums[right] {
                                right -= 1;
                            }
                            right -= 1;
                            left += 1;
                        } else if sum > 0 {
                            right -= 1;
                        } else {
                            left += 1;
                        }
                    }
                }
            }
            res
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                vec![vec![-1, -1, 2], vec![-1, 0, 1],],
                Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
            );

            assert_eq!(
                vec![vec![-2, 0, 2],],
                Solution::three_sum(vec![-2, 0, 0, 2, 2])
            );
        }
    }
}
