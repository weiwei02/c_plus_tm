// https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array
// 搜索旋转排序数组的最小值
// 已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到输入数组。例如，原数组 nums = [0,1,2,4,5,6,7] 在变化后可能得到：
// 若旋转 4 次，则可以得到 [4,5,6,7,0,1,2]
// 若旋转 7 次，则可以得到 [0,1,2,4,5,6,7]
// 注意，数组 [a[0], a[1], a[2], ..., a[n-1]] 旋转一次 的结果为数组 [a[n-1], a[0], a[1], a[2], ..., a[n-2]] 。

// 给你一个元素值 互不相同 的数组 nums ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 最小元素 。
// 你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。

// 示例 1：
// 输入：nums = [3,4,5,1,2]
// 输出：1
// 解释：原数组为 [1,2,3,4,5] ，旋转 3 次得到输入数组。

// 示例 2：
// 输入：nums = [4,5,6,7,0,1,2]
// 输出：0
// 解释：原数组为 [0,1,2,4,5,6,7] ，旋转 3 次得到输入数组。

// 示例 3：
// 输入：nums = [11,13,15,17]
// 输出：11
// 解释：原数组为 [11,13,15,17] ，旋转 4 次得到输入数组。

// 提示：
// n == nums.length
// 1 <= n <= 5000
// -5000 <= nums[i] <= 5000
// nums 中的所有整数 互不相同
// nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![1]), 1);
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
        assert_eq!(Solution::find_min(vec![1, 2]), 1);
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(Solution::find_min(vec![1, 2, 3, 4, 5, 6, 7, 0]), 0);
    }
}

struct Solution;
impl Solution {
    // 思路：二分查找
    // 时间复杂度：O(logn)
    // 设要start,end 两个下标，start = 0, end = nums.len() - 1
    // 如果nums[start] < nums[end]，则最小值在start和end之间，否则在start和end的右边
    // 每次取start和end的中间值mid，如果nums[mid] >= nums[start]，则最小值在start和mid之间
    // 如果nums[mid] < nums[start]，则最小值在mid和end之间
    // 重复上述操作，直到start和end相等为止
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);
        while start < end {
            let mid = (start + end) / 2;
            if nums[mid] > nums[start] {
                if nums[mid] > nums[end] {
                    start = mid + 1;
                } else {
                    end = mid;
                }
            } else if nums[mid] == nums[start] {
                if nums[mid + 1] < nums[start] {
                    start = mid + 1;
                } else {
                    end = mid;
                }
            } else {
                end = mid;
            }
        }
        // start == end
        nums[start]
    }

    // 更简化的写法
    pub fn find_min_simple(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[right]
    }
}
