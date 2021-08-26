pub mod solution_dp {
    /// # 思路
    ///
    /// 子问题：对于从左上角到右下角的一维前缀和：`S(O,D)=S(O,C)+S(O,B)−S(O,A)+D`，减去 S(O, A) 的原因是 S(O, C) 和 S(O, B) 中都有 S(O, A)，
    /// 即加了两次 S(O, A)，所以需要减去一次 S(O, A)。如下图：
    ///
    /// ![](https://pic.leetcode-cn.com/1614646493-EriDmE-304.001.jpeg)
    ///
    /// dp方程：定义 `dp[i][j]` 表示 从 `[0,0]` 位置到 `[i,j]` 位置的子矩形所有元素之和：
    /// `dp[i][j]=dp[i−1][j]+dp[i][j−1]−dp[i−1][j−1]+matrix[i][j]`
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
    #[embed_doc_image::embed_doc_image("img", "docs/images/2021-08-26-1119.png")]
    pub struct Solution;

    impl Solution {
        pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
            let (rows, cols) = (matrix.len(), matrix[0].len());
            let mut res = i32::MIN;

            let mut dp = vec![vec![0; cols + 1]; rows + 1];
            // 1. pre sum in 0,0 -> i,j
            for i in 1..=rows {
                for j in 1..=cols {
                    dp[i][j] =
                        dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + matrix[i - 1][j - 1];
                }
            }

            // 2. area sum in i,j -> p,q
            for i in 1..=rows {
                for j in 1..=cols {
                    for p in i..=rows {
                        for q in j..=cols {
                            let area_sum =
                                dp[p][q] - dp[p][j - 1] - dp[i - 1][q] + dp[i - 1][j - 1];
                            if area_sum <= k && area_sum > res {
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
        test(solution_dp::Solution::max_sum_submatrix);
    }

    fn test<F: Fn(Vec<Vec<i32>>, i32) -> i32>(f: F) {
        assert_eq!(f(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
        assert_eq!(f(vec![vec![2, 2, -1]], 3), 3);
        assert_eq!(f(vec![vec![2, 2, -1]], 0), -1);
        assert_eq!(
            f(
                vec![vec![5, -4, -3, 4], vec![-3, -4, 4, 5], vec![5, 1, 5, -4]],
                10
            ),
            10
        );
    }
}
