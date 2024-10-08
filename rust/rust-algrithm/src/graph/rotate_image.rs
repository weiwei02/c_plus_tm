// https://leetcode.cn/problems/rotate-image
// 48. 旋转图像
// 中等
// 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。

// 你必须在 `原地` 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。

// 示例 1：
// 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
// 输出：[[7,4,1],[8,5,2],[9,6,3]]

// 示例 2：
// 输入：matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
// 输出：[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

use std::borrow::BorrowMut;
 

// 提示：
// n == matrix.length == matrix[i].length
// 1 <= n <= 20
// -1000 <= matrix[i][j] <= 1000
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let mut ve = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut ve);
        assert_eq!(ve,
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
        );
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 1. 先水平翻转，再对角线翻转
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}