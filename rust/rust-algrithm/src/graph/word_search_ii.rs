// https://leetcode.cn/problems/word-search-ii
// 单词搜索 II
// 困难
// 给定一个 m x n 二维字符网格 board 和一个单词（字符串）列表 words， 返回所有二维网格上的单词 。
// 单词必须按照字母顺序，通过 相邻的单元格 内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母在一个单词中不允许被重复使用。

// 示例 1：
// 输入：board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
// 输出：["eat","oath"]

// 示例 2：
// 输入：board = [["a","b"],["c","d"]], words = ["abcb"]
// 输出：[]

// 提示：
// m == board.length
// n == board[i].length
// 1 <= m, n <= 12
// board[i][j] 是一个小写英文字母
// 1 <= words.length <= 3 * 10^4
// 1 <= words[i].length <= 10
// words[i] 由小写英文字母组成
// words 中的所有字符串互不相同

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'b', 'n'],
                    vec!['o', 't', 'a', 'e'],
                    vec!['a', 'h', 'k', 'r'],
                    vec!['a', 'f', 'l', 'v'],
                ],
                vec![String::from("oaa"), String::from("oa")],
            ),
            vec![String::from("oa"), String::from("oaa")]
        );
        assert_eq!(
            Solution::find_words(vec![vec!['a']], vec![String::from("a")]),
            vec![String::from("a")],
        );

        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v'],
                ],
                vec![
                    String::from("oath"),
                    String::from("pea"),
                    String::from("eat"),
                    String::from("rain")
                ]
            ),
            vec![String::from("eat"), String::from("oath")]
        );

        assert_eq!(
            Solution::find_words(
                vec![vec!['a', 'b'], vec!['c', 'd']],
                vec![String::from("abcb")]
            ),
            Vec::<String>::new(),
        );
    }
}

struct Solution;
impl Solution {
    // 单词搜索 II
    // 思路1： 字典数
    // 1. 构建字典树
    // 1.1 构建字典树，每个节点存储一个字符，每个节点有26个子节点
    // 2. 遍历每个单词，判断是否在字典树中
    // 3. 如果在，则加入结果集
    // 4. 如果不在，则跳过
    // 遍历矩阵的方法
    // 使用深度优先搜索，遍历矩阵中的每个元素，如果该元素在字典树中，则加入结果集
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let trie = Trie::build_trie(&words);
        let mut result: HashSet<String> = HashSet::new();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::deep_first_search(&mut board, i as i32, j as i32, &trie, &mut result);
            }
        }
        let mut re = result
            .iter()
            .map(|key| key.clone())
            .collect::<Vec<String>>();
        re.sort_by(|a, b| a.cmp(b));
        re
    }

    fn deep_first_search(
        board: &mut Vec<Vec<char>>,
        i: i32,
        j: i32,
        node: &Trie,
        result: &mut HashSet<String>,
    ) {
        for (x, y) in [(i, j), (i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if x < board.len() as i32 && x >= 0 && y < board[0].len() as i32 && y >= 0 {
                let (x, y) = (x as usize, y as usize);
                let c = board[x][y];
                if c == '#' {
                    continue;
                }
                board[x][y] = '#';
                if let Some(child) = node.children[(c as u8 - b'a') as usize].as_ref() {
                    // 如果该字符在字典树中，则加入结果集
                    if !child.word.is_empty() {
                        result.insert(child.word.to_string());
                    }
                    Self::deep_first_search(board, x as i32, y as i32, child, result);
                }
                board[x][y] = c;
            }
        }
    }
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    // 为了方便统计结果，在每个单词结束时，记录该单词
    word: String,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn build_trie(words: &Vec<String>) -> Self {
        let mut trie = Self::new();
        for word in words {
            let mut node = &mut trie;
            for c in word.chars() {
                if node.children[(c as u8 - b'a') as usize].is_none() {
                    node.children[(c as u8 - b'a') as usize] = Some(Box::new(Trie::new()));
                }
                node = node.children[(c as u8 - b'a') as usize].as_mut().unwrap();
            }
            // 标记该节点为单词结尾
            node.word = word.clone();
        }
        trie
    }
}
