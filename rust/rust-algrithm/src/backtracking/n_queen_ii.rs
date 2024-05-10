// https://leetcode.cn/problems/n-queens-ii/description/?envType=study-plan-v2&envId=top-interview-150
// 52. N 皇后 II
// 困难
// n 皇后问题 研究的是如何将 n 个皇后放置在 n × n 的棋盘上，并且使皇后彼此之间不能相互攻击。
// 给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。

// 示例 1：
// 输入：n = 4
// 输出：2
// 解释：如上图所示，4 皇后问题存在两个不同的解法。
// 示例 2：

// 输入：n = 1
// 输出：1

// 提示：
// 1 <= n <= 9

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_total_n_queens() {
        assert_eq!(Solution::total_n_queens(5), 10);
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
        assert_eq!(Solution::total_n_queens(2), 0)
    }
}

struct Solution;
impl Solution {
    // 52. N皇后 II
    // 1. 回溯
    // 1.1 在国际象棋中，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。总体呈现一个*字形。
    // 1.2 条件：设皇后的坐标为(x,y)， 那么皇后可以攻击的坐标有4类：
    // 1.2.1 (x,y+i) 横坐标相同
    // 1.2.22 (x+i,y) 纵坐标相同
    // 1.2.3 (x+i,y+i) 右斜线
    // 1.2.4 (x-i,y+i) 左斜线
    // 1.3 深度搜索条件
    // 1.3.1 由于n*n的棋盘中需要摆放n个皇后，由于上面的条件限制，所以每一行都需要摆放一个皇后，所以需要遍历n次
    // 1.3.2 所以我们可以设置一个递归函数 row 代表当前行，对于每一行，我们都尝试所有位置
    // 1.3.3 找到符合条件的位置后，将row + 1，然后重新进入1.3.2
    // 1.3.4 直到 row = n， 说明找到了一条可行路径
    // 2. 剪枝
    // 2.1 剪枝1： 如果某一行全部都放不下，则直接返回
    // 2.2 进入某一行时，可以直接计算出来这一行中所有可能放置的位置，这样只需对之前的放置的位置判断一次
    // 3. 优化
    pub fn total_n_queens(n: i32) -> i32 {
        let mut result = 0;
        let mut x_pos = vec![0; n as usize];
        queens_position(0, n as usize, &mut x_pos, &mut result);
        result
    }
}

fn queens_position(row: usize, size: usize, x_pos: &mut Vec<i32>, result: &mut i32) {
    if row == size {
        // println!("{:?}", x_pos);
        *result += 1;
    }

    for col in 0..size {
        if !can_put_queen(row, col, x_pos) {
            continue;
        }
        x_pos[row] = col as i32;
        queens_position(row + 1, size, x_pos, result)
    }
}

fn can_put_queen(row: usize, col: usize, x_pos: &Vec<i32>) -> bool {
    for i in 0..row {
        if x_pos[i] == col as i32 || (row - i) as i32 == (col as i32 - x_pos[i]).abs() {
            return false;
        }
    }
    true
}
