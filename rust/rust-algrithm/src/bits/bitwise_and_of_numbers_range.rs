// https://leetcode.cn/problems/bitwise-and-of-numbers-range
// 数字范围按位与
// 中等
// 给你两个整数 left 和 right ，表示区间 [left, right] ，返回此区间内所有数字 按位与 的结果（包含 left 、right 端点）。

// 示例 1：
// 输入：left = 5, right = 7
// 输出：4

// 示例 2：
// 输入：left = 0, right = 0
// 输出：0

// 示例 3：
// 输入：left = 1, right = 2147483647
// 输出：0

// 提示：
// 0 <= left <= right <= 231 - 1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
        assert_eq!(Solution::range_bitwise_and(9, 11), 8);
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }
}
pub struct Solution {}
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let dis = right - left;

        let mut ans = 0;
        for i in 0..31 {
            let m = 1 << i;
            ans |= left & right & m & if dis < m { !0 } else { 0 }
        }

        ans
    }
}
