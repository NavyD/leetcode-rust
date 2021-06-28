/// 总结
///
/// 20200918
///
/// 在solution_column_dp中要注意对每个height column计算用max-min = diff，
/// 可积水条件是min > cur height，计算积水量是用height_diff累积的
///
/// 在solution_double_pointer中是用height[left]<height[right]区分左右，
/// `height[left] < max_left`作为可信条件，不能光分可信条件了，忘记左右
///
/// 20200928
///
/// solution_column_dp要注意在left right max与下标的关系会影响计算高度差时
///
/// 20201002
///
/// solution_double_pointer注意如何分开双指针left,right条件，是根据`left_max<right_max`
/// 而不是`height[left]<height[right]`，`left==right`也要处理
pub mod solution_row {
    /// # 思路
    ///
    /// 求第 h 层的水，遍历每个位置，如果当前的高度小于 h，并且两边有高度大于等于 h 的，
    /// 说明这个地方一定有水，水就可以加 1
    ///
    /// 从左到右遍历墙的高度，遇到高度height > 当前层高度为h的水，表示可能在高度height形成h高的水，
    /// 可以尝试更新形成h高水的数量
    ///
    /// 当height<h时存在一个容量为1*1的水，更新count+=1。当height>=h表示完成之前到当前的count数量，
    /// count更新到res答案中，并count=0
    ///
    /// 由于h表示水的高度在向最高max_height迭代，每次height<h更新是表示这是一个1*1，不会重复计算
    ///
    /// 参考：
    ///
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/trapping-rain-water/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-w-8/)
    ///
    /// ### Submissions
    ///
    /// date=20200916, mem=2.1, mem_beats=11.11, runtime=560, runtime_beats=560, url=https://leetcode-cn.com/submissions/detail/108470415/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n^2)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn trap(height: Vec<i32>) -> i32 {
            let mut res = 0;
            // find max height
            let mut max_height = 0;
            for h in &height {
                if *h > max_height {
                    max_height = *h;
                }
            }
            // for h in max height
            for h in 1..=max_height {
                let mut is_started = false;
                let mut count = 0;
                for i in 0..height.len() {
                    // update count line if is started and h > height
                    if is_started && height[i] < h {
                        count += 1;
                    }
                    // update sum if height >= h
                    if height[i] >= h {
                        is_started = true;
                        res += count;
                        count = 0;
                    }
                }
            }
            res
        }
    }
}

pub mod solution_column {
    /// # 思路
    ///
    /// 求每一列的水，我们只需要关注当前列，以及左边最高的墙，右边最高的墙就够了。
    /// 当然根据木桶效应，我们只需要看左边最高的墙和右边最高的墙中较矮的一个就够了。
    ///
    /// - 较矮的墙的高度大于当前列的墙的高度
    ///
    /// 较矮的一边，也就是左边的墙的高度，减去当前列的高度就可以了
    ///
    /// ![](https://pic.leetcode-cn.com/542754f4431d93141920185252aee31664a96dd17285b92dfe390e9e977bebb1-image.png)
    ///
    ///
    /// - 较矮的墙的高度小于当前列的墙的高度
    ///
    /// 正在求的列不会有水，因为它大于了两边较矮的墙。
    ///
    /// ![](https://pic.leetcode-cn.com/19a50c8f4125c01349ad32d069f564b51fbb4347fd91eae079b6ec1a46c1ccee-image.png)
    ///
    /// - 较矮的墙的高度等于当前列的墙的高度。
    ///
    /// 和上一种情况是一样的，不会有水。
    ///
    /// 参考：
    ///
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/trapping-rain-water/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-w-8/)
    /// ### Submissions
    ///
    /// date=20200917, mem=2.1, mem_beats=22.22, runtime=72, runtime_beats=9.68, url=https://leetcode-cn.com/submissions/detail/108806916/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n^2)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn trap(height: Vec<i32>) -> i32 {
            if height.is_empty() {
                return 0;
            }
            let mut res = 0;
            // row for 1 to height.len() - 1
            for i in 1..height.len() - 1 {
                // find the max on the left of cur row
                let mut left_max = height[0];
                for j in 1..i {
                    if left_max < height[j] {
                        left_max = height[j]
                    }
                }
                // find the max on the right of cur row
                let mut right_max = height[height.len() - 1];
                for j in i + 1..height.len() {
                    if right_max < height[j] {
                        right_max = height[j];
                    }
                }
                // get min from left and right
                let min = left_max.min(right_max);
                // cur column is smaller
                if min > height[i] {
                    res += min - height[i];
                }
            }
            res
        }
    }
}

pub mod solution_column_dp {
    /// # 思路
    ///
    /// 在solution_row中多次重复计算max left,right，对于当前列i，
    /// `max_left [i]` 代表第 i 列左边最高的墙的高度，`max_right[i]` 代表第 i 列右边最高的墙的高度
    ///
    /// 如何求出当前列i的max_left, max_right
    ///
    /// i从1开始，max_left[1] = height[0]，max_left[2]=max(height[1], max_left[1])即有
    /// `max_left [i] = Max(max_left [i-1],height[i-1])`，同理可求出`max_right[i] = Max(max_right[i+1],height[i+1])`
    ///
    /// max_right[i]不能与max_left在一个循环中从0开始找，则不会后面出现更大的值时前面的就不可用了，
    /// max_right[i]中的i必须从后面找起，才能使max_right作为i右边最大的列
    ///
    /// ## 注意
    ///
    /// 下面是另一种可行解，相比当前解法，`left_max[i] = left_max[i - 1].max(height[i])`
    /// 这里是left_max[i]表示在i是`1..=i`的最大值，right_max[i]表示`i..=len-1`的最大值，在找res的高度差
    /// `height_diff = left_max[i].min(right_max[i]) - height[i] >= 0`是一定成立的，可直接res+=height_diff
    ///
    /// 如果` max_left[i] = max_left[i - 1].max(height[i - 1]);`表示i前的最大值，这个
    /// `height_diff = left_max[i].min(right_max[i]) - height[i]`是可能<0的
    ///
    /// ```ignore
    /// pub fn trap(height: Vec<i32>) -> i32 {
    ///     let len = height.len();
    ///     if len <= 2 {
    ///         return 0;
    ///     }
    ///     let mut left_max = vec![0; len];
    ///     let mut right_max = vec![0; len];
    ///     left_max[0] = height[0];
    ///     right_max[len - 1] = height[len - 1];
    ///     for i in 1..height.len() {
    ///         left_max[i] = left_max[i - 1].max(height[i]);
    ///     }
    ///     for i in (0..len - 1).rev() {
    ///         right_max[i] = right_max[i + 1].max(height[i]);
    ///     }
    ///     let mut res = 0;
    ///     for i in 0..len {
    ///         let min = left_max[i].min(right_max[i]);
    ///         res += min - height[i];
    ///     }
    ///     res
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/trapping-rain-water/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-w-8/)
    ///
    /// ### Submissions
    ///
    /// date=20200917, mem=2, mem_beats=61.11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/108816444/
    ///
    /// date=20200918, mem=2.2, mem_beats=5.56, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/109367289/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn trap(height: Vec<i32>) -> i32 {
            if height.is_empty() {
                return 0;
            }
            let mut res = 0;
            let mut max_left = vec![0; height.len()];
            let mut max_right = vec![0; height.len()];
            // get max left
            for i in 1..height.len() - 1 {
                max_left[i] = max_left[i - 1].max(height[i - 1]);
            }
            // get max right
            for i in (1..height.len() - 1).rev() {
                max_right[i] = max_right[i + 1].max(height[i + 1]);
            }
            // get res
            for i in 1..height.len() - 1 {
                let min = max_left[i].min(max_right[i]);
                if min > height[i] {
                    res += min - height[i];
                }
            }
            res
        }
    }
}

pub mod solution_double_pointer {
    /// # 思路
    ///
    /// 在solution_column_dp中提到max_right从要从len-1后向前开始找，当我们从左往右处理到left下标时，
    /// 左边的最大值left_max对它而言是可信的，但right_max对它而言是不可信的。
    ///
    /// 当我们从右往左处理到right下标时，右边的最大值right_max对它而言是可信的，但left_max对它而言是不可信的。
    ///
    /// ```ignore
    ///    right_max
    ///    left_max                             __
    ///      __                                |  |
    ///     |  |__   __??????????????????????  |  |
    ///   __|     |__|                       __|  |__
    ///           left                      right
    /// ```
    ///
    /// 对于位置left而言，它左边最大值一定是left_max，右边最大值“大于等于”right_max，
    /// 如果left_max<right_max成立，那么它就知道自己能存多少水了。无论右边将来会不会出现更大的right_max，
    /// 都不影响这个结果。 所以当left_max<right_max时，我们就希望去处理left下标，
    /// 反之，我们希望去处理right下标
    ///
    /// ```ignore
    /// while left <= right {
    ///     // left max is trusted
    ///     if left_max < right_max {
    ///         // ...
    ///     }
    ///     // right max is trusted
    ///     else {
    ///         // ...
    ///     }
    /// }
    /// // 这种方式不如上面的容易理解
    /// while left < right {
    ///     if height[left] < height[right] {
    ///         if max_left > height[left] {
    ///     }//...
    /// }
    /// ```
    ///
    /// 注意要处理`while left <= right {`，当`left==right`时，有left_max, right_max都是不包括当前
    /// 下标`left==right`的值，所以应有条件处理`left==right`
    ///
    /// 参考：
    ///
    /// - [那么如何理解双指针法呢](https://leetcode-cn.com/problems/trapping-rain-water/solution/jie-yu-shui-by-leetcode/327718)
    /// - [接雨水](https://leetcode-cn.com/problems/trapping-rain-water/solution/jie-yu-shui-by-leetcode/327718/)
    ///
    /// ### Submissions
    ///
    /// date=20200917, mem=2, mem_beats=61.11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/108826398/
    ///
    /// date=20200918, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/109371378/
    ///
    /// date=20201002, mem=2.1, mem_beats=56.25, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/112844228/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn trap(height: Vec<i32>) -> i32 {
            if height.len() <= 1 {
                return 0;
            }
            let mut res = 0;
            let (mut left, mut right) = (0, height.len() - 1);
            let (mut left_max, mut right_max) = (0, 0);
            while left <= right {
                // left max is trusted
                if left_max < right_max {
                    // get res if  cur column is smaller
                    if height[left] < left_max {
                        res += left_max - height[left];
                    } else {
                        // update max if left is bigger
                        left_max = height[left];
                    }
                    left += 1;
                }
                // right max is trusted
                else {
                    if height[right] < right_max {
                        res += right_max - height[right];
                    } else {
                        right_max = height[right];
                    }
                    right -= 1;
                }
            }
            res
        }
    }
}

pub mod solution_monotonous_stack {
    /// # 思路
    ///
    /// 用栈来跟踪可能储水的最长的条形块，在两块间低的条块中
    ///
    /// 如果当前的条形块小于或等于栈顶的条形块，我们将条形块的索引入栈，意思是当前的条形块被栈中的前一个条形块界定
    ///
    /// 如果我们发现一个条形块长于栈顶，我们可以确定栈顶的条形块被当前条形块和栈的前一个条形块界定，
    /// 因此我们可以弹出栈顶元素并且累加答案到res
    ///
    /// 方式类似于找高度，遇到第一个计算高度差height_diff与width，stack新的top与当前height计算的
    /// 高度差是不包含之前的高度，而是width包含，所以有：
    /// `(height_diff = height[i].min(height[*last]) - height[cur])*(i - last - 1)`
    ///
    /// 以solution_column_dp来看，`height[i]`表示柱子right_max, `height[last]`表示可接雨水的柱子，stack.pop后的last`height[last]`
    /// 表示left_max柱子。是以last为接雨水的柱子计算的
    ///
    /// 栈内元素是单调递减的如：`[4, 3, 1, 0]`，如果是`height=[1,2,3]`则stack中不会存在任何元素，
    /// 每一次都有`height[i] > height[*last]`导致stack.pop()一直为空
    ///
    /// 参考：
    ///
    /// - [单调栈O(n)解决，动图预警](https://leetcode-cn.com/problems/trapping-rain-water/solution/dan-diao-zhan-jie-jue-jie-yu-shui-wen-ti-by-sweeti/)
    /// - [接雨水](https://leetcode-cn.com/problems/trapping-rain-water/solution/jie-yu-shui-by-leetcode/327718/)
    ///
    /// ### Submissions
    ///
    /// date=20200918, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/109363764/
    ///
    /// date=20201002, mem=2, mem_beats=71.88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/112851473/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn trap(height: Vec<i32>) -> i32 {
            let mut stack = Vec::with_capacity(height.len());
            let mut res = 0;
            for i in 0..height.len() {
                // while cur height > top stack height
                while let Some(last) = stack.last() {
                    if height[i] <= height[*last] {
                        break;
                    }
                    let cur = stack.pop().unwrap();
                    // break if stack has no elements. can not storage
                    if let Some(last) = stack.last() {
                        let width = i - *last - 1;
                        // get min height in between cur and stack new top
                        let height_diff = height[i].min(height[*last]) - height[cur];
                        res += width as i32 * height_diff;
                    }
                }
                stack.push(i);
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_row::Solution::trap);
        test(solution_column::Solution::trap);
        test(solution_column_dp::Solution::trap);
        test(solution_double_pointer::Solution::trap);
        test(solution_monotonous_stack::Solution::trap);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(6, func(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        assert_eq!(0, func(vec![]));
        assert_eq!(0, func(vec![0]));
    }
}
