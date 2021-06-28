//! 起始基因如何变化到目标基因序列
//!
//! 每次只能改变一个字符:"A", "C", "G", "T",并且改变后的
//! 基因都要属于在基因库中.当改变后的基因==目标基因时结束统计
//! 改变的次数

pub mod solution_dfs {
    /// # 思路
    ///
    /// dfs在遍历基因库中找出可能的next回溯
    ///
    /// 在回溯时计数count可以被path.len()替代，count+1与path.push是一样的。
    /// 另外，由于回溯递归深度不会太多，path.contains可以使用Vec而不用hashset
    ///
    /// 下面是使用改变每位字母的方式，理论上这个方式更快与bfs相似。
    ///
    /// ```ignore
    /// fn _backtrack(
    ///     cur: &str,
    ///     end: &str,
    ///     bank: &std::collections::HashSet<String>,
    ///     path: &mut std::collections::HashSet<String>,
    ///     min_count: &mut usize,
    /// ) {
    ///     if *min_count < path.len() {
    ///         return;
    ///     }
    ///     if cur == end {
    ///         *min_count = path.len();
    ///         return;
    ///     }
    ///     let mut cur = cur.chars().collect::<Vec<_>>();
    ///     for i in 0..cur.len() {
    ///         let old = cur[i];
    ///         for letter in &GENETIC_BASES {
    ///             cur[i] = *letter;
    ///             let next = cur.iter().collect::<String>();
    ///             if bank.contains(&next) && !path.contains(&next) {
    ///                 path.insert(next.clone());
    ///                 _backtrack(&next, end, bank, path, min_count);
    ///                 path.remove(&next);
    ///             }
    ///         }
    ///         cur[i] = old;
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
    ///
    /// date=20201222, mem=2.1, mem_beats=16, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138084453/
    pub struct Solution;

    impl Solution {
        pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
            use std::collections::HashSet;
            const NOT_FOUND: i32 = -1;

            if !bank.contains(&end) {
                return NOT_FOUND;
            }

            fn _backtrack<'a>(
                cur: &str,
                end: &str,
                bank: &'a [String],
                path: &mut HashSet<&'a str>,
                min_count: &mut usize,
            ) {
                // path.len作为当前的深度
                if path.len() > *min_count {
                    return;
                }
                if cur == end {
                    *min_count = path.len();
                    return;
                }
                for next in bank {
                    // 与cur比较找出仅1个字母不同的next
                    let diff = cur
                        .bytes()
                        .enumerate()
                        .filter(|(i, b)| next.as_bytes()[*i] != *b)
                        .count();
                    let next = next.as_str();
                    // the current gene must be uncounted
                    if diff == 1 && !path.contains(&next) {
                        path.insert(next);
                        _backtrack(next, end, bank, path, min_count);
                        path.remove(&next);
                    }
                }
            }
            let mut min_count = std::usize::MAX;
            _backtrack(&start, &end, &bank, &mut HashSet::new(), &mut min_count);
            if min_count == std::usize::MAX {
                NOT_FOUND
            } else {
                min_count as i32
            }
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// bfs
    ///
    /// 简单起见使用额外的visited:HaseSet作为记录，而不是使用bank原地操作，下面使用bank在原地操作
    ///
    /// bank=>unvisited表示一个没有被访问过的基因库，如果start的基因变体中出现在unvisited，
    /// 表示这是合法的，并且不能出现第二次。没有remove的变体就是不合法的
    ///
    /// 使用stack: Vec与queue: Deque一样，没有pop的顺序问题，只要每层都处理完成
    ///
    /// 注意防止重复：当新的new_gene被包含在bank中时,必须要删除防止在下次出现同样的new_gene时再次
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
    ///
    /// date=20210113, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138067795/
    pub struct Solution;

    impl Solution {
        pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
            const NOT_FOUND: i32 = -1;
            const GENETIC_BASES: [char; 4] = ['A', 'C', 'G', 'T'];

            // 目标本身不在bank中 直接退出
            let bank = bank.into_iter().collect::<std::collections::HashSet<_>>();
            if !bank.contains(&end) {
                return NOT_FOUND;
            }
            // 对每层改变计数
            let mut count = 0;

            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start.clone());
            // 保存之前出现过的
            let mut visited = std::collections::HashSet::new();
            visited.insert(start);

            while !queue.is_empty() {
                count += 1;
                for _ in 0..queue.len() {
                    let mut cur = queue
                        .pop_front()
                        .map(|s| s.chars().collect::<Vec<_>>())
                        .unwrap();
                    // 改变每个位置一次 组成新gene
                    for i in 0..cur.len() {
                        let old = cur[i];
                        for letter in &GENETIC_BASES {
                            cur[i] = *letter;
                            let next = cur.iter().collect::<String>();
                            // 变化的结果在库中 之前未访问过
                            if bank.contains(&next) && !visited.contains(&next) {
                                if next == end {
                                    return count;
                                }
                                visited.insert(next.clone());
                                queue.push_back(next);
                            }
                        }
                        // 恢复
                        cur[i] = old;
                    }
                }
            }
            NOT_FOUND
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dfs::Solution::min_mutation);
        // test(solution_bfs::Solution::min_mutation);
        // test(min_mutation)
    }

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
}
