// https://leetcode.cn/problems/product-of-array-except-self
// 除自身以外数组的乘积
// 中等
// 给你一个整数数组 nums，返回 数组 `answer` ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积 。
// 题目数据 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内。

// 请 `不要使用除法`，且在 O(n) 时间复杂度内完成此题。
// 示例 1:
// 输入: nums = [1,2,3,4]
// 输出: [24,12,8,6]

// 示例 2:
// 输入: nums = [-1,1,0,-3,3]
// 输出: [0,0,9,0,0]
 
// 提示：
// 2 <= nums.length <= 10^5
// -30 <= nums[i] <= 30
// 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内
 

// 进阶：你可以在 O(1) 的额外空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组 不被视为 额外空间。）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
            );
    }
}

struct Solution;
impl Solution {
    // 不使用除法，且在 O(n) 时间复杂度内完成此题。
    // 如果使用除法，那么时间复杂度为 O(nlogn)
    // 如果使用暴力法，时间复杂度为 O(n^2)
    // 如果使用前缀和，时间复杂度为 O(n)
    // 如果使用后缀和，时间复杂度为 O(n)
    // 空间复杂度为 O(1)
    // 前缀和后缀和的思路
    // 1. 定义一个数组，长度为 nums 的长度
    // 2. 遍历 nums，计算前缀和
    // 3. 遍历 nums，计算后缀和
    // 4. 返回结果
    // 前缀和：nums[0] * nums[1] * ... * nums[i - 2] * nums[i - 1]
    // 后缀和：nums[i + 1] * nums[i + 2] * ... * nums[n - 2] * nums[n - 1]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result: Vec<i32> = vec![0; n];
        result[0] = 1;
        for i in 1..n {
            result[i] = result[i - 1] * nums[i - 1];
        }
        let mut r = 1;
        for i in (0..n).rev() {
            result[i] *= r;
            r *= nums[i];
        }
        result
    }
}