// https://leetcode.cn/problems/longest-consecutive-sequence/?envType=study-plan-v2&envId=top-interview-150
// 128. 最长连续序列
// 给你一个未排序的整数数组 nums ，（不要求序列元素在原数组中连续）并返回该序列的长度。
// 请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
// 连续递增可以由等差数列表示，其中相邻元素之间的差值==1 。
// 示例 1：
// 输入：nums = [100,4,200,1,3,2]
// 输出：4
// 解释：最长连续递增序列是 [1,2,3,4] 。它的长度为 4 。
// 示例 2：
// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
// 输出：9
// 提示：0 <= nums.length <= 10^5
// -109 <= nums[i] <= 10^9

use std::collections::HashMap;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);

    }
}

struct Solution;
impl Solution {
    // 思路1：使用hash做排序.然后借助hash的key-value对，可以快速找到连续序列的长度
    // 1. 将数据全部存取到hash中，key为数据
    // 2. 遍历hash，找到key-value对中，key+1和key-1的key是否存在

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_hash:HashMap<i32, i32> = HashMap::from_iter(nums.iter().map(|&x| (x, 1)));
        let mut max_len = 0;

        for i in num_hash.keys() {
            if num_hash.contains_key(&(i-1)) {
                continue;
            }
            let mut j = *i;
            let mut count = 1;
            while num_hash.contains_key(&(j+1)) {
                j += 1;
                count += 1;
            }
            max_len = max_len.max(count);

        }
        max_len
    }
}