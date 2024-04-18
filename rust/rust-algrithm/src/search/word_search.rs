// https://leetcode.cn/problems/word-search/description/?envType=study-plan-v2&envId=top-interview-150
// 搜索单词 中等
// 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。
// *单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。
// *同一个单元格内的字母不允许被重复使用。

// 示例 1：
// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// 输出：true
// 示例 2：

// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
// 输出：true
// 示例 3：

// 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
// 输出：false

// 提示：
// m == board.length
// n = board[i].length
// 1 <= m, n <= 6
// 1 <= word.length <= 15
// board 和 word 仅由大小写英文字母组成

use std::collections::VecDeque;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_re_match() {
        assert_eq!(
            exist_for_test(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'E', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCESEEEFS"
            ),
            true
        );
        assert_eq!(
            exist_for_test(
                vec![
                    vec!['C', 'A', 'A'],
                    vec!['A', 'A', 'A'],
                    vec!['B', 'C', 'D'],
                ],
                "AAB"
            ),
            true
        );
        assert_eq!(exist_for_test(vec![vec!['a', 'a'],], "aaa"), false);
        assert_eq!(
            exist_for_test(vec![vec!['a', 'b'], vec!['c', 'd'],], "dbac"),
            true
        );
        assert_eq!(
            exist_for_test(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED"
            ),
            true
        );
        assert_eq!(
            exist_for_test(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "SEE"
            ),
            true
        );
        assert_eq!(
            exist_for_test(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB"
            ),
            false
        );
    }

    fn exist_for_test(board: Vec<Vec<char>>, word: &str) -> bool {
        Solution::exist(board, word.to_string())
    }
}

struct Solution;

const NULL: char = '0';
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board: Vec<Vec<char>> = board;
        let word: Vec<char> = word.as_str().bytes().map(|c| c as char).collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == word[0] && Solution::ds_check(&mut board, &word, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }

    pub fn ds_check(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        x: usize,
        y: usize,
        w: usize,
    ) -> bool {
        if board[x][y] != word[w] {
            return false;
        }
        board[x][y] = NULL;
        let wl = w + 1;
        if wl == word.len() {
            return true;
        }
        if x + 1 < board.len() && Solution::ds_check(board, word, x + 1, y, wl)
        {
            return true;
        }
        if y + 1 < board[0].len() && Solution::ds_check(board, word, x, y + 1, wl)
        {
            return true;
        }
        if y >= 1 && Solution::ds_check(board, word, x, y - 1, wl) {
            return true;
        }
        if x >= 1 && Solution::ds_check(board, word, x - 1, y, wl) {
            return true;
        }
        board[x][y] = word[w];
        false
    }

    // 这是一个错误的案例，广度优先搜索算法，不能正常的求出值会产生误标记
    // 应该尝试深度优先搜索算法，并采用回溯，把标记为访问的值还原
    pub fn exist2(board: Vec<Vec<char>>, word: String) -> bool {
        let wc: Vec<char> = word.as_str().bytes().map(|c| c as char).collect();
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut board: Vec<Vec<char>> = board;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] != wc[0] {
                    continue;
                }
                let mut alters: Vec<(usize, usize, char)> = vec![(i, j, board[i][j])];
                board[i][j] = NULL;
                queue.push_back((i, j, 0 as usize));

                while let Some((row, col, wi)) = queue.pop_front() {
                    if wi == wc.len() - 1 {
                        return true;
                    }
                    let wi = wi + 1;
                    let c = wc[wi];

                    if col + 1 < board[0].len() {
                        push_queue(&mut board, &mut queue, &mut alters, row, col + 1, wi, c);
                    }
                    if col >= 1 {
                        push_queue(&mut board, &mut queue, &mut alters, row, col - 1, wi, c)
                    }
                    if row + 1 < board.len() {
                        push_queue(&mut board, &mut queue, &mut alters, row + 1, col, wi, c)
                    }
                    if row >= 1 {
                        push_queue(&mut board, &mut queue, &mut alters, row - 1, col, wi, c)
                    }
                }
                for (row, col, c) in alters {
                    board[row][col] = c;
                }
            }
        }

        // println!("bord = {:?}", board);

        false
    }
}

#[inline]
fn push_queue(
    board: &mut Vec<Vec<char>>,
    queue: &mut VecDeque<(usize, usize, usize)>,
    alters: &mut Vec<(usize, usize, char)>,
    row: usize,
    col: usize,
    wi: usize,
    c: char,
) {
    if board[row][col] == c {
        alters.push((row, col, board[row][col]));
        board[row][col] = '0';
        queue.push_back((row, col, wi));
    }
}
