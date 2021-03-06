/// 总结
///
/// 20200908
///
/// reverse要注意k = k % nums.len()。
/// solution_ring忘记了怎么反转，看一眼还是能马上写出的，
/// 环形移动还是比较特殊的
pub mod solution_copy {

    /// # 思路
    ///
    /// 在copy数组上每个元素放到正确的位置上，`(i + k as usize) % len`：i -> i + k -> 0 -> i
    ///
    /// 参考：
    ///
    /// - [旋转数组](https://leetcode-cn.com/problems/rotate-array/solution/xuan-zhuan-shu-zu-by-leetcode/)
    ///
    /// ### submissions
    ///
    /// date=20200831, mem=2.3, mem_beats=6.9, runtime=100, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103236496/
    ///
    /// date=20200901, mem=2.2, mem_beats=72.41, runtime=100, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103602800/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let mut nums_copy = nums.clone();
            let len = nums.len();
            for i in 0..len {
                nums_copy[(i + k as usize) % len] = nums[i];
            }
            std::mem::swap(nums, &mut nums_copy);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
            Solution::rotate(&mut nums, 3);
            assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);
        }
    }
}

pub mod solution_ring {
    /// # 思路
    ///
    /// 将每个元素移劫k个位置。对被替换位置的元素temp放到下个k位置，直到所有元素完成
    ///
    /// 可以理解为一个环移动元素：
    ///
    /// ![](https://segmentfault.com/img/remote/1460000021692960)
    ///
    /// 如果`n%k==0 && k==k%n` (如n=6,k=2)，此时没有遍历所有数字的情况下回到出发数字start，
    /// 需要从下个数字重新开始
    ///
    /// 环状替换的例外情况并不仅仅在n%k==0时发生, 而是当k和n的最大公约数不为1时出现,
    /// 比如n=24, k=14时, 其最大公约数为2, 每次环状替换会替换12个数字, 所以需要遍历两次.
    ///
    /// count保证只会替换一次，也可用k和n的最大公约数来控制
    ///
    /// ![](https://pic.leetcode-cn.com/ffdad22d3d8e615e889cbfa8d60a51f207a8eab1926416723a9594b7d3774cb0-%E5%9B%BE%E7%89%87.png)
    ///
    /// 否则在一次遍历的情况下就可以完成移动
    ///
    /// 参考：
    ///
    /// - [旋转数组](https://leetcode-cn.com/problems/rotate-array/solution/xuan-zhuan-shu-zu-by-leetcode/)
    /// - [【旋转数组】原地换位，详细图解](https://leetcode-cn.com/problems/rotate-array/solution/xuan-zhuan-shu-zu-yuan-di-huan-wei-xiang-xi-tu-jie/)
    /// - [每日一道算法：旋转数组](https://segmentfault.com/a/1190000021692957)
    ///
    /// ### submissions
    ///
    /// date=20200831, mem=2.2, mem_beats=44.83, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103257131/
    ///
    /// date=20200901, mem=2.1, mem_beats=96.55, runtime=4, runtime_beats=47.27, url=https://leetcode-cn.com/submissions/detail/103606992/
    ///
    /// date=20200908, mem=2.2, mem_beats=52.38, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/105887056/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let len = nums.len();
            // k % len => 51%10 = 1
            let (mut count, k, mut start) = (0, k as usize % len, 0);
            while count < len {
                let mut prev_num = nums[start];
                let mut cur = start;
                loop {
                    let next = (cur + k) % len;
                    std::mem::swap(&mut nums[next], &mut prev_num);
                    cur = next;
                    count += 1;
                    if cur == start {
                        break;
                    }
                }
                start += 1;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
            Solution::rotate(&mut nums, 3);
            assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);
        }
    }
}

pub mod solution_reverse {
    /// # 思路
    ///
    /// 旋转数组 k 次， k%n 个尾部元素会被移动到头部，剩下的元素会被向后移动
    ///
    /// ```ignore
    /// n=7, k=3
    ///
    /// 原始数组                  : 1 2 3 4 5 6 7
    /// 反转所有数字后             : 7 6 5 4 3 2 1
    /// 反转前 k 个数字后          : 5 6 7 4 3 2 1
    /// 反转后 n-k 个数字后        : 5 6 7 1 2 3 4 --> 结果
    /// ```
    ///
    /// rust api
    ///
    /// ```rust,ignore
    /// pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    ///     let len = nums.len();
    ///     let k = k as usize % len;
    ///     nums.reverse();
    ///     nums[..k].reverse();
    ///     nums[k..].reverse();
    /// }
    /// ```
    ///
    /// ```rust,ignore
    /// pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    ///     let i=nums.len() as i32;
    ///     nums.rotate_right(k.rem_euclid(i) as usize)
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [旋转数组](https://leetcode-cn.com/problems/rotate-array/solution/xuan-zhuan-shu-zu-by-leetcode/)
    /// - [Rust reversing solution](https://leetcode.com/problems/rotate-array/discuss/483725/Rust-reversing-solution)
    /// - [0ms 2.3MB 用标准库它不香吗？](https://leetcode-cn.com/problems/rotate-array/solution/0ms-23mb-yong-biao-zhun-ku-ta-bu-xiang-ma-by-qweyt/)
    ///
    /// ### Submissions
    ///
    /// date=20200831, mem=2.3, mem_beats=27.59, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103284789/
    ///
    /// date=20200901, mem=2.1, mem_beats=82.76, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103610475/
    ///
    /// date=20200908, mem=2.2, mem_beats=28.57, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/105880830/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let len = nums.len();
            let k = k as usize % len;
            if len == 0 || k == 0 {
                return;
            }
            Self::reverse(nums, 0, len - 1);
            Self::reverse(nums, 0, k - 1);
            Self::reverse(nums, k, len - 1);
        }

        fn reverse(nums: &mut Vec<i32>, mut lo: usize, mut hi: usize) {
            while lo < hi {
                nums.swap(lo, hi);
                lo += 1;
                hi -= 1;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
            Solution::rotate(&mut nums, 3);
            assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);

            nums = vec![1];
            Solution::rotate(&mut nums, 0);
            assert_eq!(vec![1], nums);
        }
    }
}
