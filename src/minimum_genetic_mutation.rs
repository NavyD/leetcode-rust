//! 起始基因如何变化到目标基因序列
//!
//! 每次只能改变一个字符:"A", "C", "G", "T",并且改变后的
//! 基因都要属于在基因库中.当改变后的基因==目标基因时结束统计
//! 改变的次数

pub mod solution_dfs {
    /// # 思路
    /// 
    /// dfs
    ///
    /// 参考:
    ///
    /// - [Java DFS （回溯）](https://leetcode-cn.com/problems/minimum-genetic-mutation/solution/java-dfs-hui-su-by-1yx/)
    ///
    /// ### Submissions
    ///
    /// date=20201221, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132618271/
    pub struct Solution;

    impl Solution {
        pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
            use std::collections::HashSet;
            fn _backtrack<'a>(
                bank: &'a Vec<String>,
                path: &mut HashSet<&'a str>,
                cur: &str,
                end: &str,
                count: i32,
                min_count: &mut i32,
            ) {
                if count >= *min_count {
                    return;
                }
                if cur == end {
                    *min_count = count;
                    return;
                }
                for gene in bank {
                    // look for gene that only changes once
                    let diff = gene
                        .bytes()
                        .enumerate()
                        .filter(|(i, b)| *b != cur.as_bytes()[*i])
                        .count();
                    let gene = gene.as_str();
                    // the current gene must be uncounted
                    if diff == 1 && !path.contains(&gene) {
                        path.insert(gene);
                        // next
                        _backtrack(bank, path, gene, end, count + 1, min_count);
                        path.remove(gene);
                    }
                }
            }
            let mut min_count = std::i32::MAX;
            _backtrack(&bank, &mut HashSet::new(), &start, &end, 0, &mut min_count);
            if min_count == std::i32::MAX {
                -1
            } else {
                min_count
            }
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    /// 
    /// bfs
    /// 
    /// 注意：当新的new_gene被包含在bank中时,必须要删除防止在下次出现同样的new_gene时再次
    /// 被queue保存,不能对同一个结果多次统计,会出现无限循环.因为在old与genetic_bases
    /// 是在其中一个相同,如AA => old=A genetic_bases中也有A,是一样的需要排除
    /// 
    /// 参考:
    /// 
    /// [Java单向广度优先搜索和双向广度优先搜索](https://leetcode-cn.com/problems/minimum-genetic-mutation/solution/javadan-xiang-yan-du-you-xian-sou-suo-he-shuang-xi/)
    /// 
    /// ### Submissions
    /// 
    /// date=20201221, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132702252/
    pub struct Solution;

    impl Solution {
        pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
            use std::collections::HashSet;
            let mut bank = bank.into_iter().collect::<HashSet<_>>();
            // 目标本身不在bank中 直接退出
            if !bank.contains(&end) {
                return -1;
            }
            bank.remove(&start);
            let mut count = 0;
            // 保存start变化的中间结果
            let mut queue = vec![start];
            let genetic_bases = ['A', 'C', 'G', 'T'];
            while !queue.is_empty() {
                count += 1;
                // 对改变count次后的new_gene再变化
                for _ in 0..queue.len() {
                    let mut gene = queue.pop().unwrap().chars().collect::<Vec<_>>();
                    for i in 0..gene.len() {
                        let old = gene[i];
                        // 用genetic 每次改变一位
                        for base in &genetic_bases {
                            gene[i] = *base;
                            // 新的基因序列
                            let new_gene = gene.iter().collect::<String>();
                            // 找到目标结果 与 最小变化的次数count
                            if new_gene == end {
                                return count;
                            }
                            // 变化的结果在库中
                            else if bank.remove(&new_gene) {
                                queue.push(new_gene);
                            }
                        }
                        // 恢复改变的字符
                        gene[i] = old;
                    }
                }
            }
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        fn test<F: Fn(String, String, Vec<String>) -> i32>(func: F) {
            assert_eq!(
                func(
                    "AAAAACCC".to_string(),
                    "AACCCCCC".to_string(),
                    ["AAAACCCC", "AAACCCCC", "AACCCCCC"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                ),
                3
            );
            assert_eq!(
                func(
                    "AACCGGTT".to_string(),
                    "AACCGGTA".to_string(),
                    vec![] as Vec<String>
                ),
                -1
            );
            assert_eq!(
                func(
                    "AACCGGTT".to_string(),
                    "AACCGGTA".to_string(),
                    vec!["AACCGGTA".to_string()]
                ),
                1
            );
            assert_eq!(
                func(
                    "AACCGGTT".to_string(),
                    "AAACGGTA".to_string(),
                    ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                ),
                2
            );
        }
        test(solution_dfs::Solution::min_mutation);
        test(solution_bfs::Solution::min_mutation);
    }
}
