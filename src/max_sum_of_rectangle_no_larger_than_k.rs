pub mod solution_dp_presum {
    /// # 思路
    ///
    /// 子问题：对于从左上角到右下角的一维前缀和：`S(O,D)=S(O,C)+S(O,B)−S(O,A)+D`，减去 S(O, A) 的原因是 S(O, C) 和 S(O, B) 中都有 S(O, A)，
    /// 即加了两次 S(O, A)，所以需要减去一次 S(O, A)。如下图：
    ///
    /// ![](https://pic.leetcode-cn.com/1614646493-EriDmE-304.001.jpeg)
    ///
    /// 一旦找到了区域和S(O,D)，就可以枚举出任意区域和
    ///
    /// dp方程：定义 `dp[i][j]` 表示 从 `[0,0]` 位置到 `[i,j]` 位置区域所有元素之和：
    /// `dp[i][j]=dp[i−1][j]+dp[i][j−1]−dp[i−1][j−1]+matrix[i][j] for i in 1..=rows: for j in 1..=cols`
    ///
    /// 根据前缀和`dp[i][j]`可以从任意一点(i,j)作为左上角开始以(i,j)右下方的点(p,q)形成的矩形面积，可以枚举每个点作为左
    /// 上角，搜索位于 (i,j) 的右下方的点 (p,q) 作为右下角，则有`area_sum = dp[p][q] - dp[i - 1][q] - dp[p][j - 1] + dp[i - 1][j - 1]`
    /// 如图：
    ///
    /// ![][img]
    ///
    /// 初始化：预处理前缀和dp，枚举(i,j)下面的(p,q)找最大的area_sum
    ///
    /// 参考：
    ///
    /// - [【宫水三叶】优化枚举的基本思路 & 将二维抽象成一维 & 最大化「二分」效益 & 空间优化](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/solution/gong-shui-san-xie-you-hua-mei-ju-de-ji-b-dh8s/)
    /// - [如何求二维的前缀和，以及用前缀和求子矩形的面积](https://leetcode-cn.com/problems/range-sum-query-2d-immutable/solution/ru-he-qiu-er-wei-de-qian-zhui-he-yi-ji-y-6c21/)
    /// - [【宫水三叶】下次如何在 30 秒内做出来？二维前缀和模板如何记忆](https://leetcode-cn.com/problems/range-sum-query-2d-immutable/solution/xia-ci-ru-he-zai-30-miao-nei-zuo-chu-lai-ptlo/)
    /// - [Java，从暴力开始优化，配图配注释](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/solution/javacong-bao-li-kai-shi-you-hua-pei-tu-pei-zhu-shi/)
    ///
    /// ### Submissions
    ///
    /// date=20210826, mem=2.2, mem_beats=100, runtime=344, runtime_beats=50
    ///
    /// date=20210830, mem=2, mem_beats=100, runtime=344, runtime_beats=28
    ///
    /// date=20210916, mem=2.3, mem_beats=6, runtime=300, runtime_beats=26
    #[embed_doc_image::embed_doc_image("img", "docs/images/2021-08-26-1119.png")]
    pub struct Solution;

    impl Solution {
        pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut res = i32::MIN;

            let mut presums = vec![vec![0; cols + 1]; rows + 1];
            // 1. pre sum in 0,0 -> i,j
            for i in 1..=rows {
                for j in 1..=cols {
                    presums[i][j] = presums[i - 1][j] + presums[i][j - 1] - presums[i - 1][j - 1]
                        + matrix[i - 1][j - 1];
                }
            }

            // 2. area sum in i,j -> p,q
            for i in 1..=rows {
                for j in 1..=cols {
                    for p in i..=rows {
                        for q in j..=cols {
                            let area_sum = presums[p][q] - presums[p][j - 1] - presums[i - 1][q]
                                + presums[i - 1][j - 1];
                            if area_sum <= k && res < area_sum {
                                res = area_sum;
                            }
                        }
                    }
                }
            }
            res
        }
    }
}

pub mod solution_dp_binary {
    /// # 思路
    ///
    /// 在前缀和的基础上，枚举固定上下边界，以(i, 0)左边起点，1..right为右边界的area sum有序保存入tree set，这样
    /// 只要当前`area_sum - tree_set[i]`任意一个就是一个以right-1..right的小矩阵和，把之前的面积保存
    /// 作为之后的减数。
    /// 利用tree set的有序性使用二分法可以快速找到当前area sum下最大的小矩阵和
    ///
    /// ![](https://pic.leetcode-cn.com/1618975243-AnNcYI-439B50D739F1D963EB2460394C5689B5.png)
    ///
    /// 向set中插入0是因为有可能不用减去子矩阵左边列与原矩阵左边列形成的子矩阵和
    ///
    /// 参考：
    ///
    /// - [【宫水三叶】优化枚举的基本思路 & 将二维抽象成一维 & 最大化「二分」效益 & 空间优化 前缀和 & 二分（抽象一维）](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/solution/gong-shui-san-xie-you-hua-mei-ju-de-ji-b-dh8s/)
    ///
    /// ### Submissions
    ///
    /// date=20210827, mem=2.2, mem_beats=50, runtime=252, runtime_beats=50
    ///
    /// date=20210830, mem=2.2, mem_beats=14, runtime=248, runtime_beats=71
    ///
    /// date=20210916, mem=2.3, mem_beats=6, runtime=216, runtime_beats=66
    pub struct Solution;

    impl Solution {
        pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut res = i32::MIN;

            // 1. pre sum
            let mut presums = vec![vec![0; cols + 1]; rows + 1];
            for i in 1..=rows {
                for j in 1..=cols {
                    presums[i][j] = presums[i - 1][j] + presums[i][j - 1] - presums[i - 1][j - 1]
                        + matrix[i - 1][j - 1];
                }
            }

            // 2. enumerate up, down, right
            for up in 1..=rows {
                for down in up..=rows {
                    // 3. find the area sum from the column 0 to the current right column
                    let mut sums = std::collections::BTreeSet::new();
                    sums.insert(0);
                    for i in 1..=cols {
                        // sum of 0..right col
                        let sum = presums[down][i] - presums[up - 1][i];
                        // 4. max left sum: sum - sum_left <= k -> sum_left >= sum - k
                        if let Some(sum_left) = sums.range(sum - k..).next() {
                            // area sum
                            res = res.max(sum - sum_left);
                        }
                        sums.insert(sum);
                    }
                }
            }
            res
        }
    }
}

pub mod solution_dp_binary_optimized {
    /// # 思路
    ///
    /// 最大化「二分」效益：先枚举的是「上下行」和「右边列」，然后通过 TreeSet 来「二分」出符合条件的「左边列」,
    /// 将「二分过程」应用到数值较大的行或者列之中，这样才能最大化我们查找的效率
    ///
    /// 空间优化：`sum[fixed]`表示以i为上界，j为下界，fixed为固定这一列的和，`sum[fixed]`仅表示一列的和，所以要用
    /// `a+=sum[fixed]`表示以fixed列为右边界的矩阵和
    ///
    /// ```java
    /// for (int i = 1; i <= (isRight ? m : n); i++) {
    ///     for (int j = i; j <= (isRight ? m : n); j++) {
    ///         int a = 0;
    ///         for (int fixed = 1; fixed <= (isRight ? n : m); fixed++) {
    ///             sum[fixed] += isRight ? mat[j - 1][fixed - 1] : mat[fixed - 1][j - 1] ;
    ///             a += sum[fixed];
    ///             Integer b = ts.ceiling(a - k);
    ///     }
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [【宫水三叶】优化枚举的基本思路 & 将二维抽象成一维 & 最大化「二分」效益 & 空间优化 前缀和 & 二分（抽象一维）](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/solution/gong-shui-san-xie-you-hua-mei-ju-de-ji-b-dh8s/)
    ///
    /// ### Submissions
    ///
    /// date=20210827, mem=2.1, mem_beats=100, runtime=256, runtime_beats=50
    ///
    /// date=20210830, mem=2, mem_beats=100, runtime=252, runtime_beats=71
    ///
    /// date=20210916, mem=2.2, mem_beats=33, runtime=112, runtime_beats=80
    pub struct Solution;

    impl Solution {
        pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let mut res = i32::MIN;
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let rotated = rows < cols;
            let (rows, cols) = if rotated { (cols, rows) } else { (rows, cols) };

            // 2. enumerate up, down, col
            for up in 0..rows {
                // col_sums[i] is the sum of column 0..col in rows up to down
                let mut col_sums = vec![0; cols];
                for down in up..rows {
                    // 3. find the area sum from the column 0..col
                    let mut sums = std::collections::BTreeSet::new();
                    sums.insert(0);
                    // all area sum of (up, 0) -> (down, i for col)
                    let mut sum = 0;
                    for i in 0..cols {
                        col_sums[i] += if rotated {
                            matrix[i][down]
                        } else {
                            matrix[down][i]
                        };
                        sum += col_sums[i];

                        // 4. max area sum
                        if let Some(left_sum) = sums.range(sum - k..).next() {
                            let area_sum = sum - left_sum;
                            // early return
                            if area_sum == k {
                                return k;
                            }
                            res = res.max(area_sum);
                        }
                        sums.insert(sum);
                    }
                }
            }
            res
        }
    }
}

pub mod solution_dp_row_sums {
    /// # 思路
    ///
    /// 以左右为边界找中间的行的和`row_sums[i]`，前缀和可以由`row_sums[i]`累加得到，区域和
    /// 由任一行开始`row_sums[i] + row_sums[i+1] +...`。在没有优化代码时与binary是一个思路
    ///
    /// ![](https://pic.leetcode-cn.com/b02979492d31c6b8e2e365d2efbd64ea485f69a32055661397c5849d3bd91251-image.png)
    ///
    /// dp方程：`row_sums[i] += matrix[i][right] for left in 0..cols: for right in left..cols: for i in 0..rows`
    ///
    /// 如何找到<=k的最大值：枚举子数组起点、终点，累计中间元素找出区域和。
    ///
    /// 优化：并不是所有时候都值得遍历找 k [maximum_subarray][crate::maximum_subarray::solution_dp::Solution]
    ///
    /// 参考：
    ///
    /// - [Java，从暴力开始优化，配图配注释 三、数组滚动](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/solution/javacong-bao-li-kai-shi-you-hua-pei-tu-pei-zhu-shi/)
    /// - [画解算法：53. 最大子序和](https://leetcode-cn.com/problems/maximum-subarray/solution/hua-jie-suan-fa-53-zui-da-zi-xu-he-by-guanpengchn/)
    ///
    /// ### Submissions
    ///
    /// date=20210830, mem=2.2, mem_beats=14, runtime=52, runtime_beats=100
    ///
    /// date=20210902, mem=2, mem_beats=100, runtime=80, runtime_beats=91
    ///
    /// date=20210916, mem=2.2, mem_beats=100, runtime=48, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut res = i32::MIN;

            for left in 0..cols {
                let mut row_sums = vec![0; rows];
                for right in left..cols {
                    for i in 0..rows {
                        row_sums[i] += matrix[i][right];
                    }

                    // It is valid when k >= the sum of the max sub array
                    {
                        // 要尽量大，就尽量不要负数
                        let (mut area_subsum, mut max_sum) = (row_sums[0], row_sums[0]);
                        for i in 1..rows {
                            if area_subsum > 0 {
                                area_subsum += row_sums[i];
                            } else {
                                // 之前的和小于0 重新开始
                                area_subsum = row_sums[i];
                            }
                            if area_subsum == k {
                                return k;
                            }
                            max_sum = max_sum.max(area_subsum);
                        }
                        if max_sum <= k {
                            res = res.max(max_sum);
                            continue;
                        }
                    }

                    // Find the largest area under the current column
                    for up in 0..rows {
                        // 从up..rows开始的区域和
                        let mut area_sum = 0;
                        for down in up..rows {
                            area_sum += row_sums[down];
                            if area_sum == k {
                                return k;
                            } else if area_sum < k && res < area_sum {
                                res = area_sum;
                            }
                        }
                    }
                }
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
        test(solution_dp_presum::Solution::max_sum_submatrix);
        test(solution_dp_binary::Solution::max_sum_submatrix);
        test(solution_dp_binary_optimized::Solution::max_sum_submatrix);
        test(solution_dp_row_sums::Solution::max_sum_submatrix);
    }

    fn test<F: Fn(Vec<Vec<i32>>, i32) -> i32>(f: F) {
        assert_eq!(f(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
        assert_eq!(f(vec![vec![2, 2, -1]], 3), 3);
        assert_eq!(f(vec![vec![2, 2, -1]], 0), -1);
        assert_eq!(
            f(
                vec![vec![5, -4, -3, 4], vec![-3, -4, 4, 5], vec![5, 1, 5, -4]],
                -2
            ),
            -2
        );
        assert_eq!(
            f(
                vec![vec![5, -4, -3, 4], vec![-3, -4, 4, 5], vec![5, 1, 5, -4]],
                10
            ),
            10
        );
    }
}
