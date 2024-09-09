// https://leetcode.cn/problems/lru-cache
// LRU 缓存
// 中等
// 请你设计并实现一个满足  LRU (最近最少使用) 缓存 约束的数据结构。
// 实现 LRUCache 类：
// LRUCache(int capacity) 以 正整数 作为容量 capacity 初始化 LRU 缓存
// int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。
// void put(int key, int value) 如果关键字 key 已经存在，则变更其数据值 value ；如果不存在，则向缓存中插入该组 key-value 。如果插入操作导致关键字数量超过 capacity ，则应该 逐出 最久未使用的关键字。
// 函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。

 

// 示例：
// 输入
// ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
// 输出
// [null, null, null, 1, null, -1, null, -1, 3, 4]

// 解释
// LRUCache lRUCache = new LRUCache(2);
// lRUCache.put(1, 1); // 缓存是 {1=1}
// lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
// lRUCache.get(1);    // 返回 1
// lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
// lRUCache.get(2);    // 返回 -1 (未找到)
// lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
// lRUCache.get(1);    // 返回 -1 (未找到)
// lRUCache.get(3);    // 返回 3
// lRUCache.get(4);    // 返回 4
 

// 提示：

// 1 <= capacity <= 3000
// 0 <= key <= 10000
// 0 <= value <= 105
// 最多调用 2 * 105 次 get 和 put

use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut lru = LRUCache::new(5);
        lru.put(1, 1);
        lru.put(2, 2);
        lru.put(3, 3);
        lru.put(5, 5);

        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(5), 5);

        lru.put(4, 4);
        lru.put(6, 6);
        assert_eq!(lru.get(6), 6);
        assert_eq!(lru.get(1), -1);
        lru.put(7, 7);
        assert_eq!(lru.get(2), -1);
        assert_eq!(lru.get(7), 7);
    }
}


struct LRUCache {
    capacity: usize,
    hashmap: std::collections::HashMap<i32, Option<Rc<RefCell<ListNode>>>>,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

#[derive(Default)]
struct ListNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 *//**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            hashmap: std::collections::HashMap::new(),
            head: None,
            tail: None,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if !self.hashmap.contains_key(&key) {
            return -1;
        }
        let node = self.hashmap[&key].clone().unwrap();
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(n) = prev.clone() {
            n.borrow_mut().next = next.clone();
        } 
        if let Some(n) = next {
            n.borrow_mut().prev = prev.clone();
        }

        node.borrow_mut().next = self.head.clone();
        if let Some(n) = self.head.clone() {
            n.borrow_mut().prev = Some(node.clone());
        }
        self.head = Some(node.clone());

        node.clone().borrow().value.clone()
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.hashmap.contains_key(&key) {
            let node = self.hashmap.get(&key).unwrap().clone();
            self.remove_node(node.unwrap().clone());
        } else if self.hashmap.len() == self.capacity {
            self.remove_node(self.tail.clone().unwrap());
        }

        // insert new node
        let new_node = Rc::new(RefCell::new(ListNode {
            key,
            value,
            prev: None,
            next: None,
        }));

        new_node.borrow_mut().next = self.head.clone();
        self.head = Some(new_node.clone());
        if self.tail.is_none() {
            self.tail = Some(new_node.clone());
        }

        self.hashmap.insert(key, Some(new_node));
    }

    fn remove_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let node = node.clone();
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(n) = prev.clone() {
            n.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }
        if let Some(n) = next.clone() {
            n.borrow_mut().prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }
        self.hashmap.remove(&node.borrow().key);
    }
}
