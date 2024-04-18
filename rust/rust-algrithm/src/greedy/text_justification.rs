// https://leetcode.cn/problems/text-justification/?envType=study-plan-v2&envId=top-interview-150
// 68. 文本左右对齐
// 困难
// 给定一个单词数组 words 和一个长度 maxWidth ，重新排版单词，使其成为每行恰好有 maxWidth 个字符，且左右两端对齐的文本。
// 你应该使用 “贪心算法” 来放置给定的单词；也就是说，尽可能多地往每行中放置单词。必要时可用空格 ' ' 填充，使得每行恰好有 maxWidth 个字符。

// 要求尽可能均匀分配单词间的空格数量。如果某一行单词间的空格不能均匀分配，则左侧放置的空格数要多于右侧的空格数。

// 文本的最后一行应为左对齐，且单词之间不插入额外的空格。

// 注意:

// 单词是指由非空格字符组成的字符序列。
// 每个单词的长度大于 0，小于等于 maxWidth。
// 输入单词数组 words 至少包含一个单词。

// 示例 1:

// 输入: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
// 输出:
// [
//    "This    is    an",
//    "example  of text",
//    "justification.  "
// ]
// 示例 2:

// 输入:words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
// 输出:
// [
//   "What   must   be",
//   "acknowledgment  ",
//   "shall be        "
// ]
// 解释: 注意最后一行的格式应为 "shall be    " 而不是 "shall     be",
//      因为最后一行应为左对齐，而不是左右两端对齐。
//      第二行同样为左对齐，这是因为这行只包含一个单词。
// 示例 3:

// 输入:words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"]，maxWidth = 20
// 输出:
// [
//   "Science  is  what we",
//   "understand      well",
//   "enough to explain to",
//   "a  computer.  Art is",
//   "everything  else  we",
//   "do                  "
// ]

// 提示:
// 1 <= words.length <= 300
// 1 <= words[i].length <= 20
// words[i] 由小写英文字母和符号组成
// 1 <= maxWidth <= 100
// words[i].length <= maxWidth
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_justify() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_string(),
                    "is".to_string(),
                    "an".to_string(),
                    "example".to_string(),
                    "of".to_string(),
                    "text".to_string(),
                    "justification.".to_string()
                ],
                16
            ),
            vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string()
            ]
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_string(),
                    "must".to_string(),
                    "be".to_string(),
                    "acknowledgment".to_string(),
                    "shall".to_string(),
                    "be".to_string()
                ],
                16
            ),
            vec![
                "What   must   be".to_string(),
                "acknowledgment  ".to_string(),
                "shall be        ".to_string()
            ]
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_string(),
                    "is".to_string(),
                    "what".to_string(),
                    "we".to_string(),
                    "understand".to_string(),
                    "well".to_string(),
                    "enough".to_string(),
                    "to".to_string(),
                   "explain".to_string(),
                    "to".to_string(),
                    "a".to_string(),
                    "computer.".to_string(),
                    "Art".to_string(),
                    "is".to_string(),
                    "everything".to_string(),
                    "else".to_string(),
                    "we".to_string(),
                    "do".to_string()
                ],
                20
            ),
            vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string()
            ]);
    }
}

struct Solution;

const SPACE: &str = " ";
impl Solution {
    // 左右对齐思路
    // 大思路： 1. 使用贪心算法，把尽可能多的单词放在一行中，每个单词中间最少1个空格。
    //         2. 居中对齐算法
    //          3. 左对齐算法

    // 贪心算法：
    // 1. 遍历单词数组，开始索引记为start，当前索引记为 i,
    // 已遍历的单词总长度 sum2 = 单词数量wcount 即 (i - start) - 1 + 所有单词长度和sum1，大于max_width，则使用 i - 1 个 单词组成一句

    // 2. 居中对齐算法
    // 如果 nspace = (max_width - sum1) / (wcount - 1)   则每个单词间都要填充 nspace 个空格
    // 如果 osapce (max_width - sum1) % (wcount - 1)， 则余下的空格都放在第一个单词后
    //
    // 3. 左对齐算法
    // 每个单词间都要填充 1 个空格，且最后一个单词后，全部用空格填充到max_width
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let strs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let mut result = vec![];

        let max_width = max_width as usize;
        let mut start = 0;
        let mut sum = 0;
        for i in 0..strs.len() {
            sum += strs[i].len();
            let word_count = i - start + 1;
            if i < strs.len() - 1 && sum + strs[i + 1].len() + (word_count) > max_width {
                // 开始居中对齐算法
                let mut line = String::new();
                let space_count = max_width - sum;
                let space_num = if word_count == 1 {
                    0
                } else {
                    space_count / (word_count - 1)
                };

                let left_space_count = if word_count > 1 {
                    space_count % (word_count - 1)
                } else {
                    space_count
                };

                for j in start..i {
                    line.push_str(strs[j]);
                    for _ in 0..space_num {
                        line.push_str(SPACE);
                    }
                    // 注意 左右不相等时，把多出的空格从左往右放,每个单词各放1个
                    if j < start + left_space_count {
                        line.push_str(SPACE);
                    }
                }
                line.push_str(strs[i]);
                if start == i {
                    for _ in 0..left_space_count {
                        line.push_str(SPACE)
                    }
                }

                // println!("line: {}, space_num: {}", line, space_num);
                result.push(line);
                start = i + 1;
                sum = 0;
            } else if i == strs.len() - 1 {
                let mut line = String::new();
                for j in start..=i {
                    line.push_str(strs[j]);
                    if j != i {
                        line.push_str(SPACE);
                    }
                }
                for _ in 0..max_width - line.len() {
                    line.push_str(SPACE);
                }
                result.push(line);
            }
        }

        result
    }
}
