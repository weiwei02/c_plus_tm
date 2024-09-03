// https://leetcode.cn/problems/permutations
// 全排列
// 中等
// 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

 
// 示例 1：
// 输入：nums = [1,2,3]
// 输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

// 示例 2：
// 输入：nums = [0,1]
// 输出：[[0,1],[1,0]]

// 示例 3：
// 输入：nums = [1]
// 输出：[[1]]
 

// 提示：
// 1 <= nums.length <= 6
// -10 <= nums[i] <= 10
// nums 中的所有整数 互不相同
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_total_n_queens() {
        assert_eq!(Solution::permute(vec![1,2,3]), 
        vec![
            vec![1,2,3],
            vec![1,3,2],
            vec![2,1,3],
            vec![2,3,1],
            vec![3,2,1],
            vec![3,1,2]
        ]);
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 全排列回溯算法：
    // 1. 设 i 是当前位置，nums[i..] 是一个长度为 n-i 的数组，即剩余待排列的元素。
    // 2. 枚举 nums[i..] 中所有元素 x，将其与 nums[i] 交换位置，得到新的排列 permute(nums, i+1)。
    // 3. 回溯时恢复现场，即将 nums[i] 与 nums[j] 交换位置。
    // 因为每个数的位置互不相同，所以不需要考虑重复的排列。不需要剪枝
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums.clone();
        Self::backtrack(&mut nums, 0, &mut res);
        res
    }

    fn backtrack(nums: &mut Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            res.push(nums.clone());
        } else {
            for j in i..nums.len() {
                nums.swap(i, j);
                Self::backtrack(nums, i + 1, res);
                nums.swap(i, j);
            }
        }
    }
}