// https://leetcode.cn/problems/gas-station/description/?envType=study-plan-v2&envId=top-interview-150
// 134. 加油站 中等
// 在一条环路上有 N 个加油站，其中第 i 个加油站有汽油 gas[i] 升
// 在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。
// 你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。
// 给定两个整数数组 gas 和 cost ，如果你可以按顺序绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一 的。

// 示例 1:
// 输入: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
// 输出: 3
// 解释:
// 从 3 号加油站(索引为 3 处)出发，可获得 4 升汽油。此时油箱有 = 0 + 4 = 4 升汽油
// 开往 4 号加油站，此时油箱有 4 - 1 + 5 = 8 升汽油
// 开往 0 号加油站，此时油箱有 8 - 2 + 1 = 7 升汽油
// 开往 1 号加油站，此时油箱有 7 - 3 + 2 = 6 升汽油
// 开往 2 号加油站，此时油箱有 6 - 4 + 3 = 5 升汽油
// 开往 3 号加油站，你需要消耗 5 升汽油，正好足够你返回到 3 号加油站。
// 因此，3 可为起始索引。

// 示例 2:
// 输入: gas = [2,3,4], cost = [3,4,3]
// 输出: -1
// 解释:
// 你不能从 0 号或 1 号加油站出发，因为没有足够的汽油可以让你行驶到下一个加油站。
// 我们从 2 号加油站出发，可以获得 4 升汽油。 此时油箱有 = 0 + 4 = 4 升汽油
// 开往 0 号加油站，此时油箱有 4 - 3 + 2 = 3 升汽油
// 开往 1 号加油站，此时油箱有 3 - 3 + 3 = 3 升汽油
// 你无法返回 2 号加油站，因为返程需要消耗 4 升汽油，但是你的油箱只有 3 升汽油。
// 因此，无论怎样，你都不可能绕环路行驶一周。
 

// 提示:
// gas.length == n
// cost.length == n
// 1 <= n <= 105
// 0 <= gas[i], cost[i] <= 104

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
        assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
        assert_eq!(Solution::can_complete_circuit(vec![3,1,1], vec![1,2,2]), 0);
        assert_eq!(Solution::can_complete_circuit(vec![4], vec![5]), -1);
    }
}

struct Solution;
impl Solution {

    // 暴力解法： 求最后是否有足够的油可以绕一周，只要最终total的结果为正数，则说明可以绕一周
    // 如果可以绕行一周，则最后的current_sum 可以定位到起点
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut current_sum = 0;
        let mut total_sum = 0;

        for (i, (&gas, &cost)) in gas.iter().zip(cost.iter()).enumerate() {
            current_sum += gas - cost;
            total_sum += gas - cost;
            if current_sum < 0 {
                start = i + 1;
                current_sum = 0;
            }
        }

        if total_sum < 0 {
            return -1;
        }

        start as i32
    }

    // 1. 暴力解法
    // 若要绕环路行驶一周，则假设从 i 出发
    // 如果从 j 处剩余油量无法撑过 cost[j], 则尝试从 j+1 处开始
    // 如果 (j + 1) % gas.len() == begin，则说明已经到达起点，可以绕一周
    pub fn can_complete_circuit1(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut begin = 0;

        let mut gas_sum = 0;
        let mut cur = 0;

        if gas.len() == 1 {
            if gas[0] >= cost[0] {
                return 0;
            } else {
                return -1;
            }
        }

        let mut retry: usize = 0;
        loop {
            gas_sum += gas[cur] - cost[cur];

            // 不能到达 next， 则尝试设置从 next 开始
            if gas_sum < 0 {
                // 已经尝试了一圈，依旧不能到达起点
                if cur < begin {
                    return -1;
                }
                begin = cur + 1;
                gas_sum = 0;
                retry = 0;
            } 

            cur = (cur + 1) % gas.len();
            // 已经到达起点
            retry += 1;
            if retry == gas.len() + 1 {
                break;
            }
        }

        begin as i32
    }
}