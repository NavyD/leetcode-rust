/// 总结
/// 
/// solution_double_points这个想不出来，只能用暴力
/// 破解
pub mod my_solution {

    /// # 思路
    /// 
    /// 暴力破解
    /// 
    /// ### Submissions
    /// 
    /// date=20200816, mem=2.2, mem_beats=22.22, runtime=184, runtime_beats=6.67, url=https://leetcode-cn.com/submissions/detail/98490296/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n^2)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut max_area = 0;
            for i in 0..height.len() {
                for j in i + 1 .. height.len() {
                    max_area = max_area.max(height[i].min(height[j]) * (j - i) as i32);
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
            assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
        }
    }
}

pub mod solution_double_points {
    /// # 思路 
    /// 
    /// `area=min(h[left], h[right])*(right-left)`如果当`h[left] < h[right]`时
    /// 有`area=h[left]*(right-left)`，此时移动`right -= 1`
    /// 
    /// - 如果`h[left] < h[right-1]`，则`min(h[left], h[right-1])=h[left]`
    /// 有`area=h[left]*(right-1-left)`
    /// - 如果`h[left] > h[right-1]`，则`min(h[left], h[right-1])=h[right-1]`
    /// 有`area=h[right-1]*(right-1-left) < area=h[left]*(right-left)`
    /// 
    /// 即`h[left] < h[right]`不管如何移动right都会比当前area小
    /// 
    /// 为何left+1,right-1后不需要再对right=len, left=0开始比较？
    /// 
    /// 由于left与right昰对称的，也就是从left看是`h[left] < h[right]`，移动left，那`h[left] > h[right]`
    /// 就是移动right
    /// 
    /// 对称意味着如果`h[left] > h[right]`时right-1，如果left=0，left=0时最大值已被计算过，无论right是多少
    /// 也不会大于，则从当前left开始
    /// 
    /// 参考：
    /// 
    /// - [盛最多水的容器](https://leetcode-cn.com/problems/container-with-most-water/solution/sheng-zui-duo-shui-de-rong-qi-by-leetcode-solution/)
    /// - [盛最多水的容器（双指针法，易懂解析，图解）](https://leetcode-cn.com/problems/container-with-most-water/solution/container-with-most-water-shuang-zhi-zhen-fa-yi-do/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200816, mem=2.2, mem_beats=33.33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98490187/
    /// 
    /// date=20200817, mem=2.2, mem_beats=45.45, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98878548/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn max_area(height: Vec<i32>) -> i32 {
            let (mut left, mut right) = (0, height.len() - 1);
            let mut max_area = 0;
            while left < right {
                let area = height[left].min(height[right]) * (right - left) as i32;
                max_area = max_area.max(area);
                if height[left] <= height[right] {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
            max_area
        }

        pub fn max_area1(height: Vec<i32>) -> i32 {
            let mut max_area = 0;
            let (mut lo, mut hi) = (0, height.len());
            while lo < hi {
                let area = height[lo].min(height[hi]) * (hi - lo) as i32;
                max_area = max_area.max(area);
                if height[lo] <= height[hi] {
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
            assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
        }
    }
}