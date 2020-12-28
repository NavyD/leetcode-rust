//! 与[minimum_genetic_mutation](crate::minimum_genetic_mutation)一样

#![allow(dead_code)]
mod solution_dfs {
    /// # 思路
    ///
    /// 超过时间限制。在word_list.len()>10时几乎不可用
    pub struct Solution;

    impl Solution {
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            fn _dfs<'a>(
                cur: &str,
                end: &str,
                min_count: &mut i32,
                path: &mut Vec<&'a str>,
                word_list: &'a Vec<String>,
            ) {
                let count = path.len() as i32;
                if count >= *min_count {
                    return;
                }
                if cur == end {
                    *min_count = count;
                    return;
                }
                for word in word_list {
                    let diff = word
                        .chars()
                        .zip(cur.chars())
                        .filter(|(a, b)| a != b)
                        .count();
                    let word = word.as_str();
                    if diff == 1 && !path.contains(&word) {
                        path.push(word);
                        _dfs(word, end, min_count, path, word_list);
                        path.pop();
                    }
                }
            }
            if !word_list.contains(&end_word) {
                return 0;
            }
            let mut min_count = std::i32::MAX;
            _dfs(
                &begin_word,
                &end_word,
                &mut min_count,
                &mut vec![],
                &word_list,
            );
            if min_count == std::i32::MAX {
                0
            } else {
                min_count + 1
            }
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// 注意：count=1开始是题目要求
    ///
    /// ### Submissions
    ///
    /// date=20201227, mem=2.4, mem_beats=96, runtime=88, runtime_beats=21, url=https://leetcode-cn.com/submissions/detail/134057724/
    ///
    /// date=20201228, mem=2.4, mem_beats=93, runtime=88, runtime_beats=21, url=https://leetcode-cn.com/submissions/detail/134406528/
    pub struct Solution;

    impl Solution {
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            use std::collections::HashSet;
            const NOT_FOUND: i32 = 0;
            if word_list.is_empty() {
                return NOT_FOUND;
            }
            let mut unvisited = word_list.into_iter().collect::<HashSet<_>>();
            if !unvisited.contains(&end_word) {
                return NOT_FOUND;
            }
            unvisited.remove(&begin_word);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(begin_word);
            let mut count = 1;
            while !queue.is_empty() {
                count += 1;
                for _ in 0..queue.len() {
                    let mut word = queue.pop_front().unwrap().chars().collect::<Vec<_>>();
                    for i in 0..word.len() {
                        let old = word[i];
                        for letter in 'a'..='z' {
                            word[i] = letter;
                            let new_word = word.iter().collect::<String>();
                            if new_word == end_word {
                                return count;
                            } else if unvisited.remove(&new_word) {
                                queue.push_back(new_word);
                            }
                        }
                        word[i] = old;
                    }
                }
            }
            NOT_FOUND
        }
    }
}

pub mod solution_bfs_two_end {
    /// # 思路
    ///
    /// 双向广度优先遍历
    ///
    /// - 已知目标顶点的情况下，可以分别从起点和目标顶点（终点）执行广度优先遍历，直到遍历的部分有交集。这种方式搜索的单词数量会更小一些；
    /// - 更合理的做法是，每次从单词数量小的集合开始扩散；
    /// - 这里 beginVisited 和 endVisited 交替使用，等价于单向 BFS 里使用队列，每次扩散都要加到总的 visited 里。
    ///
    /// 参考：
    ///
    /// - [广度优先遍历、双向广度优先遍历（Java）](https://leetcode-cn.com/problems/word-ladder/solution/yan-du-you-xian-bian-li-shuang-xiang-yan-du-you-2/)
    /// - [4 ms](https://leetcode-cn.com/submissions/api/detail/127/rust/4/)
    ///
    /// ### Submissions
    ///
    /// date=20201227, mem=2.4, mem_beats=97, runtime=12, runtime_beats=75, url=https://leetcode-cn.com/submissions/detail/134072129/
    /// 
    /// date=20201228, mem=2.5, mem_beats=66, runtime=12, runtime_beats=75, url=https://leetcode-cn.com/submissions/detail/134409400/
    pub struct Solution;

    impl Solution {
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            use std::collections::HashSet;
            const NOT_FOUND: i32 = 0;
            if word_list.is_empty() {
                return NOT_FOUND;
            }
            let mut unvisited = word_list.into_iter().collect::<HashSet<_>>();
            if !unvisited.contains(&end_word) {
                return NOT_FOUND;
            }
            unvisited.remove(&begin_word);
            let (mut begin_visited, mut end_visited) = (HashSet::new(), HashSet::new());
            begin_visited.insert(begin_word);
            end_visited.insert(end_word);
            let mut count = 1;
            while !begin_visited.is_empty() {
                count += 1;
                let mut next_begin_visited = HashSet::new();
                // 从前遍历当前第count层
                for word in begin_visited {
                    let mut word = word.chars().collect::<Vec<_>>();
                    // 修改word每个字符
                    for i in 0..word.len() {
                        let old = word[i];
                        for letter in 'a'..='z' {
                            word[i] = letter;
                            let new_word = word.iter().collect::<String>();
                            // 与后遍历相交时返回
                            if end_visited.contains(&new_word) {
                                return count;
                            }
                            // 添加到下层
                            else if unvisited.remove(&new_word) {
                                next_begin_visited.insert(new_word);
                            }
                        }
                        word[i] = old;
                    }
                }
                // 使用len小的遍历
                if next_begin_visited.len() < end_visited.len() {
                    begin_visited = next_begin_visited;
                } else {
                    begin_visited = end_visited;
                    end_visited = next_begin_visited;
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
        test(solution_bfs::Solution::ladder_length);
        test(solution_bfs_two_end::Solution::ladder_length);
    }

    fn test<F: Fn(String, String, Vec<String>) -> i32>(func: F) {
        assert_eq!(
            func(
                "hot".to_string(),
                "dog".to_string(),
                ["hot", "dog"].iter().map(|e| e.to_string()).collect()
            ),
            0
        );
        assert_eq!(
            func(
                "hit".to_string(),
                "cog".to_string(),
                ["hot", "dot", "dog", "lot", "log", "cog"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            5
        );
        assert_eq!(
            func(
                "hit".to_string(),
                "cog".to_string(),
                ["hot", "dot", "dog", "lot", "log"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            0
        );
        assert_eq!(
            func(
                "hit".to_string(),
                "cog".to_string(),
                ["hot", "dot", "tog", "cog"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            0
        );
        assert_eq!(
            func(
                "qa".to_string(),
                "sq".to_string(),
                [
                    "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm",
                    "le", "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo",
                    "ow", "sn", "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur",
                    "rh", "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th",
                    "pm", "rb", "sh", "co", "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa",
                    "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st", "er", "sc", "ne",
                    "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr", "pa",
                    "he", "lr", "sq", "ye"
                ]
                .iter()
                .map(|e| e.to_string())
                .collect()
            ),
            5
        )
    }
}
