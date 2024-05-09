// https://leetcode.cn/problems/minimum-size-subarray-sum/description/
// 209. 长度最小的子数组
// 中等
// 给定一个含有 n 个正整数的数组和一个正整数 target 。
// 找出该数组中满足其总和大于等于 target 的长度最小的 连续
// 子数组
//  [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。

// 示例 1：
// 输入：target = 7, nums = [2,3,1,2,4,3]
// 输出：2
// 解释：子数组 [4,3] 是该条件下的长度最小的子数组。

// 示例 2：
// 输入：target = 4, nums = [1,4,4]
// 输出：1

// 示例 3：
// 输入：target = 11, nums = [1,1,1,1,1,1,1,1]
// 输出：0
 

// 提示：
// 1 <= target <= 10^9
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5

// 进阶：
// 如果你已经实现 O(n) 时间复杂度的解法, 请尝试设计一个 O(n log(n)) 时间复杂度的解法。

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 解法1：队列，时间复杂度O(n), 空间复杂度O(n)
    // 1. 申请一个辅助队列 elements， 并记录当前队列中的总元素大小为 sum
    // 2. 遍历 nums， 将 nums 中的元素依次加入 elements， 并记录当前队列中的总元素大小为 sum
    // 3. 如果当前元素之和 sum >= target， 则记录当前元素个数count
    // 4. 将队列中的元素出队， 并记录当前队列中的总元素大小为 sum，如果 sum > target，则比较并替换 count
    // 5. 如果 sum < target， 则继续遍历，入队下一个元素
    // 6. 重复2-5， 直到遍历完所有元素
    // 7. 返回 count
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                count = if count == 0 {
                    (right - left + 1) as i32
                } else {
                    count.min((right - left + 1) as i32)
                };
                sum -= nums[left];
                left += 1;
            }
        }
        count
    }
}