// https://leetcode.cn/problems/binary-tree-maximum-path-sum/description/?envType=study-plan-v2&envId=top-100-liked
// 124. 二叉树中的最大路径和 困难
// 二叉树中的 *路径* 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。
// 同一个节点在一条路径序列中 *至多出现一次* 。该路径 *至少包含一个* 节点，且不一定经过根节点。
// 路径和 是路径中各节点值的总和。
// 给你一个二叉树的根节点 root ，返回其 *最大路径和* 。

// 示例 1：
// 输入：root = [1,2,3]
// 输出：6
// 解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6

// 示例 2：
// 输入：root = [-10,9,20,null,null,15,7]
// 输出：42
// 解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42
 

// 提示：
// 树中节点数目范围是 [1, 3 * 10^4]
// -1000 <= Node.val <= 1000

use std::{cell::RefCell, rc::Rc, cmp};

const NULL: i32 = -1;

#[cfg(test)]
mod test {
    #[test]
    fn test_max_path_sum() {
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

// 该题的关键是使用哪种路径来遍历二叉树
// 1. 由于一个节点只能被访问一次，所以只能选择是要节点的左子树或者右子树或者父节点
// 2. 针对每条路径，可以建立一个动态规划数组，记录当前节点到根节点的最大路径和
// 3. 使用深度优先遍历，每次访问一个节点时，更新当前节点的最大路径和
// 4. 状态转移方程：dp[i] = max(v[i], dp[i-1])
// 4.1 注意由于父节点的位置，所以当前节点如果是左子树则和右子树的标识方法不同
// 4.1 如果当前节点是左子树，则 dp[i] = max(v[i], dp[i-1])
// 4.2 当前节点是右子树，则 dp[i] = max(v[i], dp[i-2])
// 5. 初始化：dp[0] = root.val
// 6. 记录 dp 数组的最大值

struct Solution;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}