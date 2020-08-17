/// 总结
/// 
/// 没有思路
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
    /// 参考：
    /// 
    /// - [排序 + 双指针，逐行解释](https://leetcode-cn.com/problems/3sum/solution/pai-xu-shuang-zhi-zhen-zhu-xing-jie-shi-python3-by/)
    /// - [三数之和](https://leetcode-cn.com/problems/3sum/solution/san-shu-zhi-he-by-leetcode-solution/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200817, mem=3.4, mem_beats=75, runtime=28, runtime_beats=77.7, url=https://leetcode-cn.com/submissions/detail/98864502/
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
                for i in 0..len {
                    let cur = nums[i];
                    // 去重
                    if i != 0 && cur == nums[i - 1] {
                        continue;
                    }
                    let (mut left, mut right) = (i + 1, len - 1);
                    while left < right {
                        let sum = cur + nums[left] + nums[right];
                        if sum == 0 {
                            res.push(vec![cur, nums[left], nums[right]]);
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
                vec![vec![-1, -1, 2], vec![-1, 0, 1], ],
                Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
            );

            assert_eq!(
                vec![vec![-2,0,2], ],
                Solution::three_sum(vec![-2,0,0,2,2])
            );
        }
    }
}
