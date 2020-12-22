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
    /// 在回溯时计数count可以被path.len()替代，count+1与path.push是一样的。
    /// 另外，由于回溯递归深度不会太多，path.contains可以使用Vec而不用hashset
    ///
    /// 下面是使用count
    ///
    /// ```ignore
    /// fn _backtrack<'a>(
    ///     bank: &'a Vec<String>,
    ///     path: &mut HashSet<&'a str>,
    ///     cur: &str,
    ///     end: &str,
    ///     count: i32,
    ///     min_count: &mut i32,
    /// ) {
    ///     if count >= *min_count {
    ///         return;
    ///     }
    ///     if cur == end {
    ///         *min_count = count;
    ///         return;
    ///     }
    ///     for gene in bank {
    ///         //...
    ///             _backtrack(bank, path, gene, end, count + 1, min_count);
    ///         //...
    ///     }
    /// }
    /// ```
    ///
    /// 参考:
    ///
    /// - [Java DFS （回溯）](https://leetcode-cn.com/problems/minimum-genetic-mutation/solution/java-dfs-hui-su-by-1yx/)
    ///
    /// ### Submissions
    ///
    /// date=20201221, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132618271/
    ///
    /// date=20201222, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132885625/
    pub struct Solution;

    impl Solution {
        pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
            fn _backtrack<'a>(
                bank: &'a Vec<String>,
                cur: &str,
                end: &str,
                path: &mut Vec<&'a str>,
                min_count: &mut i32,
            ) {
                if path.len() >= *min_count as usize {
                    return;
                }
                if cur == end {
                    *min_count = path.len() as i32;
                    return;
                }
                for genes in bank {
                    // look for gene that only changes once
                    let diff = cur
                        .bytes()
                        .enumerate()
                        .filter(|(i, b)| genes.as_bytes()[*i] != *b)
                        .count();
                    let genes = genes.as_str();
                    // the current gene must be uncounted
                    if diff == 1 && !path.contains(&genes) {
                        path.push(genes);
                        _backtrack(bank, genes, end, path, min_count);
                        path.pop();
                    }
                }
            }
            let mut min_count = std::i32::MAX;
            _backtrack(&bank, &start, &end, &mut vec![], &mut min_count);
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
    /// bank=>unvisited表示一个没有被访问过的基因库，如果start的基因变体中出现在unvisited，
    /// 表示这是合法的，并且不能出现第二次。没有remove的变体就是不合法的
    ///
    /// 使用stack: Vec与queue: Deque一样，没有pop的顺序问题，只要每层都处理完成
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
    ///
    /// date=20201222, mem=2.2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/132868477/
    pub struct Solution;

    impl Solution {
        pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
            let mut unvisited = bank.into_iter().collect::<std::collections::HashSet<_>>();
            // 目标本身不在bank中 直接退出
            if !unvisited.contains(&end) {
                return -1;
            }
            unvisited.remove(&start);
            let mut count = 0;
            let genetic_bases = ['A', 'C', 'G', 'T'];
            // 保存start变化的中间结果
            let mut stack = vec![start];
            while !stack.is_empty() {
                count += 1;
                // 对改变count次后的new_gene再变化
                for _ in 0..stack.len() {
                    let mut genes = stack.pop().unwrap().chars().collect::<Vec<_>>();
                    for i in 0..genes.len() {
                        let old_base = genes[i];
                        // 用genetic 每次改变一位
                        for base in &genetic_bases {
                            genes[i] = *base;
                            // 新的基因序列
                            let new_genes = genes.iter().collect::<String>();
                            // 找到目标结果 与 最小变化的次数count
                            if new_genes == end {
                                return count;
                            }
                            // 变化的结果在库中
                            else if unvisited.remove(&new_genes) {
                                stack.push(new_genes);
                            }
                        }
                        // 恢复改变的字符
                        genes[i] = old_base;
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
