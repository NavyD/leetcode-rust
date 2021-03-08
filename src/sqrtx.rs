pub mod solution_binarysearch {
    /// # 思路
    /// 
    /// 注意：
    /// 
    /// 确定二分法的上界时，x的平方根一定不会超过x自己，而且一个数的平方根最多不会超过它的一半
    /// 如8 的一半是 4，4^2=16 >8。当x<4时0,1,2,3，x的平方根为0,1,1,1。
    /// 
    /// - 如果left从0开始，当x=1时有0<1/2不成立，直接返回0，应该从left=1开始，先过滤0。
    /// - 或将right置为x/2 + 1
    /// 
    /// ```ignore
    /// // 先过滤0
    /// if x == 0 {
    ///     return 0;
    /// }
    /// // 从1开始
    /// let mut left = 1;
    /// let mut right = x / 2;
    /// 
    /// let mut left = 0;
    /// let mut right = x / 2 + 1;
    /// while left < right {
    ///     // ..
    /// }
    /// ```
    /// 
    /// 在取mid时应该使用右中位数`let mid = left + (right - left + 1) / 2`，如果
    /// 使用左中位数`let mid = left + (right - left) / 2`，在只有2个元素时取了第1个，
    /// 可能导致死循环。如当x=9,当left=3,right=4时，取mid=3，使得square=3*3=9 == x =>left=3
    /// 无法退出
    /// 
    /// 参考：
    /// 
    /// - [二分查找 + 牛顿法（Python 代码、Java 代码）](https://leetcode-cn.com/problems/sqrtx/solution/er-fen-cha-zhao-niu-dun-fa-python-dai-ma-by-liweiw/)
    /// - [解释：特别好用的二分查找法模板](https://www.liwei.party/2019/06/17/leetcode-solution-new/search-insert-position/)
    /// 
    /// ### Submissions
    /// 
    /// date=20210115, mem=1.9, mem_beats=90, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138583963/
    /// 
    /// date=20210117, mem=1.9, mem_beats=68, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139011065/
    /// 
    /// date=20210308, mem=1.9, mem_beats=65, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152596842/
    pub struct Solution;

    impl Solution {
        pub fn my_sqrt(x: i32) -> i32 {
            let x = x as u64;
            let mut lo = 0;
            // 上界
            let mut hi = x / 2 + 1;
            while lo < hi {
                // 取右中位数
                let mid = (lo + hi + 1) / 2;
                let square = (mid * mid) as u64;
                if square > x  {
                    // [left, mid - 1]
                    hi = mid - 1;
                } else {
                    // [mid, right]
                    lo = mid;
                }
            }
            lo as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_binarysearch::Solution::my_sqrt);
    }

    fn test<F: Fn(i32) -> i32>(f: F) {
        assert_eq!(f(0), 0);
        assert_eq!(f(4), 2);
        assert_eq!(f(8), 2);
        assert_eq!(f(9), 3);
        assert_eq!(f(1), 1);
        assert_eq!(f(2147395599), 46339);
    }
}
