// https://leetcode.cn/problems/number-of-islands/description/?envType=study-plan-v2&envId=top-interview-150
// 200. 岛屿数量
// 中等
// 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
// 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
// 此外，你可以假设该网格的四条边均被水包围。

// 示例 1：
// 输入：grid = [
//   ["1","1","1","1","0"],
//   ["1","1","0","1","0"],
//   ["1","1","0","0","0"],
//   ["0","0","0","0","0"]
// ]
// 输出：1

// 示例 2：
// 输入：grid = [
//   ["1","1","0","0","0"],
//   ["1","1","0","0","0"],
//   ["0","0","1","0","0"],
//   ["0","0","0","1","1"]
// ]
// 输出：3

// 提示：
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] 的值为 '0' 或 '1'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1'],
            ]),
            1
       );
       assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1' , '1'],
                vec!['0', '1' , '0'],
                vec!['1','1' , '1']
            ]),
            1
       )
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 1. 遍历二维数组，如果遇到1，则将位置标记为0，并把上下左右位置为1的都加入到队列中，并将对应的值置为0
    // 2. 队列元素遍历，如果遇到1，则将位置标记为0，并把上下左右位置为1的都加入到队列中，并将对应的值置为0
    // 3. 遍历结束后，总岛屿数量+1
    // 4. 继续移动矩阵指针，寻找下一个岛屿，重复1-4
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = std::collections::VecDeque::new();
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    // 开始广度优先搜索
                    count += 1;
                    grid[i][j] = '0';
                    queue.push_back((i, j));
                    while !queue.is_empty() {
                        let (x, y) = queue.pop_front().unwrap();
                        if x > 0 && grid[x - 1][y] == '1' {
                            grid[x - 1][y] = '0';
                            queue.push_back((x - 1, y));
                        }
                        if x < m - 1 && grid[x + 1][y] == '1' {
                            grid[x + 1][y] = '0';
                            queue.push_back((x + 1, y));
                        }
                        if y > 0 && grid[x][y - 1] == '1' {
                            grid[x][y - 1] = '0';
                            queue.push_back((x, y - 1));
                        }
                        if y < n - 1 && grid[x][y + 1] == '1' {
                            grid[x][y + 1] = '0';
                            queue.push_back((x, y + 1));
                        }
                    }
                }
            }
        }
        count
    }
}
