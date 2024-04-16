// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/?envType=study-plan-v2&envId=top-interview-150
// 30. 串联所有单词的子串 困难
// 给定一个字符串 s 和一个字符串数组 words。 words 中所有字符串 长度相同。
//  s 中的 串联子串 是指一个包含  words 中所有字符串以任意顺序排列连接起来的子串。
//  例如，如果 words = ["ab","cd","ef"]， 那么 "abcdef"， "abefcd"，"cdabef"， "cdefab"，"efabcd"，
// 和 "efcdab" 都是串联子串。 "acdbef" 不是串联子串，因为他不是任何 words 排列的连接。
//  返回所有串联子串在 s 中的开始索引。你可以以 任意顺序 返回答案。

//  示例 1：
//  输入：s = "barfoothefoobarman", words = ["foo","bar"]
//  输出：[0,9]
//  解释：因为 words.length == 2 同时 words[i].length == 3，连接的子字符串的长度必须为 6。
//  子串 "barfoo" 开始位置是 0。它是 words 中以 ["bar","foo"] 顺序排列的连接。
//  子串 "foobar" 开始位置是 9。它是 words 中以 ["foo","bar"] 顺序排列的连接。
//  输出顺序无关紧要。返回 [9,0] 也是可以的。

//  示例 2：
//  输入：s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
//  输出：[]
//  解释：因为 words.length == 4 并且 words[i].length == 4，所以串联子串的长度必须为 16。
//  s 中没有子串长度为 16 并且等于 words 的任何顺序排列的连接。
//  所以我们返回一个空数组。

//  示例 3：
//  输入：s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
//  输出：[6,9,12]
//  解释：因为 words.length == 3 并且 words[i].length == 3，所以串联子串的长度必须为 9。
//  子串 "foobarthe" 开始位置是 6。它是 words 中以 ["foo","bar","the"] 顺序排列的连接。
//  子串 "barthefoo" 开始位置是 9。它是 words 中以 ["bar","the","foo"] 顺序排列的连接。
//  子串 "thefoobar" 开始位置是 12。它是 words 中以 ["the","foo","bar"] 顺序排列的连接。

//  提示：
//  1 <= s.length <= 104
//  1 <= words.length <= 5000
//  1 <= words[i].length <= 30
//  words[i] 和 s 由小写英文字母组成

use std::{collections::HashMap};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_re_match() {
        assert_eq!(
            Solution::find_substring_for_test("wordgoodgoodgoodbestword", 
            vec!["word","good","best","good"]),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring_for_test("barfoothefoobarman", vec!["foo", "bar"]),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring_for_test(
                "wordgoodgoodgoodbestword",
                vec!["word", "good", "best", "word"]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring_for_test(
                "barfoofoobarthefoobarman",
                vec!["bar", "foo", "the"]
            ),
            vec![6, 9, 12]
        );
    }
}

struct Solution;
impl Solution {
    pub fn find_substring_for_test(s: &str, words: Vec<&str>) -> Vec<i32> {
        Solution::find_substring(
            s.to_string(),
            words.iter().map(|&s| s.to_string()).collect(),
        )
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut smap: HashMap<&str, i32> = HashMap::new();
        words.iter().for_each(|word| {
            let word = word.as_str();
            if let Some(count) = smap.get_mut(word) {
                *count += 1;
            } else {
                smap.insert(word, 1);
            }
        });
        let mut res = Vec::new();

        let (wslen, wlen) = (words.len(), words[0].len());
        let sub_str_len = wlen * wslen;

        if s.len() < sub_str_len {
            return res;
        }
        for i in 0..s.len() {
            if i + sub_str_len > s.len() {
                break;
            }

            let mut str_count = HashMap::new();
            let mut mat = true;
            for j in 0..wslen {
                let word = &s[i + j * wlen..i + (j + 1) * wlen];
                if let Some(&oc) = smap.get(word) {
                    if let Some(count) = str_count.get_mut(word) {
                        *count += 1;
                        // 如果单词大于已有单词，则跳出循环
                        if *count > oc {
                            mat = false;
                            break;
                        }
                    } else {
                        str_count.insert(word, 1);
                    }
                } else {
                    // word 不在 words 中
                    mat = false;
                    break;
                }
            }

            // println!("str_count: {:?}", str_count);
            if mat {
                res.push(i as i32);
            }
        }
        res
    }
}
