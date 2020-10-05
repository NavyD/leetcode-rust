/// 总结
/// 
/// 如何保存关联相同的anagram
pub mod solution_sort {
    /// # 思路
    /// 
    /// 当且仅当它们的排序字符串相等时，两个字符串是字母异位词。
    /// 
    /// 不同的anagram排序后是相同的，使用相同的str作为key关联对应的anagram
    ///
    /// ```rust,ignore
    /// // date=20201004, mem=4.6, mem_beats=62.50, runtime=16, runtime_beats=76.32, url=https://leetcode-cn.com/submissions/detail/113162222/
    /// pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    ///     let mut map = std::collections::HashMap::new();
    ///     for s in strs {
    ///         let mut chars: Vec<char> = s.chars().collect();
    ///         chars.sort();
    ///         let sorted_str = String::from_iter(chars);
    ///         if !map.contains_key(&sorted_str) {
    ///             map.insert(sorted_str.clone(), vec![]);
    ///         }
    ///         if let Some(v) = map.get_mut(&sorted_str) {
    ///             v.push(s);
    ///         }
    ///     }
    ///     let a: Vec<Vec<String>> = map.into_iter().map(|(_, v)| v).collect();
    ///     a
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// - [字母异位词分组](https://leetcode-cn.com/problems/group-anagrams/solution/zi-mu-yi-wei-ci-fen-zu-by-leetcode/)
    /// - [12ms](https://leetcode-cn.com/submissions/api/detail/49/rust/12/)
    /// - [Rust Solution](https://leetcode.com/problems/group-anagrams/discuss/566237/Rust-Solution)
    ///
    /// ### Submissions
    ///
    /// date=20201004, mem=4.6, mem_beats=62.50, runtime=20, runtime_beats=36.84, url=https://leetcode-cn.com/submissions/detail/113177175/
    /// 
    /// date=20201005, mem=4.6, mem_beats=62.50, runtime=12, runtime_beats=97.37, url=https://leetcode-cn.com/submissions/detail/113361476/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(N*K log K)： K 是 strs 中字符串的最大长度
    /// - 空间：O(N*K)
    pub struct Solution;

    impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut anagrams = std::collections::HashMap::new();
            strs.into_iter().for_each(|s| {
                // Use the sorted bytes as the key
                let mut key: Vec<u8> = s.bytes().collect();
                key.sort_unstable();
                // or_insert_with will not create vec every time
                anagrams.entry(key).or_insert_with(|| Vec::new()).push(s);
            });
            anagrams.into_iter().map(|(_, v)| v).collect()
        }
    }
}

pub mod solution_hash {
    /// # 思路
    /// 
    /// 不同的anagram计数后的数组是相同的，使用相同的array作为key关联对应的anagram
    /// 
    /// 参考：
    /// 
    /// - [Rust Solution](https://leetcode.com/problems/group-anagrams/discuss/566237/Rust-Solution)
    /// - [字母异位词分组](https://leetcode-cn.com/problems/group-anagrams/solution/zi-mu-yi-wei-ci-fen-zu-by-leetcode/)
    /// 
    /// ### Submissions
    /// 
    /// date=20201004, mem=6.1, mem_beats=12.50, runtime=12, runtime_beats=97.37, url=https://leetcode-cn.com/submissions/detail/113175459/
    /// 
    /// date=20201005, mem=6.2, mem_beats=12.50, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/113361742/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(NK)
    /// - 空间：O(NK)
    pub struct Solution;

    impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut anagrams = std::collections::HashMap::new();
            strs.into_iter().for_each(|s| {
                // Use the count bytes as the key
                let mut key = [0; 26];
                s.bytes().for_each(|b| key[b as usize - 'a' as usize] += 1);
                anagrams.entry(key).or_insert_with(|| Vec::new()).push(s);
            });
            anagrams.into_iter().map(|(_, v)| v).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_sort::Solution::group_anagrams);
        test(solution_hash::Solution::group_anagrams);
    }

    fn test<F: Fn(Vec<String>) -> Vec<Vec<String>>>(func: F) {
        // let res = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let res = func(
                strs.clone()
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        );
        assert_eq!(3, res.len());
    }
}
