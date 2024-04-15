// https://leetcode.cn/problems/single-number-ii/?envType=study-plan-v2&envId=top-interview-150
// 137. 只出现一次的数字 II 中等
// 
// 给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。
// 你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。

// 示例 1：
// 输入：nums = [2,2,3,2]
// 输出：3
// 示例 2：

// 输入：nums = [0,1,0,1,0,1,99]
// 输出：99
 

// 提示：

// 1 <= nums.length <= 3 * 104
// -2^31 <= nums[i] <= 2^31 - 1
// nums 中，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);

    }
}

struct Solution;
// 思路：只出现一次的数字 II
// 将数字按位拆分，然后统计每一位出现的次数
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut v = vec![0;32];
        for i in nums{
            for j in 0..32{
                v[j]+= 1&(i>>j);
            }
        }
        let mut res = 0;
        for j in 0..32{
            if v[j]%3==1{
                res |= 1<<j;
            }
            
        }
        res
    }
}