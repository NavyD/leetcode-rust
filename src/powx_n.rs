//! 如何快速的计算出pow。O(n)的算法不能通过测试。
//!
//! 可以考虑分治思想，如要计算x^9 => x^4*x^4*x => 变成计算x^4，x^4 => x^2*x^2

/// 总结：
///
/// 注意i32::MIN时的越界问题。当n=-2147483648时，-n as u32是正确的
/// 但是当n = -1时 -n as u32 就增加了2^31最高位，可以将 n+1减少1变成
/// -2147483647变成正数-(n + 1) = 2147483647，再加1变成2147483648.
/// 如果n=-1时 -(n+1)=0 加1成1没有问题
///
/// 当n<0时 `-(n + 1) as u32 - 1`
pub mod solution_recursive {
    /// # 思路
    ///
    /// 快速幂算法
    ///
    /// 如果要计算`x^16`: `x^0 => x^2 => x^4 => x^8 => x^16`
    ///
    /// 如果要计算`x^15`: `x^0 => x^1 => x^3 => x^7 => x^15`。对于x的奇数次方，当前结果=(上次的结果 ^2) *x
    ///
    /// 直接从左到右进行推导看上去很困难，因为在每一步中，我们不知道在将上一次的结果平方之后，还需不需要额外乘 xx。
    /// 但如果我们从右往左看，分治的思想就十分明显了
    ///
    /// - 当我们要计算 x^n时，我们可以先递归地计算出 y = x^floor(n/2)
    /// - 如果y为偶数，则x^n = y^2。如果为奇数则x^n = y^2 * x
    /// - 当n == 0时退出
    ///
    /// 参考：
    ///
    /// [Pow(x, n)](https://leetcode-cn.com/problems/powx-n/solution/powx-n-by-leetcode-solution/)
    ///
    ///
    /// 下面是一个最初的版本：复杂度是O(log n)
    ///
    /// ```ignore
    /// pub fn my_pow(x: f64, n: i32) -> f64 {
    ///     fn _pow(x: f64, n: u32) -> f64 {
    ///         let mut res = 1 as f64;
    ///         for _ in 0..n {
    ///             res *= x;
    ///         }
    ///         res
    ///     }
    ///     if n < 0 {
    ///         let n = -n as u32;
    ///         1.0/_pow(x, n)
    ///     } else {
    ///         _pow(x, n as u32)
    ///     }
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20201213, mem_beats=38, mem=2, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/130726928/
    ///
    /// date=20201215, mem_beats=33, mem=2, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/131203027/
    ///
    /// date=20201223, mem_beats=33, mem=2, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133196389/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(log n)
    /// - 空间：O(log n)
    pub struct Solution;
    impl Solution {
        pub fn my_pow(x: f64, n: i32) -> f64 {
            fn _quick_pow(x: f64, n: u32) -> f64 {
                if n == 0 {
                    1.0
                } else {
                    let sub_pow = _quick_pow(x, n / 2);
                    if n & 1 == 1 {
                        sub_pow * sub_pow * x
                    } else {
                        sub_pow * sub_pow
                    }
                }
            }
            if n < 0 {
                1.0 / _quick_pow(x, -(n + 1) as u32 + 1)
            } else {
                _quick_pow(x, n as u32)
            }
        }
    }
}

pub mod solution_iterative {
    /// # 思路
    ///
    /// 对于x^13 => binary: 13 = 1101, x^(2^3) * x^(2^2) * x^(2^0)
    /// 对于x^15 => binary: 13 = 1111, x^(2^3) * x^(2^2) * x^(2^1) * x^(2^0)
    ///
    /// 对x^n中n的二进制的1出现时，将不断累积的x_accumulation计入res中
    ///
    /// 参考：
    ///
    /// - [Iterative Log(N) solution with Clear Explanation](https://leetcode.com/problems/powx-n/discuss/19563/Iterative-Log(N)-solution-with-Clear-Explanation)
    /// - [Pow(x, n)](https://leetcode-cn.com/problems/powx-n/solution/powx-n-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20201213, mem=2, mem_beats=47, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/130735989/
    ///
    /// date=20201215, mem=1.9, mem_beats=72, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/131208050/
    ///
    /// date=20201223, mem=1.9, mem_beats=81, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/133203133/
    pub struct Solution;
    impl Solution {
        pub fn my_pow(x: f64, n: i32) -> f64 {
            fn _quick_pow(mut x: f64, mut n: u32) -> f64 {
                let mut res = 1.0;
                while n > 0 {
                    if n & 1 == 1 {
                        res *= x;
                    }
                    n >>= 1;
                    x *= x;
                }
                res
            }
            if n < 0 {
                1.0 / _quick_pow(x, -(n + 1) as u32 + 1)
            } else {
                _quick_pow(x, n as u32)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_recursive::Solution::my_pow);
        test(solution_iterative::Solution::my_pow);
    }

    fn test<F: Fn(f64, i32) -> f64>(func: F) {
        let error_margin = f64::EPSILON;
        assert!((func(2.00000, 10) - 1024.00000).abs() < error_margin);
        assert!((func(2.0, -1) - 0.5).abs() < error_margin);
        assert!((func(0.00001, 2147483647) - 0.0).abs() < error_margin);
        assert!((func(2.0, -2147483648) - 0.0).abs() < error_margin)
    }
}
