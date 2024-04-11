// https://leetcode.cn/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-interview-150
// 42 接雨水 困难
// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
// 示例1：
// 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
// 输出：6
// 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 

// 示例 2：
// 输入：height = [4,2,0,3,2,5]
// 输出：9
 
// 提示：
// n == height.length
// 1 <= n <= 2 * 104
// 0 <= height[i] <= 105

use std::cmp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
        assert_eq!(Solution::trap(vec![4,2,0,3,2,5]), 9);
        assert_eq!(Solution::trap(vec![4,2,3]), 1);
    }
}

// 1. 利用栈保存最高点
// 2. fn = f(n -1) + 
// 2.1 if h(n) <= h(n - 1) fn = f(n -1)
// 2.2 if h(n) > h(n - 1) 不断回溯栈，找到比 h(n - 1) 更大的元素或者最大的元素
// 2.3 如果已经找到了制高点，则不再向左侧回溯
struct Solution;
impl Solution {
    // O(n^2) 解法
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }
        let mut water = height.clone();
        let mut max = height[0].max(height[1]);
        let mut dp = vec![0; height.len()];
        dp[0] = 0;
        dp[1] = 0;
        
        
        for i in 2..height.len() {
            dp[i] = dp[i - 1];
            if height[i] <= height[i - 1] {
                continue;
            }

            // 回溯查找最大值或者大于 h[i] 的值
            let mut j = i - 1;
            let mut temp_water = 0;

            while height[j] <= max && j.ge(&0) {
                // 1. 已经到了最大点，更新最大值
                if height[j] == max {
                    temp_water = height[i].min(max);
                    // 比较替换最大值
                    max = height[i].max(max);
                    break;
                } else if height[j] >= height[i] {
                    // 已经找到了最大水位
                    temp_water = height[i];
                    break;
                }
                j -= 1;
            }
            // if height.len() == 6 {
            //     println!("dp[{}] = {}, max={}, temp_water = {}, water = {:?}", i, dp[i], max, temp_water, water);
            // }
            while j < i {
                if  temp_water - water[j] > 0{
                    dp[i] +=  temp_water - water[j];
                    water[j] = temp_water;
                }
                j += 1;
            }
            
        }
        dp[dp.len() - 1]
    }

    // O(n) 解法
    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut pointer = (0, height.len() - 1);
        let mut max = (0, 0);
        
        let mut water = 0;
        while pointer.0 < pointer.1 {
            let l_height = height[pointer.0];
            let r_height = height[pointer.1];

            if l_height < r_height {
                max.0 = cmp::max(max.0, l_height);
                water += max.0 - l_height;

                pointer.0 += 1;
            } else {
                max.1 = cmp::max(max.1, r_height);
                water += max.1 - r_height;
                pointer.1 -= 1;
            }
        }
        return water;
    }
}