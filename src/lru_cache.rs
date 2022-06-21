// TODO
pub mod solution {
    use super::LRUCache;

    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    #[derive(Debug, Default)]
    struct Node {
        key: i32,
        val: i32,
        prev: Option<Rc<RefCell<Node>>>,
        next: Option<Rc<RefCell<Node>>>,
    }

    impl From<(i32, i32)> for Node {
        fn from(v: (i32, i32)) -> Self {
            Node {
                key: v.0,
                val: v.1,
                prev: None,
                next: None,
            }
        }
    }

    pub struct Solution {
        head: Rc<RefCell<Node>>,
        tail: Rc<RefCell<Node>>,
        size: usize,
        capacity: usize,
        cache: HashMap<i32, Rc<RefCell<Node>>>,
    }

    impl Solution {
        fn move_to_head(&self, node: Rc<RefCell<Node>>) {
            self.remove_node(node.clone());
            self.add_to_head(node);
        }

        fn add_to_head(&self, node: Rc<RefCell<Node>>) {
            node.borrow_mut().prev = Some(self.head.clone());
            node.borrow_mut().next = self.head.borrow().next.clone();

            self.head.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
            self.head.borrow_mut().next = Some(node);
        }

        fn remove_node(&self, node: Rc<RefCell<Node>>) {
            // let node = node.borrow_mut();
            // node.borrow().prev.as_ref().unwrap().borrow_mut().next = node.borrow().next.clone();
            // node.borrow().next.as_ref().unwrap().borrow_mut().prev = node.borrow().prev.clone();
            let prev = node.borrow_mut().prev.take().unwrap();
            let next = node.borrow_mut().next.take().unwrap();
            prev.borrow_mut().next = Some(next.clone());
            next.borrow_mut().prev = Some(prev);
        }

        fn remove_tail(&self) -> Option<Rc<RefCell<Node>>> {
            self.tail.borrow().prev.clone().map(|prev| {
                self.remove_node(prev.clone());
                prev
            })
        }
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl LRUCache for Solution {
        fn new(capacity: i32) -> Self {
            let head = Rc::new(RefCell::new(Node::default()));
            let tail = Rc::new(RefCell::new(Node::default()));
            head.borrow_mut().next = Some(tail.clone());
            tail.borrow_mut().prev = Some(head.clone());

            Self {
                cache: std::collections::HashMap::new(),
                capacity: capacity as usize,
                size: 0,
                head,
                tail,
            }
        }

        fn get(&self, key: i32) -> i32 {
            self.cache
                .get(&key)
                .cloned()
                .map(|node| {
                    let v = node.borrow().val;
                    self.move_to_head(node);
                    v
                })
                .unwrap_or(-1)
        }

        fn put(&mut self, key: i32, value: i32) {
            if let Some(node) = self.cache.get(&key).cloned() {
                node.borrow_mut().val = value;
                self.move_to_head(node)
            } else {
                let new_node = Rc::new(RefCell::new((key, value).into()));
                self.cache.insert(key, new_node.clone());
                self.add_to_head(new_node);
                self.size += 1;

                if self.size > self.capacity {
                    let node = self.remove_tail().unwrap();
                    self.cache.remove(&node.borrow().key);
                    self.size -= 1;
                }
            }
        }
    }
}

trait LRUCache: Sized {
    fn new(capacity: i32) -> Self;

    fn get(&self, key: i32) -> i32;

    fn put(&mut self, key: i32, value: i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test::<solution::Solution>();
    }

    fn test<S: LRUCache>() {
        let mut lru_cache = S::new(2);
        lru_cache.put(1, 1); // 缓存是 {1=1}
        lru_cache.put(2, 2); // 缓存是 {1=1, 2=2}
        assert_eq!(lru_cache.get(1), 1); // 返回 1
        lru_cache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
        assert_eq!(lru_cache.get(2), -1); // 返回 -1 (未找到)
        lru_cache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
        assert_eq!(lru_cache.get(1), -1); // 返回 -1 (未找到)
        assert_eq!(lru_cache.get(3), 3); // 返回 3
        assert_eq!(lru_cache.get(4), 4); // 返回 4
    }
}
