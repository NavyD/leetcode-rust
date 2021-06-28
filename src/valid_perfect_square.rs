pub mod solution_binarysearch {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [层层递进逐步最优的四种解法详解！](https://leetcode-cn.com/problems/valid-perfect-square/solution/ceng-ceng-di-jin-zhu-bu-zui-you-de-si-chong-jie-fa/)
    ///
    /// ### Submissions
    ///
    /// date=20210115, mem=2, mem_beats=80, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138588521/
    ///
    /// date=20210117, mem=2, mem_beats=86, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/139012647/
    ///
    /// date=20210308, mem=2, mem_beats=32, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152602821/
    ///
    /// date=20210526, mem=1.9, mem_beats=46, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180940781/
    pub struct Solution;

    impl Solution {
        pub fn is_perfect_square(num: i32) -> bool {
            let num = num as u64;
            // 1 <= num <= 2^31 - 1
            let (mut lo, mut hi) = (1, num);
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if mid * mid > num {
                    hi = mid - 1;
                } else {
                    lo = mid;
                }
            }
            lo * lo == num
        }
    }
}

pub mod solution_progression {
    /// # 思路
    ///
    /// `1+3+5+7+...(2N−1)=N^2`: 1, 4=1+3, 9=1+3+5, 16=1+3+5+7
    ///
    /// 模仿它可以使用一个while循环，不断减去一个从1开始不断增大的奇数，若最终减成了0，说明是完全平方数，否则，不是
    ///
    /// 参考：
    ///
    /// - [执行用时 : 0 ms , 在所有 C++ 提交中击败了 100.00% 的用户](https://leetcode-cn.com/problems/valid-perfect-square/solution/zhi-xing-yong-shi-0-ms-zai-suo-you-c-ti-jiao-zh-44/)
    /// - [层层递进逐步最优的四种解法详解！](https://leetcode-cn.com/problems/valid-perfect-square/solution/ceng-ceng-di-jin-zhu-bu-zui-you-de-si-chong-jie-fa/)
    ///
    /// ### Submissions
    ///
    /// date=20210115, mem=2, mem_beats=69, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138589573/
    pub struct Solution;

    impl Solution {
        // 1 <= num <= 2^31 - 1
        pub fn is_perfect_square(mut num: i32) -> bool {
            let mut temp = 1;
            while num > 0 {
                num -= temp;
                temp += 2;
            }
            num == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_binarysearch::Solution::is_perfect_square);
        test(solution_progression::Solution::is_perfect_square);
    }

    // 1 <= num <= 2^31 - 1
    fn test<F: Fn(i32) -> bool>(f: F) {
        // assert!(f(0));
        assert!(f(4));
        assert!(!f(8));
        assert!(f(9));
        assert!(f(1));
        assert!(!f(2147395599));
    }
}
