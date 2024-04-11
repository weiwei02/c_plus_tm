// https://leetcode.cn/problems/find-greatest-common-divisor-of-array/description/
// Find Greatest Common Divisor of Array
// 简单
// 
// 两个数的 最大公约数 是能够被两个数整除的最大正整数。
// 
// 示例 1：
// 输入：nums = [2,5,6,9,10]
// 输出：2
// 解释：
// nums 中最小的数是 2
// nums 中最大的数是 10
// 2 和 10 的最大公约数是 2

// 示例 2：
// 输入：nums = [7,5,6,8,3]
// 输出：1
// 解释：
// nums 中最小的数是 3
// nums 中最大的数是 8
// 3 和 8 的最大公约数是 1

// 示例 3：
// 输入：nums = [3,3]
// 输出：3
// 解释：
// nums 中最小的数是 3
// nums 中最大的数是 3
// 3 和 3 的最大公约数是 3
 

// 提示：
// 2 <= nums.length <= 1000
// 1 <= nums[i] <= 1000

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_gcd() {
        assert_eq!(Solution::find_gcd(vec![2,5,6,9,10]), 2);
        assert_eq!(Solution::find_gcd(vec![7,5,6,8,3]), 1);
        assert_eq!(Solution::find_gcd(vec![3,3]), 3);
    }
}

struct Solution;
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for num in &nums {
            if *num < min {
                min = *num;
            }
            if *num > max {
                max = *num;
            }
        }
        Solution::gcd(min, max)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            return 0;
        } else if a == 1 || b == 1 {
            return 1;
        }

        let mut old = a;
        let mut next = b;
        while 0 != next {
            let tmp = next;
            next = old % next;
            old = tmp;

        }
        old
    }
}