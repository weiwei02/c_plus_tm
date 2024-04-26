// Minimum Window Substring
// 76 最小覆盖子串 困难
// 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。
// 如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。
// 注意：
// 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
// 如果 s 中存在这样的子串，我们保证它是唯一的答案。

// 示例 1：
// 输入：s = "ADOBECODEBANC", t = "ABC"
// 输出："BANC"
// 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。

// 示例 2：
// 输入：s = "a", t = "a"
// 输出："a"
// 解释：整个字符串 s 是最小覆盖子串。

// 示例 3:
// 输入: s = "a", t = "aa"
// 输出: ""
// 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
// 因此没有符合条件的子字符串，返回空字符串。

// 提示：

// m == s.length
// n == t.length
// 1 <= m, n <= 10^5
// s 和 t 由英文字母组成

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(min_window_for_test("a", "aa"), "".to_string());
        assert_eq!(
            min_window_for_test("ADOBECODEBANC", "ABC"),
            "BANC".to_string()
        );
        assert_eq!(min_window_for_test("a", "a"), "a".to_string());
    }

    fn min_window_for_test(s: &str, t: &str) -> String {
        Solution::min_window(s.to_string(), t.to_string())
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 为 t 中的每个字符建立hash表，一个记录s中出现的位置index_map，一个记录t中每个字符出现的次数count_map
    // 通过这个hash表已经足以判断是否满足t是s覆盖字串的条件了
    // 接下来找到最小的覆盖字串
    // 贪心法：
    // 1. 从count_map中取出最小的count，作为聚集点，
    // 1.1 注意由于count_map所有的值都可能大于1，所以聚集点可能有多个坐标。我们最少要记录 min_init，和max_init两个点
    // 2. 设置 start 和 end 两个指针，分别指向聚集点左边和聚集点右边的边界
    // 3. 遍历 count_map，得到count，并凑够index_map中取出第count个距离聚集点最近的值。
    // 4. 求最近的方法，如果点小于start，则将点更新为新的start。如果点大于end，则将点更新为新的end。
    // 5. 循环结束后，返回start和end之间的子串。即为要求的子串。
    // 时间复杂度：O(m+n)
    // 最小字符串的长度一定大于等于 t 的长度
    pub fn min_window(s: String, t: String) -> String {
        let mut count_map = std::collections::HashMap::new();
        for c in t.as_bytes() {
            *count_map.entry(c).or_insert(0) += 1;
        }

        let sbytes = s.as_bytes();
        let mut scount_map = std::collections::HashMap::new();

        let mut not_found_char_count = count_map.len();
        let mut start = 0;
        let mut end = 0;
        let (mut min_start, mut min_end) = (0, sbytes.len());

        while end < sbytes.len() {
            let c = sbytes[end];
            if count_map.contains_key(&c) {
                if let Some(&count) = count_map.get(&c) {
                    let scount = scount_map.entry(c).or_insert(0);
                    *scount += 1;
                    if *scount == count {
                        not_found_char_count -= 1;
                    }
                }
            }
            if not_found_char_count == 0 && end - start < min_end - min_start {
                min_start = start;
                min_end = end;
            }
            while not_found_char_count == 0 && start <= end {
                if end - start < min_end - min_start {
                    min_start = start;
                    min_end = end;
                }

                let c = sbytes[start];
                if count_map.contains_key(&c) {
                    if let Some(&count) = count_map.get(&c) {
                        let scount = scount_map.entry(c).or_insert(0);
                        *scount -= 1;
                        if *scount == count - 1 {
                            not_found_char_count += 1;
                        }
                    }
                }
                start += 1;
            }
            end += 1;
        }

        if min_end < sbytes.len() {
            return sbytes[min_start..=min_end]
                .iter()
                .map(|&x| x as char)
                .collect();
        }
        "".to_string()
    }
}
