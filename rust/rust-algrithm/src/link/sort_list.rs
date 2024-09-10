// https://leetcode.cn/problems/sort-list
// 排序链表
// 中等
// 给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

// 示例 1：
// 输入：head = [4,2,1,3]
// 输出：[1,2,3,4]

// 示例 2：
// 输入：head = [-1,5,3,4,0]
// 输出：[-1,0,3,4,5]

// 示例 3：
// 输入：head = []
// 输出：[]

// 提示：
// 链表中节点的数目在范围 [0, 5 * 10^4] 内
// -10^5 <= Node.val <= 10^5

// 进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        // assert_eq!(lru.get(7), 7);
    }
}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    // 思路: 使用归并排序
    // 1. 借助数组，先遍历链表，将链表中的元素放入数组中
    // 2. 借助归并排序，将数组中的元素进行排序
    // 注意，下面这种解法由于借助了数组，会有时间超限的问题
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr: Vec<Box<ListNode>> = Vec::new();
        let mut head_copy = head.clone();
        while let Some(node) = head_copy {
            arr.push(node.clone());
            head_copy = node.next.clone();
        }

        let size = arr.len();
        if size <= 1 {
            return head;
        }
        Self::quick_sort(&mut arr, 0, size - 1);

        Self::resort_list(&mut arr)
    }

    fn resort_list_with_raw_point(arr: &mut Vec<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("======start sort len : {}", arr.len());
        let mut head = arr[0].clone();
        let mut prev = &mut head as *mut Box<ListNode>;
        for i in 1..arr.len() {
            unsafe {
                print!("{},", (*prev).val);
                (*prev).next = Some(arr[i].clone());
                prev = (*prev).next.as_mut().unwrap() as *mut Box<ListNode>;
            }
        }
        unsafe {
            (*prev).next = None;
        }
        Some(head.clone())
    }

    fn resort_list(arr: &mut Vec<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = arr[0].clone();
        let mut prev = &mut head;
        for i in 1..arr.len() {
            print!("{},", (*prev).val);
            prev.next = Some(arr[i].clone());
            prev = prev.next.as_mut().unwrap();
        }
        prev.next = None;
        Some(head.clone())
    }

    fn quick_sort(arr: &mut Vec<Box<ListNode>>, left: usize, right: usize) {
        if left < right {
            let mid = Self::partition(arr, left, right);
            if mid > 0 {
                Self::quick_sort(arr, left, mid - 1);
            }
            Self::quick_sort(arr, mid + 1, right);
        }
    }

    fn partition(arr: &mut Vec<Box<ListNode>>, left: usize, right: usize) -> usize {
        let (mut left, mut right) = (left, right);
        let pivot = arr[left].val;
        while left < right {
            while left < right && arr[right].val > pivot {
                right -= 1;
            }
            arr[left] = arr[right].clone();
            while left < right && arr[left].val <= pivot {
                left += 1;
            }
            arr[right] = arr[left].clone();
        }
        arr[left] = Box::new(ListNode { val: pivot, next: None });
        left
    }

    fn merge_sort(arr: &mut Vec<Box<ListNode>>, left: usize, right: usize) {
        if left < right {
            let mid = (left + right) / 2;
            Self::merge_sort(arr, left, mid);
            Self::merge_sort(arr, mid + 1, right);
            Self::merge(arr, left, mid, right);
        }
    }

    fn merge(arr: &mut Vec<Box<ListNode>>, left: usize, mid: usize, right: usize) {
        let mut left_arr = Vec::new();
        let mut right_arr = Vec::new();
        for i in left..mid + 1 {
            left_arr.push(arr[i].clone());
        }
        for j in mid + 1..right + 1 {
            right_arr.push(arr[j].clone());
        }

        let (mut i, mut j) = (left, 0);
        while left_arr.len() > 0 && right_arr.len() > 0 {
            arr[i] = if left_arr[0].val < right_arr[j].val {
                left_arr.remove(0)
            } else {
                right_arr.remove(0)
            };
            i += 1;
        }
        while left_arr.len() > 0 {
            arr[i] = left_arr.remove(0);
            i += 1;
        }
        while right_arr.len() > 0 {
            arr[i] = right_arr.remove(0);
            i += 1;
        }
    }
}
