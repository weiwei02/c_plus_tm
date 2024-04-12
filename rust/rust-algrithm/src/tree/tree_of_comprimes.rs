// https://leetcode.cn/problems/tree-of-coprimes/description/?envType=daily-question&envId=2024-04-11
// 1766. Tree of Coprimes
// 困难
//
// 给你一个 n 个节点的树（也就是一个无环连通无向图），节点编号从 0 到 n - 1 ，且恰好有 n - 1 条边，每个节点有一个值。树的 根节点 为 0 号点。
// 给你一个整数数组 nums 和一个二维数组 edges 来表示这棵树。nums[i] 表示第 i 个点的值，edges[j] = [uj, vj] 表示节点 uj 和节点 vj 在树中有一条边。
// 当 gcd(x, y) == 1 ，我们称两个数 x 和 y 是 互质的 ，其中 gcd(x, y) 是 x 和 y 的 最大公约数 。
// 从节点 i 到 根 最短路径上的点都是节点 i 的祖先节点。一个节点 不是 它自己的祖先节点。
// 请你返回一个大小为 n 的数组 ans ，其中 ans[i]是离节点 i 最近的祖先节点且满足 nums[i] 和 nums[ans[i]] 是 互质的 ，如果不存在这样的祖先节点，ans[i] 为 -1 。

// 示例 1：
// 输入：nums = [2,3,3,2], edges = [[0,1],vec![1,2],vec![1,3]]
// 输出：[-1,0,0,1]
// 解释：上图中，每个节点的值在括号中表示。
// - 节点 0 没有互质祖先。
// - 节点 1 只有一个祖先节点 0 。它们的值是互质的（gcd(2,3) == 1）。
// - 节点 2 有两个祖先节点，分别是节点 1 和节点 0 。节点 1 的值与它的值不是互质的（gcd(3,3) == 3）但节点 0 的值是互质的(gcd(2,3) == 1)，所以节点 0 是最近的符合要求的祖先节点。
// - 节点 3 有两个祖先节点，分别是节点 1 和节点 0 。它与节点 1 互质（gcd(3,2) == 1），所以节点 1 是离它最近的符合要求的祖先节点。

// 示例 2：
// 输入：nums = [5,6,10,2,3,6,15], edges = [[0,1],vec![0,2],vec![1,3],vec![1,4],vec![2,5],vec![2,6]]
// 输出：[-1,0,-1,0,0,0,-1]

// 提示：
// nums.length == n
// 1 <= nums[i] <= 50
// 1 <= n <= 105
// edges.length == n - 1
// edges[j].length == 2
// 0 <= uj, vj < n
// uj != vj

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        assert_eq!(
            Solution::get_coprimes(vec![2, 3, 3, 2], vec![vec![0, 1], vec![1, 2], vec![1, 3]]),
            vec![-1, 0, 0, 1]
        );
        assert_eq!(
            Solution::get_coprimes(
                vec![5, 6, 10, 2, 3, 6, 15],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ]
            ),
            vec![-1, 0, -1, 0, 0, 0, -1]
        );


        // 考虑树不是完全联通的，如果a 是根节点， b和c连接但不与a 连接。
        // assert_eq!(
        //     Solution::get_coprimes(
        //         vec![
        //             9, 16, 30, 23, 33, 35, 9, 47, 39, 46, 16, 38, 5, 49, 21, 44, 17, 1, 6, 37, 49,
        //             15, 23, 46, 38, 9, 27, 3, 24, 1, 14, 17, 12, 23, 43, 38, 12, 4, 8, 17, 11, 18,
        //             26, 22, 49, 14, 9
        //         ],
        //         vec![
        //             vec![17, 0],
        //             vec![30, 17],
        //             vec![41, 30],
        //             vec![10, 30],
        //             vec![13, 10],
        //             vec![7, 13],
        //             vec![6, 7],
        //             vec![45, 10],
        //             vec![2, 10],
        //             vec![14, 2],
        //             vec![40, 14],
        //             vec![28, 40],
        //             vec![29, 40],
        //             vec![8, 29],
        //             vec![15, 29],
        //             vec![26, 15],
        //             vec![23, 40],
        //             vec![19, 23],
        //             vec![34, 19],
        //             vec![18, 23],
        //             vec![42, 18],
        //             vec![5, 42],
        //             vec![32, 5],
        //             vec![16, 32],
        //             vec![35, 14],
        //             vec![25, 35],
        //             vec![43, 25],
        //             vec![3, 43],
        //             vec![36, 25],
        //             vec![38, 36],
        //             vec![27, 38],
        //             vec![24, 36],
        //             vec![31, 24],
        //             vec![11, 31],
        //             vec![39, 24],
        //             vec![12, 39],
        //             vec![20, 12],
        //             vec![22, 12],
        //             vec![21, 39],
        //             vec![1, 21],
        //             vec![33, 1],
        //             vec![37, 1],
        //             vec![44, 37],
        //             vec![9, 44],
        //             vec![46, 2],
        //             vec![4, 46]
        //         ]
        //     ),
        //     vec![
        //         -1, 21, 17, 43, 10, 42, 7, 13, 29, 44, 17, 31, 39, 10, 10, 29, 32, 0, 40, 23, 12,
        //         39, 12, 40, 25, 35, 15, 38, 40, 40, 17, 24, 5, 1, 19, 14, 17, 21, 25, 24, 14, 17,
        //         40, 25, 37, 17, 10
        //     ]
        // );
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 1. 由于 n 节点树时一个无环联通无向图，所以在 edges[j] = [uj, vj] 中 uj 在 nums 的索引一定小于vj，否则打破了无环联通无向图
    // 2. 对于一个节点的祖先节点，可以通过 for i = j; i > 0; j-- 遍历edge[i],找到i的第一个祖先节点
    // 3. （排除）如果一个节点的父节点没有互质祖先，那么这个节点仅需判断父节点是否与自己互质即可，所以可以借助动态规划的临时存储空间，记录每个节点是否与自己互质
    // 4. 如果一个节点的父节点没有互质节点，也需要判断这个节点与祖节点是否互质
    // 5. 如果一个节点的父节点有互质祖先，那么这个节点与祖先节点互质
    // 6. 可以构建一个数组 p[i] = parent 存储每个节点的父节点，通过这种方式可以快速找到每个节点的父节点与祖先节点
    // 7. 由于是无环联通无向图，考虑树不是完全联通的，如果a 是根节点， b和c连接但不与a 连接。重点是无向
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent_index = vec![-1; nums.len()];

        let mut parent_coprimes: Vec<i32> = vec![-1; nums.len()];
        for edge in edges {
            if edge[0] > edge[1] {
                parent_index[edge[0] as usize] = edge[1];
            } else {
                parent_index[edge[1] as usize] = edge[0];
            }
        }
        println!("parent_index: {:?}", parent_index);
        let mut i = parent_index.len() - 1;
        while i > 0 {
            let mut last_parent = parent_index[i];
            while last_parent >= 0 {
                // println!("gcd({}, {}) = {}", nums[i as usize], nums[parent_index[index] as usize], Solution::gcd(nums[i as usize], nums[parent_index[index] as usize]));
                if Solution::gcd(nums[i as usize], nums[last_parent as usize]) == 1 {
                    parent_coprimes[i] = last_parent;
                    break;
                }
                last_parent = parent_index[last_parent as usize];
            }
            // println!("parent_coprimes: {:?}, i = {}", parent_coprimes, i);
            i -= 1;
        }
        parent_coprimes
    }

    // 辗转相除法
    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
