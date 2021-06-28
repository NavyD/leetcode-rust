pub mod solution_greedy {
    /// # 思路
    ///
    /// 有如下三种情况：
    ///
    /// - 账单是5，直接收下。
    /// - 账单是10，消耗一个5，增加一个10
    /// - 账单是20，优先消耗一个10和一个5，如果不够，再消耗三个5
    ///
    /// 贪心：
    ///
    /// 客户付20美元的时候，优先找10美元零钱。
    /// 局部最优：遇到账单20，优先消耗美元10，完成本次找零。全局最优：完成全部账单的找零。
    ///
    /// 参考：
    ///
    /// - [860. 柠檬水找零【都感觉这就是模拟，但为什么是贪心呢】](https://leetcode-cn.com/problems/lemonade-change/solution/860-ning-meng-shui-zhao-ling-du-gan-jue-zpbdp/)
    ///
    /// ### Submissions
    ///
    /// date=20210113, mem=2, mem_beats=87, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138025753/
    ///
    /// date=20210114, mem=2, mem_beats=95, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138296043/
    ///
    /// date=20210127, mem=2, mem_beats=81, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/141525668/
    pub struct Solution;

    impl Solution {
        pub fn lemonade_change(bills: Vec<i32>) -> bool {
            let (mut count5, mut count10) = (0, 0);
            for bill in bills {
                match bill {
                    5 => count5 += 1,
                    10 => {
                        if count5 <= 0 {
                            return false;
                        } else {
                            count5 -= 1;
                            count10 += 1;
                        }
                    }
                    _ => {
                        // 优先消耗10
                        if count10 > 0 && count5 > 0 {
                            count10 -= 1;
                            count5 -= 1;
                        } else if count5 >= 3 {
                            count5 -= 3;
                        } else {
                            return false;
                        }
                    }
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::lemonade_change);
    }

    fn test<F: Fn(Vec<i32>) -> bool>(f: F) {
        assert!(f(vec![5, 5, 5, 10, 20]));
        assert!(f(vec![5, 5, 10]));

        assert!(!f(vec![10, 10]));
        assert!(!f(vec![5, 5, 10, 10, 20]));
    }
}
