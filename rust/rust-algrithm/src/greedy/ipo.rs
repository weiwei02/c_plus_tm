// https://leetcode.cn/problems/ipo/?envType=study-plan-v2&envId=top-interview-150
// 502. IPO
// 困难

// 假设 力扣（LeetCode）即将开始 IPO 。为了以更高的价格将股票卖给风险投资公司，力扣
// 希望在 IPO 之前开展一些项目以增加其资本。 由于资源有限，它只能在 IPO 之前完成最多 k 个不同的项目。
// 帮助 力扣 设计完成最多 k 个不同项目后得到最大总资本的方式。
//
// 给你 n 个项目。对于每个项目 i ，它都有一个纯利润 profits[i] ，和启动该项目需要的最小资本 capital[i] 。
// 最初，你的资本为 w 。当你完成一个项目时，你将获得纯利润，且利润将被添加到你的总资本中。
// 总而言之，从给定项目中选择 最多 k 个不同项目的列表，以 最大化最终资本 ，并输出最终可获得的最多资本。
// 答案保证在 32 位有符号整数范围内。

// 示例 1：
// 输入：k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
// 输出：4
// 解释：
// 由于你的初始资本为 0，你仅可以从 0 号项目开始。
// 在完成后，你将获得 1 的利润，你的总资本将变为 1。
// 此时你可以选择开始 1 号或 2 号项目。
// 由于你最多可以选择两个项目，所以你需要完成 2 号项目以获得最大的资本。
// 因此，输出最后最大化的资本，为 0 + 1 + 3 = 4。

// 示例 2：
// 输入：k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
// 输出：6

// 提示：
// 1 <= k <= 105
// 0 <= w <= 109
// n == profits.length
// n == capital.length
// 1 <= n <= 105
// 0 <= profits[i] <= 104
// 0 <= capital[i] <= 109

use std::{cell::RefCell, collections::BTreeMap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximized_capital() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
        assert_eq!(
            Solution::find_maximized_capital(10, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }
}

struct Solution;
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut profit_map: BTreeMap<i32, RefCell<Vec<(i32, i32)>>> = BTreeMap::new();

        // build profit_map
        for (i, &p) in profits.iter().enumerate() {
            let worth = capital[i];
            let tasks = if let Some(set) = profit_map.get(&p) {
                set.clone()
            } else {
                RefCell::new(Vec::new())
            };

            let mut index = 0;
            for (j, &c) in tasks.borrow().iter().enumerate() {
                if c.1 >= worth {
                    index = j;
                }
            }
            tasks.borrow_mut().insert(index, (p, worth));
            profit_map.insert(p, tasks);
        }

        let mut last_worth = w;
        let mut pc = k;

        let mut profit_key = profit_map.keys().rev().map(|&x| x).collect::<Vec<_>>().clone();

        while pc > 0 {
            
            let mut to_remove = -1;
            let mut key_index: usize = 0;
            for (keyi, &key) in profit_key.iter().enumerate() {
                if let Some(tasks) = profit_map.get(&key) {
                    let task = tasks.borrow();
                    if task[0].1 <= last_worth {
                        to_remove = key;
                        key_index = keyi;
                        last_worth += tasks.borrow()[0].0;
                        break;
                    }
                }
            }
            if to_remove != -1 {
                let mut to_remove_key = false;
                if let Some(tasks) = profit_map.get(&to_remove) {
                    if tasks.borrow().len() == 1 {
                        to_remove_key = true;
                    } else {
                        tasks.borrow_mut().remove(0);
                    }
                }
                if to_remove_key {
                    profit_map.remove(&to_remove);
                    profit_key.remove(key_index);
                }
            }
            pc -= 1;
        }

        last_worth
    }

    // 思路：由于总次数受限，故使用贪心法，寻找capital 在 current_worth 可接受下，profits 最大的项目
    // 注意是不同的项目，且是profits 最大的项目
    pub fn find_maximized_capital1(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut profits = profits;
        let mut capital = capital;
        let mut last_worth = w;
        let mut pc = k;

        // 对 profits 做快速排序，使最大值在最前，最小值在最后
        profits.sort();

        // 单纯的这个算法可能会出现超时，因为每次都要遍历所有项目
        while pc > 0 {
            let mut max_profit = 0;
            let mut index = 0;
            for (i, &p) in profits.iter().enumerate() {
                if capital[i] > last_worth {
                    continue;
                }
                if p > max_profit {
                    max_profit = p;
                    index = i;
                }
            }

            if max_profit == 0 {
                break;
            }
            last_worth += max_profit;
            profits.remove(index);
            capital.remove(index);
            pc -= 1;
        }

        last_worth
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct project {
    worth: i32,
    profit: i32,
}

impl project {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.worth.cmp(&other.worth)
    }
}
