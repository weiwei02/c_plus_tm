// 127. Word Ladder
// https://leetcode.cn/problems/word-ladder/?envType=study-plan-v2&envId=top-interview-150
//
// 单词接龙 困难
//
// 字典 wordList 中从单词 beginWord 和 endWord 的 转换序列 是一个按下述规格形成的序列
// beginWord -> s1 -> s2 -> ... -> sk：
// * 每一对相邻的单词只差一个字母。
// * 对于 1 <= i <= k 时，每个 si 都在 wordList 中。注意， beginWord 不需要在 wordList 中。
// * sk == endWord
// 给你两个单词 beginWord 和 endWord 和一个字典 wordList ，返回 从 beginWord 到 endWord 的 最短转换序列 中的 单词数目 。
// 如果不存在这样的转换序列，返回 0 。

// 示例 1：
// 输入：beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
// 输出：5
// 解释：一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog", 返回它的长度 5。

// 示例 2：
// 输入：beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
// 输出：0
// 解释：endWord "cog" 不在字典中，所以无法进行转换。

// 提示：
// 1 <= beginWord.length <= 10
// endWord.length == beginWord.length
// 1 <= wordList.length <= 5000
// wordList[i].length == beginWord.length
// beginWord、endWord 和 wordList[i] 由小写英文字母组成
// beginWord != endWord
// wordList 中的所有字符串 互不相同

use std::{cell::RefCell, collections::HashMap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladder_length() {
        assert_eq!(
            Solution::ladder_length(
                String::from("hit"),
                String::from("cog"),
                vec![
                    String::from("hot"),
                    String::from("dot"),
                    String::from("dog"),
                    String::from("lot"),
                    String::from("log"),
                    String::from("cog"),
                ]
            ),
            5
        );

        assert_eq!(
            Solution::pre_ladder_length(
                "red",
                "ted",
                vec!["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"]
            ),
            2
        );

        assert_eq!(
            Solution::pre_ladder_length(
                "red",
                "tad",
                vec!["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"]
            ),
            3
        );

        assert_eq!(
            Solution::pre_ladder_length(
                "red",
                "tax",
                vec!["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"]
            ),
            4
        );
        assert_eq!(
            Solution::pre_ladder_length(
                "red",
                "tex",
                vec!["ted", "tex", "red", "tax", "tad", "den", "rex", "pee"]
            ),
            3
        );
    }
}

// 解决题目的关键在于题目中的条件：
// * 每次只能换一个字符
// * 每次中间转换过程都在hash表中
// * 所有单词长度都一致
struct Solution;
impl Solution {
    pub fn pre_ladder_length(begin_word: &str, end_word: &str, word_list: Vec<&str>) -> i32 {
        let begin = String::from(begin_word);
        let end = String::from(end_word);
        let word_list: Vec<String> = word_list.iter().map(|x| String::from(*x)).collect();
        Solution::ladder_length(begin, end, word_list)
    }
    // 首先为了方便表示，我们先给每一个单词标号，即给每个单词分配一个 id。
    // 创建一个由单词 word 到 id 对应的映射 wordId，并将 beginWord 与 wordList
    // 中所有的单词都加入这个映射中。之后我们检查 endWord 是否在该映射内，
    // 若不存在，则输入无解。我们可以使用哈希表实现上面的映射关系。
    // 然后我们需要建图，依据朴素的思路，我们可以枚举每一对单词的组合，
    // 判断它们是否恰好相差一个字符，以判断这两个单词对应的节点是否能够相连。
    // 但是这样效率太低，我们可以优化建图。
    //
    // 具体地，我们可以创建虚拟节点。对于单词 hit，我们创建三个虚拟节点 *it、h*t、hi*，
    // 并让 hit 向这三个虚拟节点分别连一条边即可。如果一个单词能够转化为 hit，
    // 那么该单词必然会连接到这三个虚拟节点之一。
    // 对于每一个单词，我们枚举它连接到的虚拟节点，把该单词对应的 id 与这些虚拟节点对应的 id 相连即可。
    //
    // 最后我们将起点加入队列开始广度优先搜索，当搜索到终点时，我们就找到了最短路径的长度。
    // 注意因为添加了虚拟节点，所以我们得到的距离为实际最短路径长度的两倍。
    // 同时我们并未计算起点对答案的贡献，所以我们应当返回距离的一半再加一的结果。
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_map: HashMap<String, i32> = HashMap::with_capacity(word_list.len() * 2 + 1);
        let word_map = RefCell::new(word_map);
        let graph: Vec<Vec<usize>> = Vec::new();
        let graph = RefCell::new(graph);

        let get_word_id = |word: &String| {
            if let Some(id) = word_map.borrow().get(word) {
                return *id as usize;
            }

            let id = word_map.borrow().len();
            let id = id.clone();

            word_map.borrow_mut().insert(word.clone(), id as i32);
            graph.borrow_mut().push(vec![]);
            id
        };
        let add_edge = |s: &String| {
            let id1 = get_word_id(s);
            // let word = s.clone();
            let mut word_bytes = s.as_bytes().to_vec().clone();
            for i in 0..word_bytes.len() {
                let tmp = word_bytes[i];
                word_bytes[i] = b'*';
                let mut new_str = String::new();
                word_bytes.clone_into(unsafe { new_str.as_mut_vec() });
                let id2 = get_word_id(&new_str);
                if !graph.borrow_mut()[id1].contains(&id2) {
                    graph.borrow_mut()[id1].push(id2);
                }
                if !graph.borrow_mut()[id2].contains(&id1) {
                    graph.borrow_mut()[id2].push(id1);
                }
                word_bytes[i] = tmp;
            }
        };

        word_list.iter().for_each(|x| {
            add_edge(x);
        });
        add_edge(&begin_word);
        if !word_map.borrow().contains_key(&end_word) {
            return 0;
        }

        let begin_id = get_word_id(&begin_word);
        let end_id = get_word_id(&end_word);
        // 广度优先搜索
        let mut queue = vec![begin_id];
        let mut distance: Vec<i32> = vec![-1; word_map.borrow().len()];
        distance[begin_id] = 0;

        // for debug
        // let mut id_map = vec![String::new(); word_map.borrow().len()];
        // word_map.borrow().iter().for_each(|(k, &v)| {id_map[v as usize] = k.clone()}); 

        loop {
            let id = queue.remove(0);
            let dis = distance[id];

            // if end_word == "ted" {
            //     println!("id = {}: {}, dist = {}", id, id_map[id], dis);
            // }

            // 最短路径搜索到end节点，可以提前结束
            if id == end_id {
                // println!("map = {:?}", word_map.borrow());
                // println!("graph = {:?}", graph.borrow());
                // println!("distance = {:?}", distance);
                return dis / 2 + 1;
            }

            let disgra = &graph.borrow()[id];
            for next_id in disgra {
                let next_id = next_id.clone();
                let new_dis = dis + 1;
                if distance[next_id] == -1 {
                    distance[next_id] = new_dis;
                    // if end_word == "ted" {
                    //     println!("push id = {}", next_id);
                    // }
                    queue.push(next_id);
                }
            }
            if queue.is_empty() {
                break;
            }
        }
        0
    }

    // 朴素解法
    // 思路1：暴力法解题过程可以推测如下
    // 1. 检索目标单次是否在表中，如目标单词不在表中直接返回失败
    // 2. 创建一个字符串比较函数compare_str 。函数比较两个字符串不相同的字符数
    // 3. 创建一个map，map的key是start和表中的单词，但不包括end，值是空列表
    // 4. 遍历单词表，表中的每个元素都与map的key比较，如果不同的字符等于1，则在map[key]中记录一次该单词。
    // 5. 构建 map 完成
    // 6. 从start处开始从map中取元素 a，并记录层级 c
    // 7. 如果a != end， 则从 a 中再尝试取元素 a[c]
    // 8. 如果a 中没有元素，则返回0
    // 9. 如果 a 中的元素恰好包括 end 则返回c
    // 10. 最后比较多个c，取其中最小的元素
    // 11. 注意防止递归回原单词，造vec!成无限循环，可以考虑为每条路径单独建立一个栈，辅助查询
    pub fn ladder_length2(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_map: HashMap<String, Vec<String>> = HashMap::with_capacity(word_list.len());
        word_map.insert(begin_word.clone(), vec![]);
        word_list.iter().for_each(|word| {
            word_map.insert(word.clone(), vec![]);
        });
        0
    }

    fn compare_str(s1: &str, s2: &str) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut len = 0;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                len += 1;
            }
        }
        len
    }
}
