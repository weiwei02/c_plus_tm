// https://leetcode.cn/problems/reverse-nodes-in-k-group/description/?envType=study-plan-v2&envId=top-interview-150
// 25. K 个一组翻转链表
// 困难
// 给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。
// k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
// 你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。

// 示例 1：
// 输入：head = [1,2,3,4,5], k = 2
// 输出：[2,1,4,3,5]

// 示例 2：
// 输入：head = [1,2,3,4,5], k = 3
// 输出：[3,2,1,4,5]
 
// 提示：
// 链表中的节点数目为 n
// 1 <= k <= n <= 5000
// 0 <= Node.val <= 1000
 

// 进阶：你可以设计一个只用 O(1) 额外内存空间的算法解决此问题吗？

use std::{collections::LinkedList, rc::Rc};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_schedule() {
        assert_eq!(
            Solution::can_finish(4, vec![vec![0, 10], vec![3, 18], vec![5, 5]]),
            false
        );
    }
}

struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut next_head = &mut head;
        // 获取下一个头
        for _ in 0..k {
            if let Some(node) = next_head {
                next_head = &mut node.next;
            } else {
                return head;
            }
        }

        let mut new_link_head = Solution::reverse_k_group(next_head.take(), k);
        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_link_head.take();
                new_link_head = Some(node);
            } 
        }

        new_link_head
    }
}