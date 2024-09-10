// https://leetcode.cn/problems/implement-trie-prefix-tree
// 实现 Trie (前缀树)
// 中等
// Trie（发音类似 "try"）或者说 前缀树 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补全和拼写检查。
// 请你实现 Trie 类：
// Trie() 初始化前缀树对象。
// void insert(String word) 向前缀树中插入字符串 word 。
// boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回 false 。
// boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true ；否则，返回 false 。

// 示例：
// 输入
// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
// 输出
// [null, null, true, false, true, null, true]
// 解释
// Trie trie = new Trie();
// trie.insert("apple");
// trie.search("apple");   // 返回 True
// trie.search("app");     // 返回 False
// trie.startsWith("app"); // 返回 True
// trie.insert("app");
// trie.search("app");     // 返回 True

// 提示：
// 1 <= word.length, prefix.length <= 2000
// word 和 prefix 仅由小写英文字母组成
// insert、search 和 startsWith 调用次数 总计 不超过 3 * 104 次

use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    childen: HashMap<char, Trie>,
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
impl Trie {
    fn new() -> Self {
        Trie {
            childen: HashMap::new(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.childen.entry(c).or_insert_with(|| Self::new());
        }
        cur.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;
        for c in word.chars() {
            if !node.childen.contains_key(&c) {
                return false;
            }
            node = node.childen.get(&c).unwrap();
        }
        return node.is_end;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            if !node.childen.contains_key(&c) {
                return false;
            }
            node = node.childen.get(&c).unwrap();
        }
        return true;
    }
}
