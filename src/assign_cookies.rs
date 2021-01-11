pub mod solution_greedy {
    /// # 思路
    /// 
    /// 从贪心的角度考虑，应该按照孩子的胃口从小到大的顺序依次满足每个孩子，且对于每个孩子，
    /// 应该选择可以满足这个孩子的胃口且尺寸最小的饼干
    /// 
    /// 下面这个版本不要更快点4ms
    ///
    /// ```
    /// pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    ///     g.sort_unstable();
    ///     s.sort_unstable();
    ///     let mut count = 0;
    ///     let mut i = 0;
    ///     for cookie in s {
    ///         if i >= g.len() {
    ///             break;
    ///         }
    ///         if cookie >= g[i] {
    ///             i += 1;
    ///             count += 1;
    ///         }
    ///     }
    ///     count
    /// }
    /// assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [排序 + 贪心算法](https://leetcode-cn.com/problems/assign-cookies/solution/fen-fa-bing-gan-by-leetcode-solution-50se/)
    /// 
    /// ### Submissions
    /// 
    /// date=20210111, mem=2.2, mem_beats=70, runtime=8, runtime_beats=10, url=https://leetcode-cn.com/submissions/detail/137634210/
    pub struct Solution;

    impl Solution {
        pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
            g.sort_unstable();
            s.sort_unstable();

            let mut count = 0;
            let mut i = 0;
            let mut j = 0;
            while i < g.len() && j < s.len() {
                if s[j] >= g[i] {
                    j += 1;
                    i += 1;
                    count += 1;
                } else {
                    j += 1;
                }
            }
            count
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::find_content_children);
    }

    fn test<F: Fn(Vec<i32>, Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![1, 2, 3], vec![1, 1]), 1);
        assert_eq!(func(vec![1, 2], vec![1, 2, 3]), 2);
    }
}
