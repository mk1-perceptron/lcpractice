/// LRU Cache
///
/// Design and implement a data structure for LEast Recently Used (LRU) cache. It shoudl support
/// the following operations: get and put.
/// get(key) - Get the value (will always be positive) of the key if the key exists in the cache,
/// otherwise return -1.
/// put(key, value) - Set or insert the value if the key is not already present. When the cache
/// reached its capacity, it should invalidate the least recently used item before inserting a new
/// item.
/// The cache is initialized with a positive capacity.
/// Follow up:
/// Could you do bother opertations in O(1) time complexity?
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    rc::Rc,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LRUCache {
    capacity: usize,
    head: NodeRef,
    tail: NodeRef,
    map: HashMap<i32, NodeRef>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        assert!(capacity > 0);
        let capacity = capacity as usize;
        let head = Rc::new(RefCell::new(ListNode::new(-1, -1)));
        let tail = Rc::new(RefCell::new(ListNode::new(-1, -1)));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity,
            head,
            tail,
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let node = self.remove_node(key).unwrap();
            let val = node.borrow().val;
            self.add_head(&node);

            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let node = self.remove_node(key).unwrap();
            node.borrow_mut().val = value;
            self.add_head(&node);
        } else {
            if self.is_full() {
                let last = self.remove_tail();
                if let Some(last_node) = last {
                    let key = last_node.borrow().key;
                    self.map.remove(&key);
                }
            }

            let new_node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.map.insert(key, Rc::clone(&new_node));
            self.add_head(&new_node);
        }
    }

    fn is_full(&self) -> bool {
        self.map.len() >= self.capacity
    }

    fn add_head(&self, node: &NodeRef) {
        if let Some(prev_first) = &self.head.borrow().next {
            prev_first.borrow_mut().prev = Some(Rc::clone(node));
            node.borrow_mut().next = Some(Rc::clone(prev_first));
        }
        node.borrow_mut().prev = Some(Rc::clone(&self.head));
        self.head.borrow_mut().next = Some(Rc::clone(node));
    }

    fn remove_node(&self, key: i32) -> Link {
        if let Some(node) = self.map.get(&key) {
            let prev = node.borrow().prev.clone();
            let next = node.borrow().next.clone();
            if let Some(p_node) = &prev {
                p_node.borrow_mut().next = next.clone();
            }
            if let Some(n_node) = &next {
                n_node.borrow_mut().prev = prev;
            }

            Some(Rc::clone(node))
        } else {
            None
        }
    }

    fn remove_tail(&self) -> Link {
        let mut new_last = Some(Rc::clone(&self.head));
        let mut removed = None;

        if let Some(prev_last) = &self.tail.borrow().prev {
            if let Some(node) = &prev_last.borrow().prev {
                node.borrow_mut().next = Some(Rc::clone(&self.tail));
                new_last = Some(Rc::clone(&node));
                removed = Some(Rc::clone(&prev_last));
            } else {
                self.head.borrow_mut().next = Some(Rc::clone(&self.tail));
            }
        }
        self.tail.borrow_mut().prev = new_last;

        removed
    }
}

impl Display for LRUCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        let mut node_opt = self.head.borrow().next.clone();

        output.push_str("list from head:\n");
        while let Some(node) = node_opt {
            output.push('\t');
            output.push_str(&node.borrow().to_string());
            output.push('\n');
            node_opt = node.borrow().next.clone();
        }

        node_opt = self.tail.borrow().prev.clone();
        output.push_str("\nlist from tail:\n");
        while let Some(node) = node_opt {
            output.push('\t');
            output.push_str(&node.borrow().to_string());
            output.push('\n');
            node_opt = node.borrow().next.clone();
        }

        output.push_str("\nmap:\n");
        for (k, v) in &self.map {
            output.push_str(&format!("\tkey: {}, value: {}\n", k, v.borrow().val));
        }

        write!(f, "{}", output)
    }
}

type Link = Option<NodeRef>;
type NodeRef = Rc<RefCell<ListNode>>;

#[derive(PartialEq, Eq, Debug, Clone)]
struct ListNode {
    pub key: i32,
    pub val: i32,
    pub prev: Link,
    pub next: Link,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        ListNode {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key: {}, val: {}", self.key, self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut cache = LRUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_size_one() {
        let mut cache = LRUCache::new(1);

        cache.put(2, 1);
        assert_eq!(cache.get(2), 1);
        cache.put(3, 2);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 2);
    }

    #[test]
    fn test_same_key() {
        let mut cache = LRUCache::new(2);

        assert_eq!(cache.get(2), -1);
        cache.put(2, 6);
        assert_eq!(cache.get(1), -1);
        cache.put(1, 5);
        cache.put(1, 2);
        assert_eq!(cache.get(1), 2);
        assert_eq!(cache.get(2), 6);
    }
}
