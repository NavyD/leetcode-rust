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

pub mod solution_double_pointer {
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

pub mod solution_violent {
    pub struct Solution;

    impl Solution {
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            if heights.is_empty() {
                return 0;
            }
            let mut max_area = 0;
            for (i, cur_height) in heights.iter().enumerate() {
                let (mut lo, mut hi) = (i, i);
                while lo > 0 && &heights[i - 1] >= cur_height {
                    lo -= 1;
                }
                while hi < heights.len() - 1 && &heights[hi + 1] >= cur_height {
                    hi += 1;
                }
                let width = hi - lo + 1;
                max_area = max_area.max(width as i32 * cur_height);
            }
            max_area
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn basics() {
    //         assert_eq!(10, Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
    //         assert_eq!(0, Solution::largest_rectangle_area(vec![]));
    //         assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
    //         assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
    //         assert_eq!(2, Solution::largest_rectangle_area(vec![1,1]));
    //     }
    // }
}

pub mod solution_monotonous_stack {
    pub struct Solution;

    impl Solution {
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            if heights.len() == 0 {
                return 0;
            }
            if heights.len() == 1 {
                return heights[0];
            }
            let mut last_indices = Vec::with_capacity(heights.len());
            let mut max_area = 0;
            for (i, height) in heights.iter().enumerate() {
                while let Some(last) = last_indices.last() {
                    let last_height = &heights[*last];
                    if height >= last_height {
                        break;
                    }
                    // height < last_height
                    // find the last == last_height
                    while let Some(last) = last_indices.pop() {
                        if last_height != &heights[last] {
                            last_indices.push(last);
                            break;
                        }
                    }
                    let width = if let Some(last) = last_indices.last() {
                        i - last - 1
                    } else {
                        i
                    };
                    max_area = max_area.max(width as i32 * height);
                }
                last_indices.push(i);
            }
            while let Some(i) = last_indices.pop() {
                let height = heights[i];
                while let Some(last) = last_indices.pop() {
                    if heights[last] != height {
                        last_indices.push(last);
                        break;
                    }
                }
                let width = if let Some(last) = last_indices.pop() {
                    heights.len() - last - 1
                } else {
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
            // assert_eq!(10, Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
            assert_eq!(0, Solution::largest_rectangle_area(vec![]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
            assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        }
    }
}

pub mod solution_stack_sentinel {

    pub struct Solution;
    impl Solution {
        pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
            heights.push(0);
            let mut last_indices = Vec::with_capacity(heights.len());
            let mut max_area = 0;
            for (i, height) in heights.iter().enumerate() {
                while let Some(last) = last_indices.last() {
                    if height < &heights[*last] {
                        last_indices.pop();
                        let width = if let Some(last) = last_indices.last() {
                            i - last - 1
                        } else {
                            i
                        };
                        max_area = max_area.max(width as i32 * height);
                    }
                }
                last_indices.push(i);
            }
            max_area
        }

        pub fn largest_rectangle_area1(heights: Vec<i32>) -> i32 {
            let mut stack = vec![];
            let mut res = 0;
            for i in 0..=heights.len() {
                let cur_height = if i == heights.len() { 0 } else { heights[i] };

                while !stack.is_empty() && cur_height < heights[*stack.last().unwrap()] {
                    let h = stack.pop().unwrap();
                    if stack.is_empty() {
                        res = res.max(i as i32 * heights[h])
                    } else {
                        res = res.max((i - *stack.last().unwrap() - 1) as i32 * heights[h]);
                    }
                }
                stack.push(i)
            }
            res
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            // assert_eq!(10, Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
            assert_eq!(0, Solution::largest_rectangle_area(vec![]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![5]));
            assert_eq!(5, Solution::largest_rectangle_area(vec![2, 5]));
            assert_eq!(2, Solution::largest_rectangle_area(vec![1, 1]));
        }
    }
}
