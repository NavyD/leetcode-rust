/// ```ignore
/// pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
///     let (mut lo_row, mut hi_row) = (0, matrix.len() - 1);
///     let (mut lo_col, mut hi_col) = (0, matrix[hi_row].len() - 1);
///     while lo_row < hi_row && lo_col < hi_col {
///         let mid_row = (lo_row + hi_row) / 2;
///         let mid_col = (lo_col + hi_col) / 2;
///         if target > matrix[mid_row][mid_col] {
///             lo_row = mid_row + 1;
///             lo_col = mid_col + 1;
///         } else {
///             hi_row = mid_row;
///             hi_col = mid_col;
///         }
///     }
///     matrix[lo_row][hi_row] == target
/// }
/// ```

pub mod solution_binarysearch {
    /// # 思路
    /// 
    /// 转成一个m*n长度的有序数组。访问行下标`(m*n - 1)/n`，列下标：(m*n - 1)%n. m*n只是放大了n，还保存了n的关系，
    /// 可以通过%n找出
    /// 
    /// 注意：在对(lo + hi) + 1还是-1上要与条件对应
    /// 
    /// lo要与没有判断==的mid：`lo=mid`如：`if val > target {hi = mid-1} else {lo = mid}`。这里val没有判断与
    /// mid相等下一次要包含lo=mid判断。`if val < target {lo = mid + 1} else {hi = mid}`，这里判断了lo位置，
    /// 下次要+1。即`lo|hi`要包含未判断的位置
    /// 
    /// 同时要注意`lo=mid`时必有`let mid = (lo + hi + 1) / 2`，如果`mid=(lo + hi)/2`时存在`vals[mid] < target`
    /// 导致在lo=0,hi=1,时mid=0无限循环，所以要用`let mid = (lo + hi + 1) / 2`
    /// 
    /// rust的usize-1 `mid-1`可能出现overflow问题，这种模板是比较合适的
    /// 
    /// 参考：
    /// 
    /// * [搜索二维矩阵](https://leetcode-cn.com/problems/search-a-2d-matrix/solution/sou-suo-er-wei-ju-zhen-by-leetcode/)
    /// 
    /// ### Submissions
    /// 
    /// date=20210119, mem=2, mem_beats=60, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139500665/
    /// 
    /// date=20210120, mem=2.1, mem_beats=20, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139739469/
    pub struct Solution;

    impl Solution {
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            let (m, n) = (matrix.len(), matrix[0].len());
            let (mut lo, mut hi) = (0, m * n - 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if matrix[mid / n][mid % n] < target { 
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            matrix[lo / n][lo % n] == target
        }
    }
}

pub mod solution_binarysearch_row_col {
    /// # 思路
    /// 
    /// 注意：在找row的mid时，要使用` (lo + hi + 1) / 2`和` matrix[mid][0]`比较，也可以使用
    /// `(lo + hi) / 2`和`matrix[mid][n-1]`
    /// 
    /// 参考：
    /// 
    /// * [C++ 两个二分法解决！](https://leetcode-cn.com/problems/search-a-2d-matrix/solution/c-liang-ge-er-fen-fa-jie-jue-by-mecury/)
    /// * [Don't treat it as a 2D matrix, just treat it as a sorted list](https://leetcode.com/problems/search-a-2d-matrix/discuss/26220/Don't-treat-it-as-a-2D-matrix-just-treat-it-as-a-sorted-list/178290)
    /// 
    /// ### Submissions
    /// 
    /// date=20210119, mem=2.1, mem_beats=20, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139508154/
    /// 
    /// date=20210120, mem=2, mem_beats=60, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139741684/
    pub struct Solution;

    impl Solution {
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            let (m, n) = (matrix.len(), matrix[0].len());
            // 查找行数
            let (mut lo, mut hi) = (0, m - 1);
            while lo < hi {
                // 向上取整
                let mid = (lo + hi + 1) / 2;
                if matrix[mid][0] > target {
                    hi = mid - 1;
                } else {
                    lo = mid;
                }
            }
            let mid_row = lo;
            // 查找列数
            let (mut lo, mut hi) = (0, n - 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if matrix[mid_row][mid] < target {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            matrix[mid_row][lo] == target
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_binarysearch::Solution::search_matrix);
        test(solution_binarysearch_row_col::Solution::search_matrix);
    }

    fn test<F: Fn(Vec<Vec<i32>>, i32) -> bool>(f: F) {
        assert!(f(
            [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]]
                .iter()
                .map(|a| a.to_vec())
                .collect(),
            3
        ));
        assert!(!f(
            [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]]
                .iter()
                .map(|a| a.to_vec())
                .collect(),
            13
        ));
        assert!(!f([[1]].iter().map(|a| a.to_vec()).collect(), 0));
    }
}
