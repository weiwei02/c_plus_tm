// https://leetcode.cn/problems/unique-paths-ii/?envType=study-plan-v2&envId=top-interview-150
// 63. 不同路径 II 中等
// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish”）。
// 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
// 网格中的障碍物和空位置分别用 1 和 0 来表示。

// 示例 1：
// 输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
// 输出：2
// 解释：3x3 网格的正中间有一个障碍物。
// 从左上角到右下角一共有 2 条不同的路径：
// 1. 向右 -> 向右 -> 向下 -> 向下
// 2. 向下 -> 向下 -> 向右 -> 向右

// 示例 2：
// 输入：obstacleGrid = [[0,1],[0,0]]
// 输出：1

// 提示：
// m == obstacleGrid.length
// n == obstacleGrid[i].length
// 1 <= m, n <= 100
// obstacleGrid[i][j] 为 0 或 1

mod tests {
    use super::Solution;
    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 2);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);
    }
}

struct Solution;
impl Solution {
    // 思路：动态规划
    // 由于存在障碍物，所以只能从左上角出发，向右下角走，所以只能向下或者向右走
    // 那么存在障碍物的位置，无法到达，所以到达该位置的路径数为0
    // 状态转移方程：
    // dp[i][j] = dp[i-1][j] + dp[i][j-1]
    // 边界条件：
    // dp[0][0] = 1
    // 如果 obstacle_grid[i][j] = 1，则dp[i][j] = 0
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        dp[0][0] = 1;
        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[0].len() {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                    continue;
                }
                if i > 0  {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
        dp[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1]
    }
}