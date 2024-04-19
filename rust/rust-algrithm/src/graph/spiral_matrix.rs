// https://leetcode.cn/problems/spiral-matrix/description/?envType=study-plan-v2&envId=top-interview-150
// 中等
// 给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。

// 示例 1：
// 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
// 输出：[1,2,3,6,9,8,7,4,5]

// 示例 2：
// 输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
// 输出：[1,2,3,4,8,12,11,10,9,5,6,7]

// 提示：
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 10
// -100 <= matrix[i][j] <= 100

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ],),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
                vec![17, 18, 19, 20],
            ],),
            vec![1, 2, 3, 4, 8, 12, 16, 20, 19, 18, 17, 13, 9, 5, 6, 7, 11, 15, 14, 10]
        );

        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25]
            ],),
            vec![
                1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17,
                12, 13
            ]
        );
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5],],),
            vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1]
        );

        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2,], vec![8, 3,], vec![7, 4,], vec![6, 5,],],),
            vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
}

struct Solution;
impl Solution {
    // 顺时针螺旋顺序 更易理解的解法
    // 核心是使用标记法，标记已经访问过的元素
    // 比较下面的算法，优势是使用了两个指针，难度降低一倍
    // 每一步都定义了下一步该怎么走
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len() as usize, matrix[0].len() as usize);
        let mut visited = vec![vec![false; n]; m];
        let (mut i, mut j) = (0, 0);
        let mut direction = Direction::Right;
        let mut ans = Vec::new();
        loop {
            visited[i][j] = true;
            ans.push(matrix[i][j]);
            match direction {
                Direction::Up if i == 0 || visited[i - 1][j] => {
                    direction = Direction::Right;
                    j += 1;
                }
                Direction::Up => {
                    i -= 1;
                }
                Direction::Down if i == m - 1 || visited[i + 1][j] => {
                    direction = Direction::Left;
                    j -= 1;
                }
                Direction::Down => {
                    i += 1;
                }
                Direction::Left if j == 0 || visited[i][j - 1] => {
                    direction = Direction::Up;
                    i -= 1;
                }
                Direction::Left => {
                    j -= 1;
                }
                Direction::Right if j == n - 1 || visited[i][j + 1] => {
                    direction = Direction::Down;
                    i += 1;
                }
                Direction::Right => {
                    j += 1;
                }
            }
            if i > m - 1 || j > n - 1 || visited[i][j] {
                break;
            }
        }
        ans
    }
    // 顺时针螺旋顺序
    pub fn spiral_order2(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let (mut max_right, mut max_down, mut max_left, mut max_up) =
            (matrix[0].len(), matrix.len(), 0, 0);
        let size = matrix[0].len() * matrix.len();

        loop {
            if max_up >= max_down && max_left >= max_right {
                break;
            }
            if res.len() == size {
                break;
            }
            if max_left < max_right {
                // 向右
                let mut left = max_left;

                while left < max_right {
                    res.push(matrix[max_up][left]);
                    left += 1;
                }
            }
            if max_up < max_down {
                max_up += 1;
            }
            if res.len() == size {
                break;
            }
            // println!("{:?}", res);

            if max_up < max_down {
                // 向下
                let mut down = max_up;
                while down < max_down {
                    res.push(matrix[down][max_right - 1]);
                    down += 1;
                }
            }
            if max_right > 0 {
                max_right -= 1;
            }
            if res.len() == size {
                break;
            }
            // println!("{:?}", res);

            // 向左
            if max_left < max_right {
                let mut left = (max_right - 1) as i32;
                while left >= max_left as i32 {
                    res.push(matrix[max_down - 1][left as usize]);
                    left -= 1;
                }
            }
            if max_down > 0 {
                max_down -= 1;
            }
            if res.len() == size {
                break;
            }
            // println!("{:?}", res);
            if max_up < max_down {
                // 向上
                let mut up = (max_down - 1) as i32;
                while up >= max_up as i32 {
                    res.push(matrix[up as usize][max_left]);
                    up -= 1;
                }
            }
            if max_left < max_right {
                max_left += 1;
            }
            // println!("{:?}", res);
        }

        res
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
