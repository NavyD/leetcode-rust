pub mod solution_dp {
    pub struct Solution;

    impl Solution {
        /// # 思路
        ///
        /// 啥叫编辑距离？我们说word1和word2的编辑距离为X，意味着word1经过X步，变成了word2，咋变的你不用管，反正知道就需要X步，并且这是个最少的步数。
        /// 我们有word1和word2，我们定义dp[i][j]的含义为：word1的前i个字符和word2的前j个字符的编辑距离。意思就是word1的前i个字符，变成word2的前j个字符，最少需要这么多步。
        ///
        /// 例如word1 = "horse", word2 = "ros"，那么dp[3][2]=X就表示"hor"和“ro”的编辑距离，即把"hor"变成“ro”最少需要X步。
        ///
        /// 如果下标为零则表示空串，比如：dp[0][2]就表示空串""和“ro”的编辑距离
        ///
        /// 定理一：如果其中一个字符串是空串，那么编辑距离是另一个字符串的长度。比如空串“”和“ro”的编辑距离是2（做两次“插入”操作）。再比如"hor"和空串“”的编辑距离是3（做三次“删除”操作）。
        ///
        /// 定理二：当i>0,j>0时（即两个串都不空时）dp[i][j]=min(dp[i-1][j]+1,dp[i][j-1]+1,dp[i-1][j-1]+int(word1[i]!=word2[j]))。
        ///
        /// 啥意思呢？举个例子，word1 = "abcde", word2 = "fgh",我们现在算这俩字符串的编辑距离，就是找从word1，最少多少步，能变成word2？那就有三种方式：
        ///
        /// * 知道"abcd"变成"fgh"多少步（假设X步），那么从"abcde"到"fgh"就是"abcde"->"abcd"->"fgh"。（一次删除，加X步，总共X+1步）
        /// * 知道"abcde"变成“fg”多少步（假设Y步），那么从"abcde"到"fgh"就是"abcde"->"fg"->"fgh"。（先Y步，再一次添加，加X步，总共Y+1步）
        /// * 知道"abcd"变成“fg”多少步（假设Z步），那么从"abcde"到"fgh"就是"abcde"->"fge"->"fgh"。（先不管最后一个字符，把前面的先变好，用了Z步，然后把最后一个字符给替换了。这里如果最后一个字符碰巧就一样，那就不用替换，省了一步）
        ///
        /// dp方程：dp[i][j] = dp[i-1][j-1] if word1[i-1]==word2[j-1] else min(dp[i][j-1]+1, dp[i-1][j]+1, dp[i-1][j-1]+1)`
        ///
        /// 初始化：当word其中一个长度为0时非空字符串的长度就是编辑距离`dp[0][i]=i, dp[i][0]=i`
        ///
        /// 参考：
        ///
        /// * [动态规划（Java，最后有同类问题列表）](https://leetcode-cn.com/problems/edit-distance/solution/dong-tai-gui-hua-java-by-liweiwei1419/)
        /// * [C++ O(n)-space DP](https://leetcode.com/problems/edit-distance/discuss/25846/C%2B%2B-O(n)-space-DP)
        /// * [编辑距离](https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/)
        /// * [编辑距离 评论解析](https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/331399)
        ///
        /// ### Submissions
        ///
        /// date=20210706, mem=3.9, mem_beats=32, runtime=4, runtime_beats=70, url=https://leetcode-cn.com/submissions/detail/192854291/
        ///
        /// date=20210709, mem=4, mem_beats=6, runtime=4, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/193860989/
        ///
        /// date=20210714, mem=3.8, mem_beats=37, runtime=4, runtime_beats=74, url=https://leetcode-cn.com/submissions/detail/195593127/
        pub fn min_distance(word1: String, word2: String) -> i32 {
            let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
            let (len1, len2) = (word1.len(), word2.len());

            let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
            (1..=len1).for_each(|i| dp[i][0] = i);
            (1..=len2).for_each(|i| dp[0][i] = i);

            for i in 1..=len1 {
                for j in 1..=len2 {
                    dp[i][j] = if word1[i - 1] == word2[j - 1] {
                        dp[i - 1][j - 1]
                    } else {
                        dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1
                    };
                }
            }
            dp[len1][len2] as i32
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// 由定理二可知，dp[i][j]只和dp[i-1][j],dp[i][j-1],dp[i-1][j-1]三个量有关，即二维数组中，当前元素的左边，上边，左上角三个元素。
    ///
    /// 那我们不用这么大的二维数组存啊！我们就用一维数组，表示原来二维数组中的一行，然后我们就反复覆盖里面的值。
    /// dp[i-1][j]就是我当前左边的元素，dp[i][j-1]是没覆盖前我这里的值，dp[i-1][j-1]好像找不见了？那我们就单独用一个变量pre存着它，
    /// 再初始化dp[i][0]=i
    ///
    /// 参考：
    ///
    /// * [C++ O(n)-space DP](https://leetcode.com/problems/edit-distance/discuss/25846/C%2B%2B-O(n)-space-DP)
    /// * [编辑距离 评论解析](https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/331399)
    ///
    /// ### Submissions
    ///
    /// date=20210709, mem=2, mem_beats=87, runtime=4, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/193870878/
    ///
    /// date=20210714, mem=1.9, mem_beats=94, runtime=4, runtime_beats=74, url=https://leetcode-cn.com/submissions/detail/195598575/
    pub struct Solution;

    impl Solution {
        pub fn min_distance(word1: String, word2: String) -> i32 {
            let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
            let (len1, len2) = (word1.len(), word2.len());

            let mut dp = vec![0; len2 + 1];
            (1..=len2).for_each(|i| dp[i] = i);

            for i in 1..=len1 {
                let mut pre = dp[0];
                dp[0] = i;
                for j in 1..=len2 {
                    let temp = dp[j];
                    dp[j] = if word1[i - 1] == word2[j - 1] {
                        pre
                    } else {
                        dp[j].min(dp[j - 1]).min(pre) + 1
                    };
                    pre = temp;
                }
            }
            dp[len2] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::min_distance);
        test(solution_dp_optimized::Solution::min_distance);
    }

    fn test<F: Fn(String, String) -> i32>(f: F) {
        assert_eq!(f("horse".into(), "ros".into()), 3);
        assert_eq!(f("intention".into(), "execution".into()), 5);
        assert_eq!(f("".into(), "a".into()), 1);
    }
}
