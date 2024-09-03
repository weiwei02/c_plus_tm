// https://leetcode.cn/problems/binary-tree-level-order-traversal
// 二叉树的层序遍历
// 给你二叉树的根节点 root ，返回其节点值的 层序遍历 。 （即逐层地，从左到右访问所有节点）。

// 示例 1：
// 输入：root = [3,9,20,null,null,15,7]
// 输出：[[3],[9,20],[15,7]]

// 示例 2：
// 输入：root = [1]
// 输出：[[1]]

// 示例 3：
// 输入：root = []
// 输出：[]

// 提示：
// 树中节点数目在范围 [0, 2000] 内
// -1000 <= Node.val <= 1000

use std::collections::VecDeque;
const NULL: i32 = -2000;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_level_order() {
        let t = build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7]);
        assert_eq!(
            Solution::level_order(t),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );

        let t = build_tree(&vec![1]);
        assert_eq!(Solution::level_order(t), vec![vec![1]]);
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    // 解决二叉树层序遍历
    // 思路：
    // 1. 使用队列保存每一层的节点，并记录当前层数
    // 2. 遍历当前层数，将下一层加入到队列中
    // 3. 循环执行步骤2
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut q = VecDeque::new();
        if let Some(root) = root {
            q.push_back(root);
        }
        while !q.is_empty() {
            let mut temp = VecDeque::new();
            let mut data = Vec::new();
            while let Some(mut node) = q.pop_front() {
                let node = Rc::into_inner(node).unwrap();
                let mut node = RefCell::into_inner(node);
                data.push(node.val);
                if let Some(left) = node.left.take() {
                    temp.push_back(left);
                }
                if let Some(right) = node.right.take() {
                    temp.push_back(right);
                }
            }
            result.push(data);
            q = temp;
        }
        result
    }
}

fn build_tree(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_node(nums, 0)
}

fn build_tree_node(nums: &Vec<i32>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if index >= nums.len() {
        return None;
    }
    if nums[index] == NULL {
        return None;
    }
    let node = Rc::new(RefCell::new(TreeNode::new(nums[index])));
    node.borrow_mut().left = build_tree_node(nums, 2 * index + 1);
    node.borrow_mut().right = build_tree_node(nums, 2 * index + 2);
    Some(node)
}
