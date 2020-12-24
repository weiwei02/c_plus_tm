package dynamic

import "math"

/**
 * 二维动态规划问题
 *
 * 平面上有N*M个格子，每个格子中都有一定数量的苹果，每一步只能向下走或者向右走，
 * 每走到一个格子就把一个格子上的苹果收集起来，这样下去，你最多能收集到多少苹果
 */

func two_dismensional(lattices [][]int) int {
	rows := len(lattices)
	columns := len(lattices[0])
	var dp = make([][]int, rows)
	for i := range dp {
		dp[i] = make([]int, columns)
	}
	// 重复子问题
	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			if i == 0 && j == 0 {
				dp[i][j] = lattices[i][j]
			} else if i == 0 {
				dp[i][j] = lattices[i][j] + dp[i][j-1]
			} else if j == 0 {
				dp[i][j] = lattices[i][j] + dp[i-1][j]
			} else {
				// 最优子结构: dp 的最优解是从左或从左上过来的路径
				dp[i][j] = lattices[i][j] + max(dp[i-1][j], dp[i][j-1])
			}
		}
	}
	return dp[rows - 1][columns - 1]
}

func max(a, b int) int {
	return int(math.Max(float64(a), float64(b)))
}
