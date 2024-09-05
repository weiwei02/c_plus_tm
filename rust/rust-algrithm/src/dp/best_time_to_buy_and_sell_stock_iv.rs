// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv
// 买卖股票的最佳时机 IV
// 困难
// 给你一个整数数组 prices 和一个整数 k ，其中 prices[i] 是某支给定的股票在第 i 天的价格。
// 设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。也就是说，你最多可以买 k 次，卖 k 次。

// 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

// 示例 1：
// 输入：k = 2, prices = [2,4,1]
// 输出：2
// 解释：在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。

// 示例 2：
// 输入：k = 2, prices = [3,2,6,5,0,3]
// 输出：7
// 解释：在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
//      随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。

// 提示：
// 1 <= k <= 100
// 1 <= prices.length <= 1000
// 0 <= prices[i] <= 1000

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(2, vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(2, vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(2, vec![1]), 0);
        assert_eq!(Solution::max_profit(2, vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(2, vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 1]), 0);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}

struct Solution;
impl Solution {
    // 思路：
    // 1. 由于区分多次交易，所以整个过程分为多次，第一次交易和第二次交易。程序可以设计4种状态，分别表示第一次交易和第二次交易的利润情况。
    // 2. 第一次交易：buy1 = min(buy1, prices[i]) sell1 = (max, price[i] - buy1)
    // 3. 第二次交易：buy2 = sell1 - prices[i] sell2 = max(price[i] + buy2)
    // 4. 由于是最大有两次交易机会，并非限制了正好交易两次。所以最大利润是 max(sell1, sell2) 的最大值即为所求的最大利润。
    // 5. 第一次交易和第二次交易的利润情况，分别表示为：
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let (mut buyes, mut sells) = (vec![-prices[0]; k as usize], vec![0; k as usize]);
        for i in 1..prices.len() {
            for j in 0..k as usize {
                if j == 0 {
                    buyes[j] = buyes[j].max(-prices[i]);
                    sells[j] = sells[j].max(prices[i] + buyes[j]);
                } else {
                    buyes[j] = buyes[j].max(sells[j - 1] - prices[i]);
                    sells[j] = sells[j].max(prices[i] + buyes[j]);
                }
            }
        }
        *sells.iter().max().unwrap()
    }
}
