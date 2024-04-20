// https://leetcode.cn/problems/3sum/description/?envType=study-plan-v2&envId=top-interview-150
// 15. 三数之和  中等
// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 
// 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请
// 你返回所有和为 0 且不重复的三元组。
// 注意：答案中不可以包含重复的三元组。

// 示例 1：

// 输入：nums = [-1,0,1,2,-1,-4]
// 输出：[[-1,-1,2],[-1,0,1]]
// 解释：
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
// 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
// 注意，输出的顺序和三元组的顺序并不重要。
// 示例 2：

// 输入：nums = [0,1,1]
// 输出：[]
// 解释：唯一可能的三元组和不为 0 。
// 示例 3：

// 输入：nums = [0,0,0]
// 输出：[[0,0,0]]
// 解释：唯一可能的三元组和为 0 。
 

// 提示：

use std::collections::VecDeque;

// 3 <= nums.length <= 3000
// -10^5 <= nums[i] <= 10^5
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1,-1,2],vec![-1, 0, 1]]);
        
    }
}

struct Solution;
impl Solution {
    // 暴力解法：
    // 三个指针：
    // 第一个指针：i 从 0 到 nums.len() - 2
    // 第二个指针：j 从 i + 1 到 nums.len() - 1
    // 第三个指针：k 从 nums.len - 1 到 j + 1
    // 遍历数组，将三个指针分别指向不同的数，然后判断三个数的和是否等于0，如果等于0，则将三个数加入结果数组中，然后继续遍历下一个数，直到遍历完数组。
    // 时间复杂度：O(n^2) 因为 j + k 的取值范围才是一个完整的 n 的数组，所以时间复杂度是 O(n^2)。
    // 空间复杂度：O(n)
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut set = vec![];
        nums.sort();
        for i in 0..nums.len() - 2 {
            // 使用过一次的数字，不用再做尝试了
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let target = -nums[i];
            let mut k = nums.len() -1;
            for j in i + 1..nums.len() - 1 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                while j < k && nums[j] + nums[k] > target {
                    k -= 1;
                }

                // 没有找到合适的数
                if j == k {
                    break;
                }

                if nums[j] + nums[k] == target {
                    set.push(vec![nums[i], nums[j], nums[k]]);
                }
            }
        }
        
        set
    }
}