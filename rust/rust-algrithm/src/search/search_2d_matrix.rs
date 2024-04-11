// https://leetcode.cn/problems/search-a-2d-matrix/description/?envType=study-plan-v2&envId=top-100-liked
// 
// 搜索二维矩阵
// 
// 给你一个满足下述两条属性的 m x n 整数矩阵：
// * 每行中的整数从左到右按非严格递增顺序排列。
// * 每行的第一个整数大于前一行的最后一个整数。
// 给你一个整数 target ，如果 target 在矩阵中，返回 true ；否则，返回 false 。
// 
// 示例 1：
// 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
// 输出：true
// 
// 示例 2：
// 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
// 输出：false

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::search_matrix(vec![
            vec![1, 3, 5, 7], 
            vec![10, 11, 16, 20], 
            vec![23, 30, 34, 60],
        ], 3), true);

        assert_eq!(Solution::search_matrix(vec![
            vec![1, 3, 5, 7], 
            vec![10, 11, 16, 20], 
            vec![23, 30, 34, 60],
        ], 13), false);
        assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
        assert_eq!(Solution::search_matrix(vec![vec![1, 1]], 2), false);
        assert_eq!(Solution::search_matrix(vec![vec![1], vec![3]], 3), true);
        assert_eq!(Solution::search_matrix(vec![vec![1], vec![3], vec![5]], 5), true);
        assert_eq!(Solution::search_matrix(vec![vec![1], vec![3], vec![5]], 3), true);
        assert_eq!(Solution::search_matrix(vec![vec![1], vec![3], vec![5]], 1), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,2], vec![3,4], vec![5,6]], 5), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,2,3, 4], vec![4,5,7,8]], 7), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,2,3, 4], vec![4,5,7,8]], 8), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,2,3, 4], vec![4,5,7,8]], 3), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,2,3, 4], vec![4,5,7,8]], 9), false);
    }
}

struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Solution::inner_search(&matrix, 0, (matrix.len() * matrix[0].len()) as i32, target)
    }

    fn inner_search(matrix: &Vec<Vec<i32>>, left: i32, righ: i32, target: i32) -> bool {
        if left > righ {
            return false;
        }

        let mid = left + (righ - left) / 2;
        let mut row = mid / matrix[0].len() as i32;
        if row >= matrix.len() as i32 {
            row = matrix.len() as i32 - 1;
        }
        let col = mid % matrix[0].len() as i32;
        match matrix[row as usize][col as usize].cmp(&target) {
            std::cmp::Ordering::Less => return Solution::inner_search(matrix, mid + 1, righ, target),
            std::cmp::Ordering::Greater => return Solution::inner_search(matrix, left, mid - 1, target),
            _ => {
                return true;
            }
        }
    }
}