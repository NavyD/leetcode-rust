#[allow(unused)]
mod solution_node {
    /// 参考：
    ///
    /// * [【宫水三叶】一题双解 :「二维数组」&「TrieNode」](https://leetcode-cn.com/problems/implement-trie-prefix-tree/solution/gong-shui-san-xie-yi-ti-shuang-jie-er-we-esm9/)
    /// * [Trie Tree 的实现 (适合初学者)🌳](https://leetcode-cn.com/problems/implement-trie-prefix-tree/solution/trie-tree-de-shi-xian-gua-he-chu-xue-zhe-by-huwt/)
    /// * [Rust Solution 8ms beats 100.00%](https://leetcode.com/problems/implement-trie-prefix-tree/discuss/301435/Rust-Solution-8ms-beats-100.00)
    ///
    /// date=20220329, mem=13.3, mem_beats=88, runtime=12, runtime_beats=100
    ///
    struct Trie {
        root: Node,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl Trie {
        fn new() -> Self {
            Self {
                root: Default::default(),
            }
        }

        fn insert(&mut self, word: String) {
            let cur = word.bytes().map(index).fold(&mut self.root, |cur, i| {
                cur.children[i].get_or_insert_with(|| Box::new(Default::default()))
            });
            cur.is_end = true;
        }

        fn search(&self, word: String) -> bool {
            word.bytes()
                .map(index)
                .try_fold(&self.root, |cur, i| cur.children[i].as_deref())
                .map(|node| node.is_end)
                .unwrap_or(false)
        }

        fn starts_with(&self, prefix: String) -> bool {
            prefix
                .bytes()
                .map(index)
                .try_fold(&self.root, |cur, i| cur.children[i].as_deref())
                .is_some()
        }
    }

    #[derive(Default)]
    struct Node {
        is_end: bool,
        children: [Option<Box<Node>>; 26],
    }

    #[inline(always)]
    fn index(c: u8) -> usize {
        (c - b'a') as usize
    }

    #[test]
    fn test() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string())); // 返回 True
        assert!(!trie.search("app".to_string())); // 返回 False
        assert!(trie.starts_with("app".to_string())); // 返回 True
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string())); // 返回 True
    }
}
