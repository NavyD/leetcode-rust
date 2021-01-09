//! 如何处理bfs中当前与上层节点建立联系，保存全路径
//!
//! 如何找最短路径并保存

pub mod solution_dfs_bfs {
    /// # 思路
    ///
    /// _get_word_successors中的next_visited用于保存
    ///
    /// ## 怎么找到节点的所有孩子节点。这里有两种方案。
    ///
    /// - 遍历 wordList 来判断每个单词和当前单词是否只有一个字母不同。
    ///
    /// ```java,ignore
    /// for (int i = 0; i < wordList.size(); i++) {
    ///     String curWord = wordList.get(i);
    ///     //符合只有一个单词不同，就进入递归
    ///     if (oneChanged(beginWord, curWord)) {
    ///         //此时代表可以从 beginWord -> curWord
    ///     }
    /// }
    /// ```
    ///
    /// - 将要找的节点单词的每个位置换一个字符，然后看更改后的单词在不在 wordList 中。
    ///
    /// ```ignore
    /// fn _get_next_words(cur: &str, word_list: &HashSet<String>) -> Vec<String> {}
    /// ```
    ///
    /// ## 如何减少dfs多判断很多无用的路径
    ///
    /// - 只需要考虑最短的层：BFS计算最短路径的高度dfs递归不能超过这个高度
    /// - 提前计算dfs中的递归树中每个节点对应的相邻节点：BFS计算出每个节点的相邻节点successors map
    ///
    /// 由于要记录所有的路径，广度优先遍历「当前层」到「下一层」的所有路径都得记录下来。
    /// 因此找到下一层的结点 next_word 以后，不能马上添加到 visited 哈希表里，
    /// 还需要检查当前队列中未出队的单词是否还能与 next_word 建立联系。这是与`word_ladder`不同，
    /// word_ladder不需要路径记录，只需能到达同个节点的一步，而这里 hot->aot, ait->aot都要记录
    ///
    /// ```ignore
    ///     <-> hot  <->
    /// hit             aot  
    ///     <-> ait  <->
    /// ```
    ///
    /// 另外，word_ladder中visited也是可以在for外再添加next_word，只是存在重复的到queue中了，但
    /// 这里`!visited.contains && !next_visited.contains`时插入queue也可以避免重复
    ///
    /// 所以可以在当前层用next_visited临时保存当前层与next_word的联系，不能直接visited.insert影响
    /// 当前层后面的word与next_word的关系。当前层与当前层word间联系也不能被记录，
    /// 由于同一层的连接肯定不是起点到终点的最短路径的边，所以使用 `next_visited: HashSet<String>`
    /// （避免同层word的next_words出现相同的当前层word，同层相连）
    ///
    /// ![](https://pic.leetcode-cn.com/da957ec0ab9fe9eda166acab86ba9fca8b6b0f6b04bda3e35677a46fb53d7d17-image.png)
    ///
    /// word_successors表示使用BFS遍历找到的最短路径中的word，每个word联系着最短路径中的next_words
    ///
    /// - 考虑第 k 层的某一个单词，如果这个单词在第 1 到 k-1 层已经出现过，我们其实就不过继续向下探索了。方式：
    ///   - distances map word <=> depth
    ///   - visited hashset word: 当保存之前遍历过的word。如果不存在set中才计算next words
    ///
    /// 参考：
    ///
    /// [单双向广度优先遍历 + 回溯算法（Java、Python）](https://leetcode-cn.com/problems/word-ladder-ii/solution/yan-du-you-xian-bian-li-shuang-xiang-yan-du-you--2/)
    ///
    /// ### Submissions
    ///
    /// date=20201231, mem=4.3, mem_beats=62, runtime=104, runtime_beats=37.5, url=https://leetcode-cn.com/submissions/detail/135135940/
    ///
    /// date=20210103, mem=3.6, mem_beats=92, runtime=108, runtime_beats=57, url=https://leetcode-cn.com/submissions/detail/135720501/
    ///
    /// date=20210104, mem=3.6, mem_beats=92, runtime=112, runtime_beats=50, url=https://leetcode-cn.com/submissions/detail/135893824/
    ///
    /// date=20210104, mem=3.8, mem_beats=65, runtime=108, runtime_beats=50, url=https://leetcode-cn.com/submissions/detail/137174798/
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

            fn _get_word_successors(
                begin_word: &str,
                end_word: &str,
                word_list: &HashSet<String>,
            ) -> HashMap<String, Vec<String>> {
                let mut word_successors = HashMap::new();

                let mut queue = VecDeque::new();
                queue.push_back(begin_word.to_string());

                let mut visited = HashSet::new();
                visited.insert(begin_word.to_string());
                // 当前层访问过的结点，当前层全部遍历完成以后，再添加到总的 visited 集合里
                let mut next_level_visited = HashSet::new();
                let mut is_found = false;

                while !queue.is_empty() {
                    for _ in 0..queue.len() {
                        let cur_word = queue.pop_front().unwrap();
                        for next_word in _get_next_words(&cur_word, word_list) {
                            if !visited.contains(&next_word) {
                                if next_word == end_word {
                                    is_found = true;
                                }
                                // 避免下层元素重复加入队列
                                if next_level_visited.insert(next_word.clone()) {
                                    queue.push_back(next_word.clone());
                                }
                                // 不能放在`if !visited.contains(&next_word) {`外，可导致
                                // 反向到上层的word重复添加
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
                    // 清空到visited中
                    next_level_visited.drain().for_each(|e| {
                        visited.insert(e);
                    });
                }

                word_successors
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

            let mut res = vec![];
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return res;
            }
            let word_successors = _get_word_successors(&begin_word, &end_word, &word_list);
            _backtrack(
                &begin_word,
                &end_word,
                &word_successors,
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
    /// 注意：
    ///
    /// 在双向交替添加word到word_successors中时，如果是从后面找的next_words
    /// cur_word=kot, next_words: [hot, kit]这时不能以前面的方向放入successors中
    ///
    /// ```ignore
    ///     <-> hot <->
    /// hit             kot
    ///     <-> kit <->
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20210103, mem=2.6, mem_beats=100, runtime=16, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/135723941/
    ///
    /// date=20210104, mem=2.6, mem_beats=100, runtime=28, runtime_beats=85, url=https://leetcode-cn.com/submissions/detail/135902953/
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
                // visited.insert(begin.to_string());
                // visited.insert(end.to_string());

                let (mut begin_visited, mut end_visited) = (HashSet::new(), HashSet::new());
                begin_visited.insert(begin.to_string());
                end_visited.insert(end.to_string());
                let mut is_forward = true;
                let mut is_found = false;


                // 在保证了 beginVisited 总是较小（可以等于）大小的集合前提下，
                // && !endVisited.isEmpty() 可以省略
                while !begin_visited.is_empty() && !is_found {
                    // 默认 beginVisited 是小集合
                    if begin_visited.len() > end_visited.len() {
                        std::mem::swap(&mut begin_visited, &mut end_visited);
                        // 每次改变都对应 begin_visited 的方向
                        is_forward = !is_forward;
                    }
                    // 临时保存当前层对应的下层words
                    let mut next_visited = HashSet::new();
                    for cur in begin_visited {
                        visited.insert(cur.clone());
                        for next in _get_next_words(&cur, word_list) {
                            if !visited.contains(&next) && !begin.contains(&next) {
                                if end_visited.contains(&next) {
                                    is_found = true;
                                }

                                next_visited.insert(next.clone());
                                let (cur, next) = if is_forward {
                                    (cur.clone(), next)
                                } else {
                                    (next, cur.clone())
                                };
                                word_successors.entry(cur)
                                    .or_insert(vec![])
                                    .push(next);
                            }
                        }
                    }
                    begin_visited = next_visited;
                }
                word_successors
            }


            // /// with bfs two end
            // fn _get_word_successors(
            //     begin: &str,
            //     end: &str,
            //     word_list: &HashSet<String>,
            // ) -> HashMap<String, Vec<String>> {
            //     let mut word_successors = HashMap::new();
            //
            //     let mut visited = HashSet::new();
            //     visited.insert(begin.to_string());
            //     visited.insert(end.to_string());
            //
            //     let (mut begin_visited, mut end_visited) = (HashSet::new(), HashSet::new());
            //     begin_visited.insert(begin.to_string());
            //     end_visited.insert(end.to_string());
            //     let mut is_forward = true;
            //     let mut is_found = false;
            //
            //     // 将word为key的successors中插入next_word。
            //     // 如果is_forward=false则反向插入：successors[next_word].push(word)
            //     let mut successors_insert =
            //         |mut word: String, mut next_word: String, is_forward: bool| {
            //             if !is_forward {
            //                 std::mem::swap(&mut word, &mut next_word);
            //             }
            //             word_successors
            //                 .entry(word)
            //                 .or_insert(vec![])
            //                 .push(next_word);
            //         };
            //     // 在保证了 beginVisited 总是较小（可以等于）大小的集合前提下，
            //     // && !endVisited.isEmpty() 可以省略
            //     while !begin_visited.is_empty() {
            //         // 默认 beginVisited 是小集合
            //         if begin_visited.len() > end_visited.len() {
            //             std::mem::swap(&mut begin_visited, &mut end_visited);
            //             // 每次改变都对应 begin_visited 的方向
            //             is_forward = !is_forward;
            //         }
            //         // 临时保存当前层对应的下层words
            //         let mut next_visited = HashSet::new();
            //         for word in begin_visited {
            //             for next_word in _get_next_words(&word, word_list) {
            //                 // 在另一侧找到单词以后，还需把这一层关系添加到「后继结点列表」
            //                 if end_visited.contains(&next_word) {
            //                     is_found = true;
            //                     // 当找到时必定存在`visited.contains(&next_word) = true`。end与begin
            //                     // visited相交表示visited之前已经相遇过
            //                     successors_insert(word.clone(), next_word.clone(), is_forward);
            //                 }
            //
            //                 if !visited.contains(&next_word) {
            //                     next_visited.insert(next_word.clone());
            //                     successors_insert(word.clone(), next_word.clone(), is_forward);
            //                 }
            //             }
            //         }
            //         if is_found {
            //             break;
            //         }
            //         next_visited.iter().for_each(|e| {
            //             visited.insert(e.clone());
            //         });
            //         begin_visited = next_visited;
            //     }
            //     word_successors
            // }

            let mut res = vec![];
            let word_list = word_list.into_iter().collect::<HashSet<_>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return res;
            }
            _backtrack(
                &begin_word,
                &end_word,
                dbg!(&_get_word_successors(&begin_word, &end_word, &word_list)),
                &mut vec![&begin_word],
                &mut res,
            );
            res
        }
    }
}

pub mod solution_bfs_simple {

    /// # 思路
    ///
    /// 在bfs的queue中保存完整的path，最先找到的path的level
    /// 将被添加到res中表示最短路径。
    ///
    /// 在遍历path最前的level的cur_word时，
    /// 如果其中的next_word==end_word，表示在当前level找到了
    /// 最短路径，只需要在当前level再找剩下的path
    ///
    /// [与c++版本相似的rust翻译版本](https://leetcode-cn.com/submissions/detail/137200279/)
    ///
    /// 注意在这里修改了更普通易读的版本，而不是快速省空间的方式
    ///
    /// 参考：
    ///
    /// - [C++ solution using standard BFS method, no DFS or backtracking](https://leetcode.com/problems/word-ladder-ii/discuss/40434/C%2B%2B-solution-using-standard-BFS-method-no-DFS-or-backtracking)
    ///
    /// ### submissions
    ///
    /// date=20210109, mem=7.7, mem_beats=5, runtime=492, runtime_beats=10, url=https://leetcode-cn.com/submissions/detail/137200279/
    pub struct Solution;

    impl Solution {
        pub fn find_ladders(
            begin_word: String,
            end_word: String,
            word_list: Vec<String>,
        ) -> Vec<Vec<String>> {
            use std::collections::{HashSet, VecDeque};

            let mut res = vec![];
            let word_list = word_list.into_iter().collect::<HashSet<String>>();
            if word_list.is_empty() || !word_list.contains(&end_word) {
                return res;
            }

            // path.len()作为当前的level
            let mut min_level = std::usize::MAX;
            // 保存当前的paths
            let mut queue = VecDeque::new();
            queue.push_back(vec![begin_word.clone()]);
            // 当前的访问过的word节点
            let mut visited = HashSet::new();
            visited.insert(begin_word.clone());
            // 下个level访问时保存word，避免path遗漏
            let mut next_visited = HashSet::new();

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let path = queue.pop_front().unwrap();
                    // 已找到最短level与paths，直接退出
                    if path.len() > min_level {
                        break;
                    }
                    // 当前level的word
                    let mut cur_word = path.last().map(|e| e.chars().collect::<Vec<_>>()).unwrap();
                    // 生成next word
                    for i in 0..cur_word.len() {
                        let old = cur_word[i];
                        for letter in 'a'..='z' {
                            cur_word[i] = letter;
                            let next_word = cur_word.iter().collect::<String>();
                            // next_word要合法 并 没有在上级出现 避免反向
                            if word_list.contains(&next_word) && !visited.contains(&next_word) {
                                // 连接next_word生成新路径 之前的path不需要了
                                let mut next_path = path.clone();
                                next_path.push(next_word.clone());

                                if next_word == end_word {
                                    // 当前level是最小的 最短路径path只能在这个level出现
                                    min_level = path.len();
                                    res.push(next_path);
                                } else {
                                    // 继续找新的path
                                    queue.push_back(next_path);
                                }
                                next_visited.insert(next_word);
                            };
                        }
                        cur_word[i] = old;
                    }
                }
                // 清空到visited
                next_visited.drain().for_each(|e| {
                    visited.insert(e);
                });
            }
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
        // test(solution_dfs_bfs::Solution::find_ladders);
        test(solution_dfs_bfs_two_end::Solution::find_ladders);
        // test(solution_bfs_simple::Solution::find_ladders);
    }

    fn test<F: Fn(String, String, Vec<String>) -> Vec<Vec<String>>>(func: F) {
        // assert_eq!(
        //     func(
        //         "hot".to_string(),
        //         "dog".to_string(),
        //         ["hot", "dog"].iter().map(|e| e.to_string()).collect()
        //     ),
        //     vec![] as Vec<Vec<String>>
        // );

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

        let res = func(
            "qa".to_string(),
            "sq".to_string(),
            [
                "si", "go", "se", "cm", "so", "ph", "mt", "db", "mb", "sb", "kr", "ln", "tm", "le",
                "av", "sm", "ar", "ci", "ca", "br", "ti", "ba", "to", "ra", "fa", "yo", "ow", "sn",
                "ya", "cr", "po", "fe", "ho", "ma", "re", "or", "rn", "au", "ur", "rh", "sr", "tc",
                "lt", "lo", "as", "fr", "nb", "yb", "if", "pb", "ge", "th", "pm", "rb", "sh", "co",
                "ga", "li", "ha", "hz", "no", "bi", "di", "hi", "qa", "pi", "os", "uh", "wm", "an",
                "me", "mo", "na", "la", "st", "er", "sc", "ne", "mn", "mi", "am", "ex", "pt", "io",
                "be", "fm", "ta", "tb", "ni", "mr", "pa", "he", "lr", "sq", "ye",
            ]
            .iter()
            .map(|e| e.to_string())
            .collect(),
        );
        let expected: Vec<Vec<String>> = [
            ["qa", "ba", "be", "se", "sq"],
            ["qa", "ba", "bi", "si", "sq"],
            ["qa", "ba", "br", "sr", "sq"],
            ["qa", "ca", "cm", "sm", "sq"],
            ["qa", "ca", "co", "so", "sq"],
            ["qa", "la", "ln", "sn", "sq"],
            ["qa", "la", "lt", "st", "sq"],
            ["qa", "ma", "mb", "sb", "sq"],
            ["qa", "pa", "ph", "sh", "sq"],
            ["qa", "ta", "tc", "sc", "sq"],
            ["qa", "fa", "fe", "se", "sq"],
            ["qa", "ga", "ge", "se", "sq"],
            ["qa", "ha", "he", "se", "sq"],
            ["qa", "la", "le", "se", "sq"],
            ["qa", "ma", "me", "se", "sq"],
            ["qa", "na", "ne", "se", "sq"],
            ["qa", "ra", "re", "se", "sq"],
            ["qa", "ya", "ye", "se", "sq"],
            ["qa", "ca", "ci", "si", "sq"],
            ["qa", "ha", "hi", "si", "sq"],
            ["qa", "la", "li", "si", "sq"],
            ["qa", "ma", "mi", "si", "sq"],
            ["qa", "na", "ni", "si", "sq"],
            ["qa", "pa", "pi", "si", "sq"],
            ["qa", "ta", "ti", "si", "sq"],
            ["qa", "ca", "cr", "sr", "sq"],
            ["qa", "fa", "fr", "sr", "sq"],
            ["qa", "la", "lr", "sr", "sq"],
            ["qa", "ma", "mr", "sr", "sq"],
            ["qa", "fa", "fm", "sm", "sq"],
            ["qa", "pa", "pm", "sm", "sq"],
            ["qa", "ta", "tm", "sm", "sq"],
            ["qa", "ga", "go", "so", "sq"],
            ["qa", "ha", "ho", "so", "sq"],
            ["qa", "la", "lo", "so", "sq"],
            ["qa", "ma", "mo", "so", "sq"],
            ["qa", "na", "no", "so", "sq"],
            ["qa", "pa", "po", "so", "sq"],
            ["qa", "ta", "to", "so", "sq"],
            ["qa", "ya", "yo", "so", "sq"],
            ["qa", "ma", "mn", "sn", "sq"],
            ["qa", "ra", "rn", "sn", "sq"],
            ["qa", "ma", "mt", "st", "sq"],
            ["qa", "pa", "pt", "st", "sq"],
            ["qa", "na", "nb", "sb", "sq"],
            ["qa", "pa", "pb", "sb", "sq"],
            ["qa", "ra", "rb", "sb", "sq"],
            ["qa", "ta", "tb", "sb", "sq"],
            ["qa", "ya", "yb", "sb", "sq"],
            ["qa", "ra", "rh", "sh", "sq"],
            ["qa", "ta", "th", "sh", "sq"],
        ]
        .iter()
        .map(|a| a.iter().map(|e| e.to_string()).collect())
        .collect();

        assert_eq!(res.len(), expected.len());
        assert!(is_contains_vec2(&res, &expected));
    }
}
