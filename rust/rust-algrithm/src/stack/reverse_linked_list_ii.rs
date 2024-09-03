// https://leetcode.cn/problems/reverse-linked-list-ii
// 反转链表 II
// 中等
// 给你单链表的头指针 head 和两个整数 left 和 right ，其中 left <= right 。请你反转从位置 left 到位置 right 的链表节点，返回 反转后的链表 。
 

// 示例 1：
// 输入：head = [1,2,3,4,5], left = 2, right = 4
// 输出：[1,4,3,2,5]

// 示例 2：
// 输入：head = [5], left = 1, right = 1
// 输出：[5]
 
// 提示：
// 链表中节点数目为 n
// 1 <= n <= 500
// -500 <= Node.val <= 500
// 1 <= left <= right <= n

#[cfg(test)]
mod tests {
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
    // 翻转链表思路：
    // 1. 找到第 left - 1 个节点，记为 pre
    // 2. 从第 left 个节点开始，依次翻转链表中的每个节点
    // 3. 将第 pre 节点的 next 指向第 right 个节点
    // 4. 将 left 节点的 next 指向 end (right + 1) 个节点
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: -1, next: head }));
        let mut ptr = &mut dummy;
        
        for _ in 0..left - 1{
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        let mut curr = ptr.as_mut().unwrap().next.take();
        let mut next;
        for _ in left..right {
            next = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = ptr.as_mut().unwrap().next.take();
            ptr.as_mut().unwrap().next = next.take();
        }
        while ptr.as_mut().unwrap().next.is_some(){
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        //重新接上
        ptr.as_mut().unwrap().next = curr.take();
        dummy.unwrap().next
    }
}