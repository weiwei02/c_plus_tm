// https://leetcode.cn/problems/longest-palindromic-substring/?envType=study-plan-v2&envId=top-interview-150
// 5. 最长回文子串
// 中等
// 给你一个字符串 s，找到 s 中最长的回文子串
// 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。

// 示例 1：
// 输入：s = "babad"
// 输出："bab"
// 解释："aba" 同样是符合题意的答案。

// 示例 2：
// 输入：s = "cbbd"
// 输出："bb"

// 提示：
// 1 <= s.length <= 1000
// s 仅由数字和英文字母组成

#[cfg(test)]
mod tests {
    use super::*;
}

struct Solution;
impl Solution {
    // 最长回文子串
    // 思路：
    // 判断是否回文字符串的方法：
    // 1， 选定开始和结束位置，判断是否相等
    // 2. 如果相等，则把开始和结束位置向中间移动一位，判断是否相等
    // 3. 直到开始和结束位置相等，或者开始位置大于结束位置，结束循环。
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();

        let mut dp = vec![vec![true; n]; n];
        let mut res = (0, 0);

        for k in 1..n {
            for i in 0..(n - k) {
                if k == 1 {
                    dp[i][i + k] = s[i] == s[i + 1];
                } else {
                    dp[i][i + k] = (s[i] == s[i + k]) && dp[i + 1][i + k - 1];
                }

                if dp[i][i + k] {
                    res = (i, i + k);
                }
            }
        }

        s[res.0..=res.1].into_iter().collect::<String>()
    }
}
