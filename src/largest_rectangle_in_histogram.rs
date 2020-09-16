/// 总结
/// 
/// 这个题目花了3次才开始了解，从开始不清楚背解法 -> 理解暴力解法 -> 单调栈，哨兵
/// 流程还是不能少，不能直接背解法，记不住，要由于题意开始一步步深入
/// 
/// 开始尝试用[container_with_most_water](https://leetcode-cn.com/problems/container-with-most-water/)这个双指针
/// 解法，但发现其意思与该题不同，柱形面积不是两边最小的height*width，而是用当前
/// 高度为最小相邻的有多少个柱形width
/// 
/// 20200916
/// 
/// 注意solution_stack_sentinel中`width=i-last-1 | i`的含义
pub mod solution_violent_width {
    /// # 思路
    ///
    /// 枚举「宽」，我们可以使用两重循环枚举矩形的左右边界以固定宽度 w，
    /// 此时矩形的高度 h，就是所有包含在内的柱子的「最小高度」，对应的面积为 w * h
    ///
    /// 参考：
    ///
    /// - [柱状图中最大的矩形](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/zhu-zhuang-tu-zhong-zui-da-de-ju-xing-by-leetcode-/)
    ///
    /// ### Submissions
    ///
    /// date=20200906, mem=2.2, mem_beats=87.50, runtime=684, runtime_beats=5.97, url=https://leetcode-cn.com/submissions/detail/105186410/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n^2)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            let mut max_area = 0;
            for left in 0..heights.len() {
                let mut min_height = heights[left];
                // 获取右边最小高度 对应的width
                for right in left..heights.len() {
                    let width = (right - left + 1) as i32;
                    min_height = min_height.min(heights[right]);
                    max_area = max_area.max(min_height * width);
                }
            }
            max_area
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
            assert_eq!(0, Solution::largest_rectangle_area(vec![]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
            assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        }
    }
}

pub mod solution_violent_height {
    /// # 思路
    ///
    /// 枚举以每个柱形为高度的最大矩形的面积，依次遍历柱形的高度，
    /// 对于每一个高度分别向两边扩散，求出以当前高度为矩形的最大宽度多少
    ///
    /// - 左边看一下，看最多能向左延伸多长，找到大于等于当前柱形高度的最左边元素的下标；
    /// - 右边看一下，看最多能向右延伸多长；找到大于等于当前柱形高度的最右边元素的下标。
    ///
    /// ![](https://pic.leetcode-cn.com/b4125f95419bc2306c7f16d1679c32e538b0b087bd9d0f70658c1a8528afca6b-image.png)
    ///
    /// 参考：
    ///
    /// - [暴力解法、栈（单调栈、哨兵技巧）](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/bao-li-jie-fa-zhan-by-liweiwei1419/)
    /// - [柱状图中最大的矩形](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/zhu-zhuang-tu-zhong-zui-da-de-ju-xing-by-leetcode-/)
    ///
    /// ### Submissions
    ///
    /// date=20200906, mem=2.2, mem_beats=93.75, runtime=880, runtime_beats=5.97, url=https://leetcode-cn.com/submissions/detail/105190091/
    /// 
    /// date=20200908, mem=2.3, mem_beats=77.78, runtime=884, runtime_beats=5.8, url=https://leetcode-cn.com/submissions/detail/105869662/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n^2)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            let mut max_area = 0;
            for mid in 0..heights.len() {
                let height = heights[mid];
                let (mut left, mut right) = (mid, mid);
                // 向左边找 >= height
                while left > 0 && heights[left - 1] >= height {
                    left -= 1;
                }
                // 向右边找 >= height
                while right < heights.len() - 1 && heights[right + 1] >= height {
                    right += 1;
                }
                let width = (right - left + 1) as i32;
                max_area = max_area.max(width * height);
            }
            max_area
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
            assert_eq!(0, Solution::largest_rectangle_area(vec![]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
            assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        }
    }
}

pub mod my_solution_double_pointer {
    /// 如何解决中间的最小值
    pub struct Solution;

    impl Solution {
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            if heights.is_empty() {
                return 0;
            }
            let mut max_area = heights[0];
            let (mut lo, mut hi) = (0, heights.len() - 1);
            while lo < hi {
                let area = heights[lo].min(heights[hi]) * (hi - lo + 1) as i32;
                // Interval and individual
                max_area = max_area.max(area).max(heights[hi]);
                if heights[lo] <= heights[hi] {
                    lo += 1;
                } else {
                    hi -= 1;
                }
            }
            max_area
        }
    }
}

pub mod solution_monotonous_stack {
    /// # 思路
    ///
    /// 在枚举高度的方法中，当前柱形的高度比它上一个柱形的高度严格小的时候，
    /// 一定可以确定它之前的某些柱形的最大宽度，并且确定的柱形宽度的顺序是从右边向左边。
    ///
    /// 在遍历的时候需要记录的信息就是遍历到的柱形的下标，
    /// 它一左一右的两个柱形的下标的差就是这个面积最大的矩形对应的最大宽度
    ///
    /// 确定一个柱形的面积的时候，右边要比当前严格小，左边也要比当前高度严格小。相等高度时是不能确定的
    ///
    /// 在缓存数据的时候，是从左向右缓存的，我们计算出一个结果的顺序是从右向左的，
    /// 并且计算完成以后我们就不再需要了，符合后进先出的特点 栈
    /// 
    /// 注意：`while let Some(last) = last_indices.last() {`往前找的部分可移除，在后面循环中被
    /// 多次处理
    /// 
    ///
    /// 参考：
    ///
    /// - [暴力解法、栈（单调栈、哨兵技巧）](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/bao-li-jie-fa-zhan-by-liweiwei1419/)
    ///
    /// ### Submissions
    ///
    /// date=20200907, mem=2.4, mem_beats=29.41, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/105482050/
    ///
    /// date=20200908, mem=2.5, mem_beats=27.78, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/105874885/
    /// 
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            let mut max_area = 0;
            let mut last_indices = Vec::with_capacity(heights.len());
            for i in 0..heights.len() {
                // 找可以确定面积heights[i] < last_height 之前的柱形，可能有多个比heights[i]大的柱形可确定
                while let Some(last) = last_indices.last() {
                    let last_height = heights[*last];
                    // 左边height <= 当前height不能确定左边height面积
                    if last_height <= heights[i] {
                        break;
                    }
                    last_indices.pop();
                    // find the last == last_height 相同高度的柱形 往前找
                    while let Some(last) = last_indices.last() {
                        if last_height != heights[*last] {
                            break;
                        }
                        last_indices.pop();
                    }
                    // last_height对应的宽度 以当前下标i
                    let last_width = if let Some(last) = last_indices.last() {
                        i - last - 1
                    } else {
                        i
                    };
                    max_area = max_area.max(last_width as i32 * last_height);
                }
                last_indices.push(i);
            }
            // 还在栈里的柱形
            while let Some(i) = last_indices.pop() {
                let height = heights[i];
                // 相同高度的柱形 往前找
                while let Some(last) = last_indices.last() {
                    if height != heights[*last] {
                        break;
                    }
                    last_indices.pop();
                }
                // 以当前下标为heights.len()计算width
                let width = if let Some(last) = last_indices.last() {
                    heights.len() - last - 1
                }
                // 最小的柱形 宽度是整个数组
                else {
                    heights.len()
                };
                max_area = max_area.max(width as i32 * height);
            }
            max_area
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
            assert_eq!(0, Solution::largest_rectangle_area(vec![]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
            assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        }
    }
}

pub mod solution_stack_sentinel {

    /// # 思路
    /// 
    /// 遍历完成以后，栈中还有元素，在heights最后加个高度为 0 （或者是 0.5，只要比 1 严格小都行）的柱形，
    /// 以回避上面这后面的单独循环，在最后for一次处理，
    /// 因为它一定比输入数组里任何一个元素小，它会让所有输入数组里的元素出栈
    /// 
    /// 可以省略往前找的循环`while let Some(last) = last_indices.last() {`，外部循环while let可处理
    /// 
    /// 如何处理width与i的关系
    /// 
    /// - 当last_indices中还存在元素时，即`i.last.last`存在，last_heigth对应的`i.last`已经被pop了，则`i-last-1= i - i.last`
    /// 如计算height=5的面积：`last_indices=1; last=1, i=4 ==> i-last-1 = 4 - 1 - 1 = 2`
    /// 
    /// 当last_indices没有元素时，表示当前last就是最后的也是最小的height，这个柱形的width就是最长的，整个未加sentinel的heigth.len()
    /// 
    /// 参考：
    /// 
    /// - [单调递增栈](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/xi-li-hu-tu-bu-ming-suo-yi-by-alien-7/)
    /// - [暴力解法、栈（单调栈、哨兵技巧）](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/bao-li-jie-fa-zhan-by-liweiwei1419/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200907, mem=2.4, mem_beats=29.41, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/105495785/
    /// 
    /// date=20200916, mem=2.3, mem_beats=46.15, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/108503945/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;
    impl Solution {
        pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
            // sentinel
            heights.push(0);
            let mut last_indices = Vec::with_capacity(heights.len());
            let mut max_area = 0;
            for i in 0..heights.len() {
                // 找可以确定面积heights[i] < last_height 之前的柱形，可能有多个比heights[i]大的柱形可确定
                // sentinel 0: i=len,height=0 最后一次for时while可处理 清空栈
                while let Some(last) = last_indices.last() {
                    let last_height = heights[*last];
                    // 左边height <= 当前height不能确定左边height面积
                    if heights[i] >= last_height {
                        break;
                    }
                    last_indices.pop();
                    let last_width = if let Some(last) = last_indices.last() {
                        i - last - 1
                    } else {
                        i
                    };
                    max_area = max_area.max(last_width as i32 * last_height);
                }
                last_indices.push(i);
            }
            // 最后栈还有一个sentinel元素=len
            max_area
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(10, Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
            assert_eq!(0, Solution::largest_rectangle_area(vec![]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
            assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        }
    }
}
