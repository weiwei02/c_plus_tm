// https://leetcode.cn/problems/decode-string/description/?envType=study-plan-v2&envId=top-100-liked
// 394. 字符串解码 中等
// 
// 给定一个经过编码的字符串，返回它解码后的字符串。
// 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
// 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
// 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
// 
// 示例 1：
// 输入：s = "3[a]2[bc]"
// 输出："aaabcbc"
// 示例 2：
// 
// 输入：s = "3[a2[c]]"
// 输出："accaccacc"

// 示例 3：
// 输入：s = "2[abc]3[cd]ef"
// 输出："abcabccdcdcdef"
// 
// 示例 4：
// 输入：s = "abc3[cd]xyz"
// 输出："abccdcdcdxyz"
// 
// 提示：
// 1 <= s.length <= 30
// s 由小写英文字母、数字和方括号 '[]' 组成
// s 保证是一个 有效 的输入。
// s 中所有整数的取值范围为 [1, 300] 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        assert_eq!(Solution::decode_string(String::from("3[a]2[bc]")), "aaabcbc");
        assert_eq!(Solution::decode_string(String::from("3[a2[c]]")), "accaccacc");
        assert_eq!(Solution::decode_string(String::from("2[abc]3[cd]ef")), "abcabccdcdcdef");
        assert_eq!(Solution::decode_string(String::from("abc3[cd]xyz")), "abccdcdcdxyz");
        
        assert_eq!(Solution::decode_string(String::from("100[leetcode]")), "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode");
    }
}

struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let sbytes = s.as_bytes();
        let mut stack = Vec::new();
        let mut i = 0;
        while i < sbytes.len() {
            if sbytes[i] == b']' {
                let mut tmp = String::new();
                while let Some(c) = stack.pop() {
                    match c {
                        b'[' => break,
                        _ => tmp.push(c as char),
                    }
                }

                // count before '['
                let mut num_tmp = String::new();
                while let Some(c) = stack.pop() {
                    if c >= b'0' && c <= b'9' {
                        num_tmp.insert(0, c as char);
                    } else {
                        stack.push(c);
                        break;
                    }
                }
                let count = num_tmp.chars().collect::<String>().parse::<usize>().unwrap_or(0);

                tmp = tmp.chars().rev().collect::<String>();
                for _ in 0..count {
                    for c in tmp.as_bytes() {
                        stack.push(*c);
                    }
                }
            } else {
                stack.push(sbytes[i]);
            }
            i += 1;
        } 

        // trance stack to string
        stack.iter().map(|c| *c as char).collect::<String>()
               
    }
}