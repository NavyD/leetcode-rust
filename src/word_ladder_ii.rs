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
//!
//! 由于要记录所有的路径，广度优先遍历「当前层」到「下一层」的所有路径都得记录下来。
//! 因此找到下一层的结点 wordA 以后，不能马上添加到 visited 哈希表里，
//! 还需要检查当前队列中未出队的单词是否还能与 wordA 建立联系。
//!
//! 所以可以在当前层用next_visited临时保存当前层与next_word的联系，不能直接visited.insert影响
//! 当前层后面的word与next_word的关系。当前层与当前层word间联系也不能被记录，
//! 由于同一层的连接肯定不是起点到终点的最短路径的边，所以使用 `next_visited: HashSet<String>`
//! （避免同层word的next_words出现相同的当前层word，同层相连）
//!
//! ![](https://pic.leetcode-cn.com/da957ec0ab9fe9eda166acab86ba9fca8b6b0f6b04bda3e35677a46fb53d7d17-image.png)
//!
//! word_successors表示使用BFS遍历找到的最短路径中的word，每个word联系着最短路径中的next_words
//!
//! - 考虑第 k 层的某一个单词，如果这个单词在第 1 到 k-1 层已经出现过，我们其实就不过继续向下探索了。方式：
//!   - distances map word <=> depth
//!   - visited hashset word: 当保存之前遍历过的word。如果不存在set中才计算next words
//!
//! 参考：
//!
//! [单双向广度优先遍历 + 回溯算法（Java、Python）](https://leetcode-cn.com/problems/word-ladder-ii/solution/yan-du-you-xian-bian-li-shuang-xiang-yan-du-you--2/)

pub mod solution_dfs_bfs {
    /// # 思路
    ///
    /// _get_word_successors中的next_visited用于保存
    ///
    /// ### Submissions
    ///
    /// date=20201231, mem=4.3, mem_beats=62, runtime=104, runtime_beats=37.5, url=https://leetcode-cn.com/submissions/detail/135135940/
    ///
    /// date=20210103, mem=3.6, mem_beats=92, runtime=108, runtime_beats=57, url=https://leetcode-cn.com/submissions/detail/135720501/
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::{HashMap, HashSet, VecDeque};

            fn _backtrack<'a>(
                cur: &str,
                end: &str,
                word_successors: &'a HashMap<String, Vec<String>>,
                path: &mut Vec<&'a str>,
                res: &mut Vec<Vec<String>>,
            ) {
                if cur == end {
                    res.push(path.iter().map(|e| e.to_string()).collect());
                } else if let Some(successors) = word_successors.get(cur) {
                    for next in successors {
                        path.push(next);
                        _backtrack(next, end, word_successors, path, res);
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

            // bfs
            fn _get_word_successors(
                begin: &str,
                end: &str,
                word_list: &HashSet<String>,
            ) -> HashMap<String, Vec<String>> {
                let mut word_successors = HashMap::new();

                let mut visited = HashSet::new();
                visited.insert(begin.to_string());
                let mut queue = VecDeque::new();
                queue.push_back(begin.to_string());

                let mut is_found = false;
                while !queue.is_empty() {
                    let mut next_visited = HashSet::new();
                    for _ in 0..queue.len() {
                        let cur_word = queue.pop_front().unwrap();
                        for next_word in _get_next_words(&cur_word, word_list) {
                            if next_word == end {
                                is_found = true;
                            }
                            if !visited.contains(&next_word) {
                                // 避免下层元素重复加入队列
                                if next_visited.insert(next_word.clone()) {
                                    queue.push_back(next_word.clone());
                                }
                                word_successors
                                    .entry(cur_word.clone())
                                    .or_insert(vec![])
                                    .push(next_word);
                            }
                        }
                    }
                    if is_found {
                        break;
                    }
                    next_visited.into_iter().for_each(|e| {
                        visited.insert(e);
                    });
                }
                word_successors
            }

            let mut res = vec![];
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return res;
            }
            _backtrack(
                &begin_word,
                &end_word,
                &_get_word_successors(&begin_word, &end_word, &word_list),
                &mut vec![&begin_word],
                &mut res,
            );
            res
        }
    }
}

pub mod solution_dfs_bfs_two_end {
    /// # 思路
    ///
    /// ### Submissions
    ///
    /// date=20210103, mem=2.6, mem_beats=100, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/135723941/
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::{HashMap, HashSet};
    
            fn _backtrack<'a>(
                cur: &str,
                end: &str,
                word_successors: &'a HashMap<String, Vec<String>>,
                path: &mut Vec<&'a str>,
                res: &mut Vec<Vec<String>>,
            ) {
                if cur == end {
                    res.push(path.iter().map(|e| e.to_string()).collect());
                } else if let Some(successors) = word_successors.get(cur) {
                    for next in successors {
                        path.push(next);
                        _backtrack(next, end, word_successors, path, res);
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
    
            /// with bfs two end
            fn _get_word_successors(
                begin: &str,
                end: &str,
                word_list: &HashSet<String>,
            ) -> HashMap<String, Vec<String>> {
                let mut word_successors = HashMap::new();
    
                let mut visited = HashSet::new();
                visited.insert(begin.to_string());
                visited.insert(end.to_string());
    
                let (mut begin_visited, mut end_visited) = (HashSet::new(), HashSet::new());
                begin_visited.insert(begin.to_string());
                end_visited.insert(end.to_string());
                let mut is_forward = true;
                let mut is_found = false;
    
                // 将word为key的successors中插入next_word。
                // 如果is_forward=false则反向插入：successors[next_word].push(word)
                let mut successors_insert =
                    |mut word: String, mut next_word: String, is_forward: bool| {
                        if !is_forward {
                            std::mem::swap(&mut word, &mut next_word);
                        }
                        word_successors.entry(word).or_insert(vec![]).push(next_word);
                    };
                // 在保证了 beginVisited 总是较小（可以等于）大小的集合前提下，
                // && !endVisited.isEmpty() 可以省略
                while !begin_visited.is_empty() {
                    // 默认 beginVisited 是小集合
                    if begin_visited.len() > end_visited.len() {
                        std::mem::swap(&mut begin_visited, &mut end_visited);
                        // 每次改变都对应 begin_visited 的方向
                        is_forward = !is_forward;
                    }
                    // 临时保存当前层对应的下层words
                    let mut next_visited = HashSet::new();
                    for word in begin_visited {
                        for next_word in _get_next_words(&word, word_list) {
                            // 在另一侧找到单词以后，还需把这一层关系添加到「后继结点列表」
                            if end_visited.contains(&next_word) {
                                is_found = true;
                                successors_insert(word.clone(), next_word.clone(), is_forward);
                            }
                            if !visited.contains(&next_word) {
                                next_visited.insert(next_word.clone());
                                successors_insert(word.clone(), next_word.clone(), is_forward);
                            }
                        }
                    }
                    if is_found {
                        break;
                    }
                    next_visited.iter().for_each(|e| {
                        visited.insert(e.clone());
                    });
                    begin_visited = next_visited;
                }
                word_successors
            }
    
            let mut res = vec![];
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return res;
            }
            _backtrack(
                &begin_word,
                &end_word,
                &_get_word_successors(&begin_word, &end_word, &word_list),
                &mut vec![&begin_word],
                &mut res,
            );
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn basic() {
        test(solution_dfs_bfs::Solution::find_ladders);
        test(solution_dfs_bfs_two_end::Solution::find_ladders);

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
