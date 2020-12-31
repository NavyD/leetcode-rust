//! 如何处理bfs中当前与上层节点建立联系，保存全路径
//!
//! 如何找最短路径并保存
//!
//! ## 怎么找到节点的所有孩子节点。这里有两种方案。
//!
//! - 遍历 wordList 来判断每个单词和当前单词是否只有一个字母不同。
//!
//! ```java
//! for (int i = 0; i < wordList.size(); i++) {
//!     String curWord = wordList.get(i);
//!     //符合只有一个单词不同，就进入递归
//!     if (oneChanged(beginWord, curWord)) {
//!         //此时代表可以从 beginWord -> curWord
//!     }
//! }
//! ```
//!
//! - 将要找的节点单词的每个位置换一个字符，然后看更改后的单词在不在 wordList 中。
//!
//! ```ignore
//! fn _get_next_words(cur: &str, word_list: &HashSet<String>) -> Vec<String> {}
//! ```
//!
//! ## 如何减少dfs多判断很多无用的路径
//!
//! - 只需要考虑最短的层：BFS计算最短路径的高度dfs递归不能超过这个高度
//! - 提前计算dfs中的递归树中每个节点对应的相邻节点：BFS计算出每个节点的successors map
//! - 考虑第 k 层的某一个单词，如果这个单词在第 1 到 k-1 层已经出现过，我们其实就不过继续向下探索了。方式：
//!   - distances map word <=> depth
//!   - visited hashset word: 当保存之前遍历过的word。如果不存在set中才计算next words
//!
//!

pub mod solution_dfs_general {

    /// # 思路
    ///
    /// dfs如何保存找到最短路径
    ///
    /// 在找path时维护一个min_count，如果path.len() > min_count时表示这个path
    /// 当前不是最短的。如果path.len() == min_count个表示这个path是一样短的，
    /// 加入已有的res中。如果path.len() < min_count 表示这个path是最短的且
    /// 之前的res不再有效，清空
    ///
    /// 使用begin_word开始找递归树方式
    ///
    /// 可以通过从word_list中递归找 或
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::HashSet;
            fn _backtrack(
                cur: &str,
                end: &str,
                word_list: &HashSet<String>,
                min_count: &mut usize,
                path: &mut Vec<String>,
                res: &mut Vec<Vec<String>>,
            ) {
                if path.len() > *min_count {
                    return;
                }
                if cur == end {
                    if path.len() < *min_count {
                        res.clear();
                        *min_count = path.len();
                    }
                    res.push(path.clone());
                    return;
                }

                for next in _get_next_words(cur, word_list) {
                    if !path.contains(&next) {
                        path.push(next.clone());
                        _backtrack(&next, end, word_list, min_count, path, res);
                        path.pop();
                    }
                }
            }

            fn _get_next_words(cur: &str, word_list: &HashSet<String>) -> Vec<String> {
                let mut word = cur.chars().collect::<Vec<_>>();
                let mut next_words = vec![];
                for i in 0..word.len() {
                    let old = word[i];
                    for letter in 'a'..='z' {
                        if letter == old {
                            continue;
                        }
                        word[i] = letter;
                        let next_word = word.iter().collect::<String>();
                        if word_list.contains(&next_word) {
                            next_words.push(next_word);
                        }
                    }
                    word[i] = old;
                }
                next_words
            }

            let mut res = vec![];
            if word_list.is_empty() {
                return res;
            }
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if !word_list.contains(&end_word) {
                return res;
            }
            let mut min_count = std::usize::MAX;
            _backtrack(
                &begin_word,
                &end_word,
                &word_list,
                &mut min_count,
                &mut vec![begin_word.clone()],
                &mut res,
            );
            res
        }
    }
}

pub mod solution_dfs_optimized {
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::{HashMap, HashSet, VecDeque};

            fn _backtrack(
                cur: &str,
                end: &str,
                word_list: &HashSet<String>,
                path: &mut Vec<String>,
                next_successors: &HashMap<String, Vec<String>>,
                min_count: usize,
                res: &mut Vec<Vec<String>>,
            ) {
                if cur == end {
                    res.push(path.clone());
                    return;
                }
                // 不能先判断
                if path.len() > min_count {
                    return;
                }
                if let Some(successors) = next_successors.get(cur) {
                    for next in successors {
                        if !path.contains(next) {
                            path.push(next.clone());
                            _backtrack(next, end, word_list, path, next_successors, min_count, res);
                            path.pop();
                        }
                    }
                }
            }

            fn _get_next_words(cur: &str, word_list: &HashSet<String>) -> Vec<String> {
                let mut word = cur.chars().collect::<Vec<_>>();
                let mut next_words = vec![];
                for i in 0..word.len() {
                    let old = word[i];
                    for letter in 'a'..='z' {
                        if letter == old {
                            continue;
                        }
                        word[i] = letter;
                        let next_word = word.iter().collect::<String>();
                        if word_list.contains(&next_word) {
                            next_words.push(next_word);
                        }
                    }
                    word[i] = old;
                }
                next_words
            }

            fn _get_word_successors_and_min_count(
                begin_word: &str,
                end_word: &str,
                word_list: &HashSet<String>,
            ) -> (HashMap<String, Vec<String>>, usize) {
                let mut successors = HashMap::with_capacity(word_list.len());
                let mut queue: VecDeque<String> = VecDeque::new();
                queue.push_back(begin_word.to_owned());
                let mut min_count = 0;
                let mut found = false;
                while !queue.is_empty() {
                    min_count += 1;
                    for _ in 0..queue.len() {
                        let word = queue.pop_front().unwrap();
                        let next_words = _get_next_words(&word, word_list);
                        for next_word in &next_words {
                            if next_word == end_word {
                                found = true;
                            }
                            queue.push_back(next_word.clone());
                        }
                        successors.insert(word, next_words);
                    }
                    if found {
                        break;
                    }
                }
                (successors, min_count)
            }

            let mut res = vec![];
            if word_list.is_empty() {
                return res;
            }
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            dbg!(&word_list);
            if !word_list.contains(&end_word) {
                return res;
            }
            let (next_successors, min_count) =
                _get_word_successors_and_min_count(&begin_word, &end_word, &word_list);
            _backtrack(
                &begin_word,
                &end_word,
                &word_list,
                // path从begin开始
                &mut vec![begin_word.clone()],
                &next_successors,
                min_count,
                &mut res,
            );
            res
        }
    }
}

pub mod solution_dfs {
    /// # 思路
    ///
    /// ### Submissions
    ///
    /// date=20201231, mem=4.3, mem_beats=62, runtime=104, runtime_beats=37.5, url=https://leetcode-cn.com/submissions/detail/135135940/
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::{HashMap, HashSet, VecDeque};

            fn _backtrack(
                cur: &str,
                end: &str,
                word_list: &HashSet<String>,
                path: &mut Vec<String>,
                word_successors: &HashMap<String, Vec<String>>,
                res: &mut Vec<Vec<String>>,
            ) {
                if cur == end {
                    res.push(path.clone());
                    return;
                }
                if let Some(successors) = word_successors.get(cur) {
                    for next in successors {
                        path.push(next.clone());
                        _backtrack(next, end, word_list, path, word_successors, res);
                        path.pop();
                    }
                }
            }

            fn _get_next_words(cur: &str, word_list: &HashSet<String>) -> Vec<String> {
                let mut word = cur.chars().collect::<Vec<_>>();
                let mut next_words = vec![];
                for i in 0..word.len() {
                    let old = word[i];
                    for letter in 'a'..='z' {
                        if letter != old {
                            word[i] = letter;
                            let next_word = word.iter().collect::<String>();
                            if word_list.contains(&next_word) {
                                next_words.push(next_word);
                            }
                        }
                    }
                    word[i] = old;
                }
                next_words
            }

            fn _get_word_successors(
                begin_word: &str,
                end_word: &str,
                word_list: &HashSet<String>,
            ) -> HashMap<String, Vec<String>> {
                let mut successors = HashMap::with_capacity(word_list.len());

                let mut visited = HashSet::new();
                let mut queue: VecDeque<String> = VecDeque::new();
                queue.push_back(begin_word.to_string());
                let mut is_found = false;

                while !queue.is_empty() {
                    // 保存已找过的words
                    queue.iter().for_each(|e| {
                        visited.insert(e.to_string());
                    });
                    for _ in 0..queue.len() {
                        let word = queue.pop_front().unwrap();
                        // 剪枝：不重复计算相同的next_words
                        if !successors.contains_key(&word) {
                            let mut next_words = _get_next_words(&word, word_list);
                            next_words.retain(|next| {
                                // 剪枝：移除所有之前path出现的word后继节点
                                if !visited.contains(next) {
                                    if next == end_word {
                                        is_found = true;
                                    }
                                    queue.push_back(next.clone());
                                    true
                                } else {
                                    false
                                }
                            });
                            successors.insert(word, next_words);
                        }
                    }
                    if is_found {
                        break;
                    }
                }
                successors
            }

            let mut res = vec![];
            if word_list.is_empty() {
                return res;
            }
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if !word_list.contains(&end_word) {
                return res;
            }
            let word_successors = _get_word_successors(&begin_word, &end_word, &word_list);
            _backtrack(
                &begin_word,
                &end_word,
                &word_list,
                // path从begin开始
                &mut vec![begin_word.clone()],
                &word_successors,
                &mut res,
            );
            res
        }
    }
}

pub mod solution_bfs_wait {
    use std::collections::{HashMap, HashSet, VecDeque};
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::{HashMap, HashSet, VecDeque};

            fn _get_next_words(cur: &str, word_list: &HashSet<String>) -> Vec<String> {
                let mut word = cur.chars().collect::<Vec<_>>();
                let mut next_words = vec![];
                for i in 0..word.len() {
                    let old = word[i];
                    for letter in 'a'..='z' {
                        if letter != old {
                            word[i] = letter;
                            let next_word = word.iter().collect::<String>();
                            if word_list.contains(&next_word) {
                                next_words.push(next_word);
                            }
                        }
                    }
                    word[i] = old;
                }
                next_words
            }

            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if !word_list.contains(&end_word) {
                return vec![];
            }

            let mut path_queue = VecDeque::new();
            path_queue.push_back(vec![begin_word]);
            let mut visited = HashSet::new();
            let mut is_found = false;

            while !path_queue.is_empty() {
                path_queue.iter().flatten().for_each(|e| {
                    if !visited.contains(e) {
                        visited.insert(e.clone());
                    }
                });
                for _ in 0..path_queue.len() {
                    let mut path = path_queue.pop_front().unwrap();
                    if let Some(cur_word) = path.last() {
                        // if visited.contains(cur_word) {
                        //     continue;
                        // }
                        for next in _get_next_words(cur_word, &word_list) {
                            if !visited.contains(&next) {
                                if next == end_word {
                                    is_found = true;
                                    path.push(next.clone());
                                    path_queue.push_back(path.clone());
                                    path.pop();
                                }

                                path.push(next);
                                path_queue.push_back(path.clone());
                                path.pop();
                            }
                        }
                    }
                }
                if is_found {
                    break;
                }
            }
            path_queue.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn basic() {
        test(solution_dfs::Solution::find_ladders);
        // test(solution_bfs_wait::Solution::find_ladders);
        // test(solution_dfs_optimized::Solution::find_ladders);
    }

    fn test<F: Fn(String, String, Vec<String>) -> Vec<Vec<String>>>(func: F) {
        assert_eq!(
            func(
                "hot".to_string(),
                "dog".to_string(),
                ["hot", "dog"].iter().map(|e| e.to_string()).collect()
            ),
            vec![] as Vec<Vec<String>>
        );

        let res = func(
            "hit".to_string(),
            "cog".to_string(),
            ["hot", "dot", "dog", "lot", "log", "cog"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
        );
        let expected: Vec<Vec<String>> = [
            ["hit", "hot", "dot", "dog", "cog"],
            ["hit", "hot", "lot", "log", "cog"],
        ]
        .iter()
        .map(|a| a.iter().map(|e| e.to_string()).collect())
        .collect();
        assert_eq!(res.len(), expected.len());
        assert!(is_contains_vec2(&res, &expected));
    }
}
