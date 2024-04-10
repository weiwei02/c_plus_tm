// 给定一个字符串(s)和一个字符模式(p)。实现支持 '.' 和 '*' 的正则表达式匹配。
// '.' 匹配任意单个字符
// 星号'*'匹配零个或多个前面的元素
// 匹配应该覆盖整个字符串(s)的起始和结束位置。
// 说明：
// 1.s 可能为空，且只包含从 a-z 的小写字母。
// 2.p 可能为空，且只包含从 a-z 的小写字母以及字符 '.' 和 '*'。
// 
// 示例1:
// 输入:
// s = "aa"
// p = "a"
// 输出: false
// 解释: "a" 无法匹配 "aa" 整个字符串。
// 
// 示例2:
// 输入:
// s = "aa"
// p = "a*"
// 输出: true
// 解释: '*' 代表可以匹配零个或多个前面的元素, 即这里匹配 'a'。因此重复'a'一次, 字符串 "aa" 可匹配。
// 
// 示例3:
// 输入:
// s = "ab"
// p = ".*"
// 输出: true
// 解释: ".*" 表示能匹配零个或多个('*')任意字符('.').
// 
// 示例4:
// 输入:
// s = "aab"
// p = "c*a*b"
// 输出: true
// 解释: c 可以不被重复, a 可以被重复一次。因此匹配字符串 "aab"。
// 
// 示例5:
// 输入:
// s = "mississippi"
// p = "mis*is*p*."
// 输出: false
// 
// 原文链接：https://leetcode-cn.com/problems/regular-expression-matching
// 解析: https://github.com/DavidNiSilence/ZXBlog/blob/master/%E5%88%B7%E9%A2%98/LeetCode/DP/LeetCode%20-%2010%20-%20Regular%20Expression%20Matching.md

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_re_match() {
        assert_eq!(re_match("aa", "a"), false);
        assert_eq!(re_match("aa", "a*"), true);
        assert_eq!(re_match("ab", ".*"), true);
        assert_eq!(re_match("aab", "c*a*b"), true);
        assert_eq!(re_match("mississippi", "mis*is*p*."), false);
    }

    #[test]
    fn test_re_match_res() {
        assert_eq!(re_match_res("aa", "a"), false);
        assert_eq!(re_match_res("aa", "a*"), true);
        assert_eq!(re_match_res("ab", ".*"), true);
        assert_eq!(re_match_res("aab", "c*a*b"), true);
        assert_eq!(re_match_res("mississippi", "mis*is*p*."), false);
    }
}

// re_match_res 使用递归方式完成正则表达式动态规划，时间复杂度 O(n^2)
fn re_match_res(s: &str, p: &str) -> bool {
    match_rescu(s.as_bytes(), p.as_bytes(), &mut vec![vec![false; p.len() + 1]; s.len() + 1])
}

fn match_rescu(s: &[u8], p: &[u8], dp: &mut Vec<Vec<bool>>) -> bool {
    let mut res = false;

    let ls = s.len();
    let lp = p.len();

    if p[lp - 1] == b'.' || p[lp - 1] == s[ls - 1]  {
        res = match_rescu(&s[0..ls - 1], &p[0..lp -1 ], dp)
    }

    // 分三种情况处理 *
    if p[lp - 1] == b'*' {
        if p[lp - 2] == b'.' || p[lp - 2] == s[ls - 1] {
            // 当 * 前的字符是 . 时，匹配任意字符，有三种情况
            res = match_rescu(&s[0..ls - 1], &p[0..lp -1 ], dp) ||
            // 
                match_rescu(&s[0..ls - 1], p, dp) ||
                // 直接忽略 .*
                match_rescu(s, &p[0..lp - 2], dp);
        } else if s[ls -1 ] == p[lp - 2] {
            res = match_rescu(&s[0..ls - 1], &p[0..lp - 2], dp) ||
            match_rescu(&s[0..ls - 1], p, dp) ||
            match_rescu(s, &p[0..lp - 2], dp);
        } else {
            res = match_rescu(s, &p[0..lp - 2], dp);
        }
    }

    dp[ls][lp] = res;
    res
}


// 使用递推方法
fn re_match(s: &str, p: &str) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();

    let m = s.len();
    let n = p.len();

    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for i in 1..=n {
        if p[i - 1] == b'*' {
            dp[0][i] = dp[0][i - 2];
        }
    }

    let mut i = 1;

    let print_dp = | dp: &Vec<Vec<bool>>| {
        if p.eq("a*".as_bytes()) {
            println!("dp is:");
            for i in 0..m + 1 {
                println!("{:?}", dp[i])
            }
        }
    };
    while i <= m {
        let mut j = 1;
        while j <= n {
            if s[i - 1] == p[j - 1] || p[j - 1] == b'.' {
                dp[i][j] = dp[i - 1][j - 1];
            }else if p[j - 1] == b'*' {
                if p[j - 2] == b'.' {
                    dp[i][j] = dp[i - 1][j - 1] || dp[i - 1][j] || dp[i][j - 2];
                } else if s[i - 1] == p[j - 2] {
                    dp[i][j] = dp[i - 1][j - 2] || dp[i - 1][j] || dp[i][j - 2];
                } else {
                    dp[i][j] = dp[i][j - 2];
                }
            }
            j += 1;
        }
        i += 1;
    }
    print_dp(&dp);
    dp[m][n]
}