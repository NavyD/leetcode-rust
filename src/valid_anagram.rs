pub mod solution_sort {
    /// # 思路
    ///
    /// ### Submissions
    ///
    /// date=20220624, mem=2.1, mem_beats=90, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool {
            let (mut s, mut t) = (s.into_bytes(), t.into_bytes());
            s.sort_unstable();
            t.sort_unstable();
            s == t
        }
    }
}

pub mod solution_count {
    /// # 思路
    ///
    /// 计算两个字符串中每个字母的出现次数并进行比较。
    ///
    /// 参考：
    ///
    /// * [画解算法：242. 有效的字母异位词](https://leetcode-cn.com/problems/valid-anagram/solution/hua-jie-suan-fa-242-you-xiao-de-zi-mu-yi-wei-ci-by/)
    /// * [有效的字母异位词](https://leetcode.cn/problems/valid-anagram/solution/you-xiao-de-zi-mu-yi-wei-ci-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20200926, mem=2.2, mem_beats=60.87, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/111470082/
    ///
    /// date=20200928, mem=2, mem_beats=95.65, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/112090969/
    ///
    /// date=20201004, mem=2.2, mem_beats=46.43, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/113152313/
    ///
    /// date=20220624, mem=2.1, mem_beats=94, runtime=0, runtime_beats=100
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool {
            #[inline]
            fn index(c: impl Into<usize>) -> usize {
                c.into() - 'a' as usize
            }

            let (s, t) = (s.as_bytes(), t.as_bytes());
            if s.len() != t.len() {
                return false;
            }

            let mut counts = [0; 26];
            for i in 0..t.len() {
                counts[index(t[i])] += 1;
                counts[index(s[i])] -= 1;
            }
            counts.iter().all(|count| *count == 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_count::Solution::is_anagram);
        test(solution_sort::Solution::is_anagram);
    }

    fn test<F: Fn(String, String) -> bool>(func: F) {
        assert!(func("anagram".to_string(), "nagaram".to_string()));
        assert!(!func("rat".to_string(), "car".to_string()));
        assert!(!func("ab".to_string(), "a".to_string()));
    }
}
