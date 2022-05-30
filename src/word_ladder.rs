//! 与[minimum_genetic_mutation](crate::minimum_genetic_mutation)一样
//!
//! 与[word_ladder_ii]的区别：bfs中的next_visited设计为何在这里没有

pub mod solution_bfs {
    /// # 思路
    ///
    /// 注意：count=1开始是题目要求
    ///
    /// 由于word可以改变一个字母，在无向图中可以改变成之前出现的单词：hit -> hot, hot -> hit，也可以在同层
    /// 的word相互转换，相当于树中多叉节点，但不能跨节点转换（每次转换只能改变一个字母）
    ///
    /// ## 如何避免重复
    ///
    /// 使用visited避免重复visited.insert(next_word)必须在queue.push_back时添加，不能在queue.pop_front时才insert，
    /// 因为没有这个visited.contains包含，当前层的之前可能出现一样的word被重复添加到queue中导致超时
    ///
    /// 还有一种使用word_list作为unvisited替代visited in-place remove的方式操作，与insert相反，但是
    /// 这个visited是更通用的方式
    ///
    /// ### Submissions
    ///
    /// date=20201227, mem=2.4, mem_beats=96, runtime=88, runtime_beats=21, url=https://leetcode-cn.com/submissions/detail/134057724/
    ///
    /// date=20201228, mem=2.4, mem_beats=93, runtime=88, runtime_beats=21, url=https://leetcode-cn.com/submissions/detail/134406528/
    ///
    /// date=20210109, mem=2.9, mem_beats=26, runtime=104, runtime_beats=19, url=https://leetcode-cn.com/submissions/detail/137131008/
    ///
    /// date=20220530, mem=2.8, mem_beats=16, runtime=52, runtime_beats=32
    pub struct Solution;

    impl Solution {
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            use std::collections::{HashSet, VecDeque};
            const NOT_FOUND: i32 = 0;

            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return NOT_FOUND;
            }
            // 要求从1开始
            let mut count = 1;

            let word_len = begin_word.len();
            let (mut queue, mut visited) = (VecDeque::new(), HashSet::new());
            // 从begin_word开始
            queue.push_back(begin_word.clone());
            // 避免再次从begin_word计算next_words
            visited.insert(begin_word);

            while !queue.is_empty() {
                count += 1;
                for _ in 0..queue.len() {
                    let mut word = queue.pop_front().map(Into::<Vec<u8>>::into).unwrap();
                    for i in 0..word_len {
                        let old = word[i];
                        for c in b'a'..=b'z' {
                            word[i] = c;
                            let new_word = unsafe { std::str::from_utf8_unchecked(&word) };
                            if word_list.contains(new_word) {
                                if new_word == end_word {
                                    return count;
                                }
                                if visited.insert(new_word.to_string()) {
                                    queue.push_back(new_word.to_string());
                                }
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
    /// 注意：
    ///
    /// 这是的begin,end visited表示前后当前层的words,而global_visited表示在bfs中同样的含义，即防止重复到下层。
    ///
    /// begin,end visited使用hashset是为了在end.contains加速，使用vec也可以正常工作可以通过[leetcode测试](https://leetcode-cn.com/submissions/detail/137144826/)
    ///
    /// haset.contains与insert逻辑可简写：
    ///
    /// ```ignore
    /// if global_visited.insert(&next_word) {
    ///     next_level_visited.insert(next_word.clone());
    /// }
    /// // 替换
    /// // if !global_visited.contains(&next_word) {
    /// //     next_level_visited.insert(next_word.clone());
    /// //     global_visited.insert(next_word);
    /// // }
    /// ```
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
    ///
    /// date=20210109, mem=2.5, mem_beats=69, runtime=12, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/137149168/
    pub struct Solution;

    impl Solution {
        pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
            use std::collections::HashSet;
            const NOT_FOUND: i32 = 0;

            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return NOT_FOUND;
            }

            let (mut begin_level_visited, mut end_level_visited) = (HashSet::new(), HashSet::new());
            let mut global_visited = HashSet::new();
            // 左边和右边扩散
            begin_level_visited.insert(begin_word.clone());
            end_level_visited.insert(end_word.clone());
            // 避免重复一次 不加也没事，重复的会被global_visited过滤 不会添加到next_level
            global_visited.insert(begin_word);
            global_visited.insert(end_word);
            // 计数从1开始
            let mut count = 1;

            while !begin_level_visited.is_empty() {
                count += 1;
                if begin_level_visited.len() > end_level_visited.len() {
                    std::mem::swap(&mut begin_level_visited, &mut end_level_visited);
                }
                let mut next_level_visited = HashSet::new();
                for cur_word in begin_level_visited {
                    let mut cur_word = cur_word.chars().collect::<Vec<_>>();
                    for i in 0..cur_word.len() {
                        let old = cur_word[i];
                        for letter in 'a'..='z' {
                            cur_word[i] = letter;
                            let next_word = cur_word.iter().collect::<String>();
                            if word_list.contains(&next_word) {
                                if end_level_visited.contains(&next_word) {
                                    return count;
                                }
                                // global_visited不存在时添加 next_word
                                if global_visited.insert(next_word.clone()) {
                                    next_level_visited.insert(next_word);
                                }
                            }
                        }
                        cur_word[i] = old;
                    }
                }
                begin_level_visited = next_level_visited;
            }
            NOT_FOUND
        }
    }
}

#[allow(dead_code)]
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
                word_list: &'a [String],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_bfs_two_end::Solution::ladder_length);
        test(solution_bfs::Solution::ladder_length);
        test(ladder_length)
    }

    fn test<F: Fn(String, String, Vec<String>) -> i32>(func: F) {
        fn arr<const N: usize>(a: [&str; N]) -> Vec<String> {
            a.into_iter().map(ToString::to_string).collect()
        }

        assert_eq!(
            func("hot".to_string(), "dog".to_string(), arr(["hot", "dog"])),
            0
        );
        assert_eq!(
            func(
                "hit".to_string(),
                "cog".to_string(),
                arr(["hot", "dot", "dog", "lot", "log", "cog"])
            ),
            5
        );
        assert_eq!(
            func(
                "hit".to_string(),
                "cog".to_string(),
                arr(["hot", "dot", "dog", "lot", "log"])
            ),
            0
        );
        assert_eq!(
            func(
                "hit".to_string(),
                "cog".to_string(),
                arr(["hot", "dot", "tog", "cog"])
            ),
            0
        );
        assert_eq!(
            func(
                "qa".to_string(),
                "sq".to_string(),
                arr([
                    "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm",
                    "le", "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo",
                    "ow", "sn", "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur",
                    "rh", "sr", "tc", "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th",
                    "pm", "rb", "sh", "co", "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa",
                    "pi", "os", "uh", "wm", "an", "me", "mo", "na", "la", "st", "er", "sc", "ne",
                    "mn", "mi", "am", "ex", "pt", "io", "be", "fm", "ta", "tb", "ni", "mr", "pa",
                    "he", "lr", "sq", "ye"
                ])
            ),
            5
        )
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::{HashSet, VecDeque};
        const NOT_FOUND: i32 = 0;

        let word_list = word_list.into_iter().collect::<HashSet<_>>();
        if word_list.is_empty() || !word_list.contains(&end_word) {
            return NOT_FOUND;
        }

        let mut count = 1;
        let word_len = begin_word.len();
        let mut queue = VecDeque::new();
        queue.push_back(begin_word.clone());

        let mut visited = HashSet::new();
        visited.insert(begin_word);

        while !queue.is_empty() {
            count += 1;
            for _ in 0..queue.len() {
                let mut word = queue.pop_front().map(Into::<Vec<u8>>::into).unwrap();
                for i in 0..word_len {
                    let old = word[i];
                    for c in b'a'..=b'z' {
                        word[i] = c;
                        let new_word = unsafe { std::str::from_utf8_unchecked(&word) };
                        if word_list.contains(new_word) {
                            if new_word == end_word {
                                return count;
                            }
                            if visited.insert(new_word.to_string()) {
                                queue.push_back(new_word.to_string());
                            }
                        }
                    }
                    word[i] = old;
                }
            }
        }

        NOT_FOUND
    }
}
