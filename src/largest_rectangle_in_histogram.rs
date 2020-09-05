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
            assert_eq!(2, Solution::largest_rectangle_area(vec![1,1]));
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
            assert_eq!(2, Solution::largest_rectangle_area(vec![1,1]));
        }
    }

}