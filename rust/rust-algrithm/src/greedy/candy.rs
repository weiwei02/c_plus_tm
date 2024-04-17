// https://leetcode.cn/problems/candy/description/?envType=study-plan-v2&envId=top-interview-150
// 135. 分发糖果 困难
// n 个孩子站成一排。给你一个整数数组 ratings 表示每个孩子的评分。

// 你需要按照以下要求，给这些孩子分发糖果：
// * 每个孩子至少分配到 1 个糖果。
// * 相邻两个孩子评分更高的孩子会获得更多的糖果。
// * 请你给每个孩子分发糖果，计算并返回需要准备的 最少糖果数目 。

// 示例 1：
// 输入：ratings = [1,0,2]
// 输出：5
// 解释：你可以分别给第一个、第二个、第三个孩子分发 2、1、2 颗糖果。

// 示例 2：
// 输入：ratings = [1,2,2]
// 输出：4
// 解释：你可以分别给第一个、第二个、第三个孩子分发 1、2、1 颗糖果。
//      第三个孩子只得到 1 颗糖果，这满足题面中的两个条件。

// 提示：
// n == ratings.length
// 1 <= n <= 2 * 104
// 0 <= ratings[i] <= 2 * 104

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_re_match() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
        assert_eq!(Solution::candy(vec![1, 2]), 3);
        assert_eq!(Solution::candy(vec![2, 1]), 3);
        assert_eq!(Solution::candy(vec![1, 1]), 2);
        assert_eq!(Solution::candy(vec![1, 1, 1]), 3);
        assert_eq!(Solution::candy(vec![1, 1, 1, 0]), 5);
    }
}

struct Solution;
impl Solution {
    // 优化：使用双向指针法
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        // 默认糖果全是0
        let mut ans: Vec<i32> = vec![1; n];
        // 顺推： 给  ratings[i] > ratings[i - 1] 多一个糖果
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                ans[i] = ans[i].max(ans[i - 1] + 1);
            }
        }
        // 逆推，找到 ratings[i] > ratings[i + 1] 并给 i 一个糖果
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                ans[i] = ans[i].max(ans[i + 1] + 1);
            }
        }
        ans.iter().sum()
    }
    // 思路：使用贪心算法
    // 1. 对于第i个孩子有两种分发糖果的方法
    // 1.1 如果 ratings[i] > ratings[i - 1] && rating[i] > rating[i + 1]则应该分发2个
    // 1.2 如果 ratings[i] == ratings[i - 1] 则应该分发1个

    // 1.3 复杂的是 如果 rating[i] < rating[i - 1] 则应该分发1个，但是应该回溯判断是否给之前的孩子补发
    // 1.4 回溯的逻辑是给 i-j 个孩子补发一个糖果， 直到 ratings[i - j] >= ratings[i - j - 1]
    pub fn candy2(ratings: Vec<i32>) -> i32 {
        let mut candys = vec![0; ratings.len()];

        candys[0] = 1;
        for i in 1..ratings.len() {
            if ratings[i] == ratings[i - 1] {
                candys[i] = 1;
            } else if ratings[i] > ratings[i - 1] {
                candys[i] = candys[i - 1] + 1;
            } else if ratings[i] < ratings[i - 1] {
                candys[i] = 1;
                // back
                let mut j = i - 1;
                while ratings[j] > ratings[j + 1] && candys[j] <= candys[j + 1] {
                    candys[j] = candys[j + 1] + 1;
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
            }
        }

        // println!("candys = {:?}", candys);
        candys.iter().sum()
    }
}
