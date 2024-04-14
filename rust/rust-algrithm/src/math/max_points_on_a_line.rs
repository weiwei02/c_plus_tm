// Max Points on a Line
// https://leetcode.cn/problems/max-points-on-a-line/description/?envType=study-plan-v2&envId=top-interview-150
// 149 直线上最多的点 困难
// 给你一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点。
// 求在笛卡尔坐标轴上最多有多少个点在同一条直线上。

// 示例 1：
// 输入：points = [[1,1],vec![2,2],vec![3,3]]
// 输出：3

// 示例 2：
// 输入：points = [[1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]]
// 输出：4

// 提示：
// 1 <= points.length <= 300
// points[i].length == 2
// -104 <= xi, yi <= 104
// points 中的所有点 互不相同

use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_points() {
        assert_eq!(Solution::max_points(vec![vec![0, 1], vec![0, 0]]), 2);
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![vec![4, 5], vec![4, -1], vec![4, 0]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4],
            ]),
            4
        );
        assert_eq!(Solution::max_points(vec![vec![0, 0],]), 1);
        assert_eq!(
            Solution::max_points(vec![
                vec![7, 3],
                vec![19, 19],
                vec![-16, 3],
                vec![13, 17],
                vec![-18, 1],
                vec![-18, -17],
                vec![13, -3],
                vec![3, 7],
                vec![-11, 12],
                vec![7, 19],
                vec![19, -12],
                vec![20, -18],
                vec![-16, -15],
                vec![-10, -15],
                vec![-16, -18],
                vec![-14, -1],
                vec![18, 10],
                vec![-13, 8],
                vec![7, -5],
                vec![-4, -9],
                vec![-11, 2],
                vec![-9, -9],
                vec![-5, -16],
                vec![10, 14],
                vec![-3, 4],
                vec![1, -20],
                vec![2, 16],
                vec![0, 14],
                vec![-14, 5],
                vec![15, -11],
                vec![3, 11],
                vec![11, -10],
                vec![-1, -7],
                vec![16, 7],
                vec![1, -11],
                vec![-8, -3],
                vec![1, -6],
                vec![19, 7],
                vec![3, 6],
                vec![-1, -2],
                vec![7, -3],
                vec![-6, -8],
                vec![7, 1],
                vec![-15, 12],
                vec![-17, 9],
                vec![19, -9],
                vec![1, 0],
                vec![9, -10],
                vec![6, 20],
                vec![-12, -4],
                vec![-16, -17],
                vec![14, 3],
                vec![0, -1],
                vec![-18, 9],
                vec![-15, 15],
                vec![-3, -15],
                vec![-5, 20],
                vec![15, -14],
                vec![9, -17],
                vec![10, -14],
                vec![-7, -11],
                vec![14, 9],
                vec![1, -1],
                vec![15, 12],
                vec![-5, -1],
                vec![-17, -5],
                vec![15, -2],
                vec![-12, 11],
                vec![19, -18],
                vec![8, 7],
                vec![-5, -3],
                vec![-17, -1],
                vec![-18, 13],
                vec![15, -3],
                vec![4, 18],
                vec![-14, -15],
                vec![15, 8],
                vec![-18, -12],
                vec![-15, 19],
                vec![-9, 16],
                vec![-9, 14],
                vec![-12, -14],
                vec![-2, -20],
                vec![-3, -13],
                vec![10, -7],
                vec![-2, -10],
                vec![9, 10],
                vec![-1, 7],
                vec![-17, -6],
                vec![-15, 20],
                vec![5, -17],
                vec![6, -6],
                vec![-11, -8]
            ]),
            6
        );
    }
}

// 求解思路：
// 1. 计算出所有点的斜率，然后统计函数出现的次数
// 2. 一元一次函数：y = kx + b，最少知道两个点才能求出相关函数
// 3. 以函数为key，统计满足函数出现的次数
// 4. 以每两点为基准，先使用已有函数尝试匹配两点，如果所有函数都不匹配，则求出新函数
// 5. 每个点都要尝试与其它点相连，所以时间复杂度是O(n^2)
struct Solution;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut func_count_map: HashMap<String, HashSet<String>> = HashMap::new();
        let point_key = points
            .iter()
            .map(|p| format!("{:?}", p))
            .collect::<Vec<_>>();
        let mut max_count = 1;
        if points.len() >= 2 {
            max_count = 2;
        }

        // 计算出所有函数
        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                let line = Solution::line_func(&points[i], &points[j]);
                if line.is_empty() {
                    continue;
                }
                let mut set = match func_count_map.get(&line) {
                    Some(set) => set.clone(),
                    None => HashSet::new(),
                };
                set.insert(point_key[i].clone());
                set.insert(point_key[j].clone());

                if max_count < set.len() as i32 {
                    max_count = set.len() as i32;
                }

                func_count_map.insert(line, set);
            }
        }

        if points.len() == 3 {
            func_count_map.iter().
            // filter(|(_, v)| v.len() > 6).
            for_each(|(key, v)|{
                println!("key = {}, value = {:?}", key, v);
            });
        }
        max_count
    }

    // y = kx + b
    // 1 = 4k +b
    // 3 = 5b + b
    // y2 - y1 = k(x2 - x1)
    fn line_func(p1: &Vec<i32>, p2: &Vec<i32>) -> String {
        let k = if p2[0] - p1[0] > 0 {
            (p2[1] - p1[1]) as f64 / (p2[0] - p1[0]) as f64
        } else {
            (p1[1] - p2[1]) as f64 / (p1[0] - p2[0]) as f64
        };
        let b = p1[1] as f64 - k * p1[0] as f64;
        if k == f64::INFINITY || k == f64::NEG_INFINITY {
            if p2[0] - p1[0] == 0 {
                return format!("x = {}", p1[0]);
            }
            if p2[1] - p1[1] == 0 {
                return format!("y = {}", p1[1]);
            }
            return String::new();
        }
        format!("{},{}", k, b)
    }
}
