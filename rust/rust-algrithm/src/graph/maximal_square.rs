// https://leetcode.cn/problems/maximal-square/?envType=study-plan-v2&envId=top-interview-150
// 最大正方形
// 中等
// 在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。

// 示例 1：
// 输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
// 输出：4

// 示例 2：
// 输入：matrix = [["0","1"],["1","0"]]
// 输出：1

// 示例 3：
// 输入：matrix = [["0"]]
// 输出：0

// 提示：
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 300
// matrix[i][j] 为 '0' 或 '1'


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]),
            1
        );
        assert_eq!(Solution::maximal_square(vec![vec!['0']]), 0);
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 正方形组成的方法是：一个元素
    // 使用动态规划，dp[i][j]表示以i,j为右下角的最大正方形的面积
    // if matrix[i][j] == '1' then dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1
    // else dp[i][j] = 0
    // 返回 max(dp[i][j])
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        let mut max_dp = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
                        dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j - 1]) + 1;
                    }
                    if dp[i][j] > max_dp {
                        max_dp = dp[i][j];
                    }
                }
            }
        }

        //    for i in 0..m {
        //     println!("{:?}", dp[i]);
        //    }

        max_dp * max_dp
    }
}
