// https://leetcode.cn/problems/minimum-path-sum
// 最小路径和
// 中等
// 给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

// 说明：每次只能向下或者向右移动一步。

 

// 示例 1：
// 输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
// 输出：7
// 解释：因为路径 1→3→1→1→1 的总和最小。

// 示例 2：
// 输入：grid = [[1,2,3],[4,5,6]]
// 输出：12
 

// 提示：
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 200
// 0 <= grid[i][j] <= 200

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(Solution::min_path_sum(
            vec![
                vec![1, 3, 1], 
                vec![1, 5, 1],
                vec![4, 2, 1],
            ]), 7);
        assert_eq!(Solution::min_path_sum(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ],
        ), 12);
    }
}

struct Solution;
impl Solution {
    // 求最小路径和思路
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        dp[0][0] = grid[0][0];
        for i in 1..grid.len() {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        for j in 1..grid[0].len() {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }
        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
            }
        }
        dp[grid.len() - 1][grid[0].len() - 1]
    }
}