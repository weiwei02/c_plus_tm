// https://leetcode.cn/problems/sliding-window-maximum/?envType=study-plan-v2&envId=top-100-liked
// 239 滑动窗口最大值 困难

// 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
// 返回 滑动窗口中的最大值 。
// 示例 1：
// 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
// 输出：[3,3,5,5,6,7]
// 解释：
// 滑动窗口的位置                最大值
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
//  1 [3  -1  -3] 5  3  6  7       3
//  1  3 [-1  -3  5] 3  6  7       5
//  1  3  -1 [-3  5  3] 6  7       5
//  1  3  -1  -3 [5  3  6] 7       6
//  1  3  -1  -3  5 [3  6  7]      7

// 示例 2：
// 输入：nums = [1], k = 1
// 输出：[1]

// 提示：
// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4
// 1 <= k <= nums.length

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1])
    }
}

struct Solution;
impl Solution {
    // 优化方法 2：借助二叉堆来实现优化效果
    // 1. 使用二叉堆来存储当前窗口的最大值
    // 2. 每次移动窗口时，将当前窗口的最大值弹出堆中
    // 3. 将当前窗口的下一个元素加入堆中
    // 4. 尝试对堆中的过期元素进行弹出操作，直到堆中的最大值在窗口中
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..k {
            heap.push((nums[i], i));
        }

        let mut result = Vec::new();
        for i in k..nums.len() {
            if let Some(&(max, index)) = heap.peek() {
                result.push(max);
                heap.push((nums[i], i));
            }
            while let Some(&(_, index)) = heap.peek() {
                if index > i - k {
                    break;
                }
                heap.pop();
            }
        }
        if let Some(&(max, _)) = heap.peek() {
            result.push(max);
        }

        result
    }

    // 优化方法：
    // 1. 使用两个指针，一个指向当前窗口的最大值，另一个指向当前窗口的起始位置
    // 2. 每次移动窗口时，如果当前窗口的最大值小于当前元素，则将当前窗口的最大值更新为当前元素
    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let k: usize = k as usize;

        let first = if nums.len() <= k { nums.len() } else { k };
        let mut max_index: usize = 0;
        for i in 0..first {
            if nums[i] > nums[max_index] {
                max_index = i;
            }
        }
        res.push(nums[max_index]);
        if nums.len() <= first {
            return res;
        }

        for i in first..nums.len() {
            if nums[i] >= nums[max_index] {
                max_index = i;
            }
            if (i - k) >= max_index {
                max_index = i - k + 1;
                for j in max_index + 1..i + 1 {
                    if nums[j] >= nums[max_index] {
                        max_index = j;
                    }
                }
            }
            res.push(nums[max_index]);
        }

        res
    }

    // 暴力解法
    // 该方法 k 较大时，每次都需要重新计算一次 max 值，时间复杂度较高
    pub fn max_sliding_window1(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut deque = std::collections::VecDeque::new();

        for i in 0..nums.len() + 1 {
            if deque.len() == k as usize || i == nums.len() {
                if let Some(&window_max) = deque.iter().max() {
                    res.push(window_max);
                }
                deque.pop_front();
            }
            if i == nums.len() {
                break;
            }
            deque.push_back(nums[i])
        }

        res
    }
}
