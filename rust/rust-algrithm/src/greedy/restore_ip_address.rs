// https://leetcode.cn/problems/restore-ip-addresses/
//
// 93. 复原 IP 地址 中等

// 有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。
// 例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，
// 但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
// 给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。

// 示例 1：
// 输入：s = "25525511135"
// 输出：["255.255.11.135","255.255.111.35"]
// 示例 2：

// 输入：s = "0000"
// 输出：["0.0.0.0"]

// 示例 3：
// 输入：s = "101023"
// 输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]

// 提示：
// 1 <= s.length <= 20
// s 仅由数字组成

#[cfg(test)]
mod tests {
    use super::*;
    fn restore_ip_addresses_to_test(s: &str) -> Vec<String> {
        Solution::restore_ip_addresses(s.to_string())
    }

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            restore_ip_addresses_to_test("25525511135"),
            vec![
                String::from("255.255.11.135"),
                String::from("255.255.111.35"),
            ]
        );
        assert_eq!(
            restore_ip_addresses_to_test("0000"),
            vec![String::from("0.0.0.0")]);
        assert_eq!(
            restore_ip_addresses_to_test("101023"),
            vec![
                String::from("1.0.10.23"),
                String::from("1.0.102.3"),
                String::from("10.1.0.23"),
                String::from("10.10.2.3"),
                String::from("101.0.2.3"),
                ]);
    }
}
struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = Vec::new();

        let chars = s.as_str();
        let l = chars.len();
        if l < 4 {
            return res;
        }

        for i in 0..3.min(l - 3) {
            let a = str_to_ip_e(&chars[0..i + 1]);
            if a == -1 {
                continue;
            }
            if (l - i) < 4 || (l - i) > 10 {
                continue;
            }

            for j in (i + 1)..(i + 4).min(l - 2) {
                if (l - j) < 3 || (l - j) > 7 {
                    continue;
                }
                let b = str_to_ip_e(&chars[i + 1..j + 1]);
                if b == -1 {
                    continue;
                }

                for k in (j + 1)..(j + 4).min(l - 1) {
                    if (l - k) < 2 || (l - k) > 4 {
                        continue;
                    }
                    let c = str_to_ip_e(&chars[j + 1..k + 1]);
                    if c == -1 {
                        continue;
                    }
                    let d = str_to_ip_e(&chars[k + 1..l]);
                    if d == -1 {
                        continue;
                    }
                    res.push(format!("{}.{}.{}.{}", a, b, c, d));
                }
            }
        }
        res
    }
}

fn str_to_ip_e(s: &str) -> i32 {
    if s.starts_with("0") && s.len() > 1 {
        return -1;
    }
    let i = s.parse::<i32>();
    if i.is_err() {
        return -1;
    }
    let i = i.unwrap();
    if i < 0 || i > 255 {
        return -1;
    }
    i
}
