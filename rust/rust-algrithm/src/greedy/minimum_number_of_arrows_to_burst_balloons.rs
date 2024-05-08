// https://leetcode.cn/problems/minimum-number-of-arrows-to-burst-balloons/?envType=study-plan-v2&envId=top-interview-150
// 452. 用最少数量的箭引爆气球
// 中等

// 有一些球形气球贴在一堵用 XY 平面表示的墙面上。墙面上的气球记录在整数数组 points ，
// 其中points[i] = [xstart, xend] 表示水平直径在 xstart 和 xend之间的气球。你不知道气球的确切 y 坐标。
// 一支弓箭可以沿着 x 轴从不同点 完全垂直 地射出。在坐标 x 处射出一支箭，若有一个气球的直径的开始和结束
// 坐标为 xstart，xend， 且满足  xstart ≤ x ≤ xend，则该气球会被 引爆 。可以射出的弓箭的数量 没有限制 。
//  弓箭一旦被射出之后，可以无限地前进。
// 给你一个数组 points ，返回引爆所有气球所必须射出的*最小*弓箭数 。

 
// 示例 1：
// 输入：points = [[10,16],[2,8],[1,6],[7,12]]
// 输出：2
// 解释：气球可以用2支箭来爆破:
// -在x = 6处射出箭，击破气球[2,8]和[1,6]。
// -在x = 11处发射箭，击破气球[10,16]和[7,12]。

// 示例 2：
// 输入：points = [[1,2],[3,4],[5,6],[7,8]]
// 输出：4
// 解释：每个气球需要射出一支箭，总共需要4支箭。

// 示例 3：
// 输入：points = [[1,2],[2,3],[3,4],[4,5]]
// 输出：2
// 解释：气球可以用2支箭来爆破:
// - 在x = 2处发射箭，击破气球[1,2]和[2,3]。
// - 在x = 4处射出箭，击破气球[3,4]和[4,5]。

// 提示:
// 1 <= points.length <= 1^05
// points[i].length == 2
// -2^31 <= xstart < xend <= 2^31 - 1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_arrow_shots() {
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]), 2);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]), 4);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), 2);
    }
}

struct Solution;
impl Solution {
    // 排序+贪心算法
    // 设总箭数 arrows = 1
    // 1. 按照每个区间的右边界先进行从小到大排序
    // 2. 遍历排序后的区间，如果当前区间的左边界小于等于上一个区间的右边界，则说明可以 burst 当前区间
    // 3. 否则不能 burst,将 arrows + 1。在这时右侧其实存在可以被这支箭爆破的区间，但由于区间都是经过排序的，
    // 能被当前的箭爆破的就一定能被后面的箭爆破
    // 4. 重复2
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|pa, pb| pa[1].cmp(&pb[1]));
        let mut arrows = 1;
        let mut last_point = points[0][1];
        for i in 1..points.len() {
            if points[i][0] > last_point {
                arrows += 1;
                last_point = points[i][1];
            }
        }
        arrows
    }
}