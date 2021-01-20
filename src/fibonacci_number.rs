pub mod solution_recursive {
    pub struct Solution;

    impl Solution {
        pub fn fib(n: i32) -> i32 {
            if n < 2 {
                n
            } else {
                Self::fib(n - 1) + Self::fib(n - 2)
            }
        }
    }
}

pub mod solution_dp {
    pub struct Solution;

    impl Solution {
        pub fn fib(n: i32) -> i32 {
            if n < 2 {
                return n;
            }
            let n = n as usize;
            let mut dp = vec![0; n + 1];
            dp[1] = 1;
            for i in 2..=n {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            dp[n]
        }
    }
}

pub mod solution_dp_optimized {
    pub struct Solution;

    impl Solution {
        pub fn fib(n: i32) -> i32 {
            if n < 2 {
                return n;
            }
            let (mut pre, mut cur) = (0, 1);
            for _ in 2..=n {
                let sum = pre + cur;
                pre = cur;
                cur = sum;
            }
            cur
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_recursive::Solution::fib);
        test(solution_dp::Solution::fib);
        test(solution_dp_optimized::Solution::fib);
    }

    fn test<F: Fn(i32) -> i32>(f: F) {
        assert_eq!(f(2), 1);
        assert_eq!(f(3), 2);
        assert_eq!(f(4), 3);
    }
}
