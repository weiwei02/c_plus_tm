// 给定一个二叉树，判断它是否是 
// 平衡二叉树
// 
// 示例 1：
// 输入：root = [3,9,20,null,null,15,7]
// 输出：true
// 
// 示例 2：
// 输入：root = [1,2,2,3,3,null,null,4,4]
// 输出：false
// 
// 示例 3：
// 输入：root = []
// 输出：true
// 
// 树中的节点数在范围 [0, 5000] 内
// -104 <= Node.val <= 104

use std::{cell::RefCell, rc::Rc, cmp};

const NULL: i32 = -1;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sorted_list_to_bst() {
        let t = build_tree(&vec![3, 9, 20, NULL, NULL, 15, 7]);
        assert_eq!(is_balanced(t), true);

        let t = build_tree(&vec![1, 2, 2, 3, 3, NULL, NULL, 4, 4]);
        assert_eq!(is_balanced(t), false);
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

  pub fn heigh(&self) -> i32 {
    let left_heigh = match self.left.clone() {
        Some(node) => node.borrow().heigh(),
        None => 0,
    };
    let right_heigh = match self.right.clone() {
        Some(node) => node.borrow().heigh(),
        None => 0,
    };
    
    cmp::max(left_heigh, right_heigh) + 1
  }
}

// 使用深度优先遍历判断是否平衡二叉树
fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        let left_heigh = match node.borrow().left.clone() {
            Some(node) => node.borrow().heigh(),
            None => 0,
        };
        let right_heigh = match node.borrow().right.clone() {
            Some(node) => node.borrow().heigh(),
            None => 0,
            };
        return (left_heigh - right_heigh).abs() <= 1 && is_balanced(node.borrow().left.clone()) && is_balanced(node.borrow().right.clone())
    }
    true
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