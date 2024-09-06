// https://leetcode.cn/problems/set-matrix-zeroes
// 矩阵置零
// 中等
// 给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。请使用 原地 算法。

// 示例 1：
// 输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
// 输出：[[1,0,1],[0,0,0],[1,0,1]]

// 示例 2：
// 输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// 输出：[[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 

// 提示：
// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -231 <= matrix[i][j] <= 231 - 1

struct Solution;
impl Solution {
    // 思路使用标记数组记录需要置零的行列位置
    // 然后遍历矩阵，如果当前位置需要置零，则将对应行列的元素置为0
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut marks:Vec<Vec<usize>> = vec![];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    marks.push(vec![i, j]);
                }
            }
        }

        for mark in marks {
            for i in 0..matrix.len() {
                if i != mark[0] {
                    matrix[i][mark[1]] = 0;
                }
            }
            for j in 0..matrix[mark[0]].len() {
                if j != mark[1] {
                    matrix[mark[0]][j] = 0;
                }
            }
        }
    }
}