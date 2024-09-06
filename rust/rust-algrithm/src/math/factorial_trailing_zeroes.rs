// https://leetcode.cn/problems/factorial-trailing-zeroes
// 阶乘后的零
// 中等
// 给定一个整数 n ，返回 n! 结果中尾随零的数量。
// 提示 n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1

// 示例 1：
// 输入：n = 3
// 输出：0
// 解释：3! = 6 ，不含尾随 0

// 示例 2：
// 输入：n = 5
// 输出：1
// 解释：5! = 120 ，有一个尾随 0

// 示例 3：
// 输入：n = 0
// 输出：0

// 提示：
// 0 <= n <= 104

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(13), 2);
        assert_eq!(Solution::trailing_zeroes(30), 7);
        assert_eq!(Solution::trailing_zeroes(625), 156);
    }
}

struct Solution;
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut zeros = 0;
        let mut result: i64 = 1;

        if n < 2 {
            return zeros;
        }

        // 只需保留个位即可
        for i in 2..=n {
            result *= i as i64;
            while result % 10 == 0 {
                result /= 10;
                zeros += 1;
            } 
            if result > 1000000 {
                result %= 1000000;
            }
        }
        
        zeros
    }

    /*
    这个问题可以通过计算因子 5 的数量来解决。因为 10 是由 2 和 5 相乘得到的，而在 n! 的因子中，2 的数量总是大于或等于 5 的数量，
    所以尾随零的数量就等于因子 5 的数量
    */
    pub fn trailing_zeroes2(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            n /= 5;
            count += n;
        }
        count
    }
}
