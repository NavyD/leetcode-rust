/// 总结
/// 
/// 20200904
/// 
/// 第二次写的时候有点没搞清如何进位
pub mod solution_iterative {
    /// # 思路
    /// 
    /// - 当digit!=9时，+1直接返回 
    /// - 当digit=9时，向前进位1 digits[i-1] += 1;
    /// - 当i-1==0时，插入高位1
    /// 
    /// 如何处理进位
    /// 
    /// 循环做+1，只有当digit>=9时循环。digits[i]=0, 
    /// 在下个循环中处理digit[i-1] +=1
    /// 
    /// ### Submissions
    /// 
    /// date=20200903, mem=2.1, mem_beats=37.50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104312187/
    /// 
    /// date=20200904, mem=2.1, mem_beats=34.38, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104671100/
    /// 
    /// date=20200904, mem=2.1, mem_beats=42.86, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/107842564/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
            for i in (0..digits.len()).rev() {
                if digits[i] != 9 {
                    digits[i] += 1;
                    break;
                }
                digits[i] = 0;
                if i == 0 {
                    digits.insert(0, 1);
                    break;
                }
            }
            digits
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(vec![1,2,4], Solution::plus_one(vec![1,2,3]));
            assert_eq!(vec![1,3,0], Solution::plus_one(vec![1,2,9]));
            assert_eq!(vec![2,0,0], Solution::plus_one(vec![1,9,9]));
            assert_eq!(vec![1,0,0], Solution::plus_one(vec![9,9]));
        }
    }
}