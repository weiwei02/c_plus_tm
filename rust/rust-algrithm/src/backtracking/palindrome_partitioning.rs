// https://leetcode.cn/problems/palindrome-partitioning/
// 131. 分割回文串
// 中等
// 给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是
// 回文串。
// 返回 s 所有可能的分割方案。

// 示例 1：
// 输入：s = "aab"
// 输出：[["a","a","b"],["aa","b"]]
// 示例 2：

// 输入：s = "a"
// 输出：[["a"]]

// 提示：
// 1 <= s.length <= 16
// s 仅由小写英文字母组成

mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            partition_to_test("aab"),
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ]
        );
        assert_eq!(partition_to_test("a"), vec![vec!["a".to_string()],]);
    }

    fn partition_to_test(s: &str) -> Vec<Vec<String>> {
        Solution::partition(String::from(s))
    }
}

struct Solution;
impl Solution {
    // 1. 要求每个子串都是回文串
    // 2. 回文串的长度是奇数，偶数个字符
    // 3. 每个字母本身是一个回文串
    // 4. 再从字母中组合成回文串
    // 5. 组合的方法是:偶数 s1..si = sn..si+1
    // 6. 奇数 s1..s(i-1) = sn..si+1
    // 回溯
    // 1. 假设我们当前搜索到字符串的第 i 个字符，且 s[0..i−1] 位置的所有字符已经被分割成若干个回文串，
    //   并且分割结果被放入了答案数组 `ans` 中，那么我们就需要枚举下一个回文串的右边界 j，使得 s[i..j] 是一个回文串。
    // 2. 因此，我们可以从 i 开始，从小到大依次枚举 j。对于当前枚举的 j 值，我们使用双指针的方法判断 s[i..j] 是否为回文串：
    // 3. 如果 s[i..j] 是回文串，那么就将其加入答案数组 ans 中，并以 j+1 作为新的 i 进行下一层搜索，并在未来的回溯时将 s[i..j] 从 ans 中移除。
    // 4. 如果我们已经搜索完了字符串的最后一个字符，那么就找到了一种满足要求的分割方法。
    // 细节
    // 5. 当我们在判断 s[i..j] 是否为回文串时，常规的方法是使用双指针分别指向 i 和 j，每次判断两个指针指向的字符是否相同，直到两个指针相遇。然而这种方法会产生重复计算，例如下面这个例子：
    // 5.1 当 s=aabas时，对于前 2 个字符 aa，我们有 2 种分割方法 [aa] 和 [a,a]，当我们每一次搜索到字符串的第 i=2 个字符 b 时，都需要对于每个 s[i..j] 使用双指针判断其是否为回文串，这就产生了重复计算。
    // 6.我们可以使用动态规划的方法进行优化，即预处理出字符串 s 中每个字符是否为回文串。
    // 5.1 设 f(i,j) 表示 s[i..j] 是否为回文串，那么有状态转移方程：
    // f(i,j) = (s[i]==s[j]) && f(i+1, j-1)
    // 其中 && 代表逻辑与运算符。即 s[i..j] 为回文串，当且仅当其为空串（i>j），其长度为 1（i=j），或者首尾字符相同且 s[i+1..j−1] 为回文串。
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut dp = vec![vec![true; s.len()]; s.len()];
        Solution::init_dp(&s, &mut dp);
        let mut tmp: Vec<String> = vec![];
        Solution::backtrack_with_dp(&s, 0, &mut ans, &mut tmp, &dp);
        ans
    }

    // 使用动态规划
    fn backtrack_with_dp(
        s: &str,
        start: usize,
        ans: &mut Vec<Vec<String>>,
        tmp: &mut Vec<String>,
        dp: &Vec<Vec<bool>>,
    ) {
        // 指针到最后一个字符，意为着找到了一种方案
        if start == s.len() {
            ans.push(tmp.clone());
            return;
        }
        for i in start..s.len() {
            if dp[start][i] {
                tmp.push(s[start..=i].to_string());
                Solution::backtrack_with_dp(s, i + 1, ans, tmp, dp);
                tmp.pop();
            }
        }
    }

    // 使用暴力求解
    pub fn partition_simple(s: String) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut tmp: Vec<String> = vec![];
        Solution::backtrack(&s, 0, &mut ans, &mut tmp);
        ans
    }

    fn backtrack(s: &str, start: usize, ans: &mut Vec<Vec<String>>, tmp: &mut Vec<String>) {
        // 指针到最后一个字符，意为着找到了一种方案
        if start == s.len() {
            ans.push(tmp.clone());
            return;
        }

        for i in start..s.len() {
            if Solution::is_palindrome(s, start, i) {
                tmp.push(s[start..=i].to_string());
            } else {
                continue;
            }
            Solution::backtrack(s, start + 1, ans, tmp);
            tmp.pop();
        }
    }

    fn init_dp(s: &str, dp: &mut Vec<Vec<bool>>) {
        let s = s.as_bytes();
        for i in (0..s.len()).rev() {
            for j in i + 1..s.len() {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
    }

    fn is_palindrome(s: &str, start: usize, end: usize) -> bool {
        let chars = s.as_bytes();
        let mut i = start;
        let mut j = end;
        while i < j {
            if chars[i] != chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
