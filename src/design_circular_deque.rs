#![allow(dead_code)]

pub mod solution_array {
    /** 
["MyCircularDeque","insertFront","deleteLast","getFront","insertLast","insertFront","getFront","getRear","getFront","getFront","getRear","insertLast"]
[[2],[7],[],[],[5],[0],[],[],[],[],[],[0]]

Line 92, Char 60: called `Option::unwrap()` on a `None` value (solution.rs)
     * **/

    /// 参考：
    /// 
    /// - [数组实现的循环双端队列](https://leetcode-cn.com/problems/design-circular-deque/solution/shu-zu-shi-xian-de-xun-huan-shuang-duan-dui-lie-by/)
    pub struct MyCircularDeque {
        data: Vec<Option<i32>>,
        head: usize,
        tail: usize,
        capacity: usize,
    }
    /**
     * Your MyCircularDeque object will be instantiated and called as such:
     * let obj = MyCircularDeque::new(k);
     * let ret_1: bool = obj.insert_front(value);
     * let ret_2: bool = obj.insert_last(value);
     * let ret_3: bool = obj.delete_front();
     * let ret_4: bool = obj.delete_last();
     * let ret_5: i32 = obj.get_front();
     * let ret_6: i32 = obj.get_rear();
     * let ret_7: bool = obj.is_empty();
     * let ret_8: bool = obj.is_full();
     */

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MyCircularDeque {
        /** Initialize your data structure here. Set the size of the deque to be k. */
        fn new(k: i32) -> Self {
            let capacity = k as usize + 1;
            Self {
                data: vec![None; capacity],
                head: 0,
                tail: 0,
                capacity,
            }
        }

        /** Adds an item at the front of Deque. Return true if the operation is successful. */
        fn insert_front(&mut self, value: i32) -> bool {
            if self.is_full() {
                return false;
            }
            // 使head走到后面
            self.head = (self.head + self.capacity - 1) % self.capacity;
            self.data[self.head] = Some(value);
            true
        }

        /** Adds an item at the rear of Deque. Return true if the operation is successful. */
        fn insert_last(&mut self, value: i32) -> bool {
            if self.is_full() {
                return false;
            }
            self.data[self.tail] = Some(value);
            self.tail = (self.tail + 1) % self.capacity;
            true
        }

        /** Deletes an item from the front of Deque. Return true if the operation is successful. */
        fn delete_front(&mut self) -> bool {
            if self.is_empty() {
                return false;
            }
            self.data[self.head] = None;
            self.head = (self.head + 1) % self.capacity;
            true
        }

        /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
        fn delete_last(&mut self) -> bool {
            if self.is_empty() {
                return false;
            }
            self.data[(self.tail - 1) % self.capacity].take();
            // 可退回到array尾部
            self.tail = (self.tail + self.capacity - 1) % self.capacity;
            true
        }

        /** Get the front item from the deque. */
        fn get_front(&self) -> i32 {
            if self.is_empty() {
                -1
            } else {
                self.data[self.head].unwrap()
            }
        }

        /** Get the last item from the deque. */
        fn get_rear(&self) -> i32 {
            if self.is_empty() {
                -1
            } else {
                self.data[(self.tail - 1) % self.capacity].unwrap()
            }
        }

        /** Checks whether the circular deque is empty or not. */
        fn is_empty(&self) -> bool {
            self.tail == self.head
        }

        /** Checks whether the circular deque is full or not. */
        fn is_full(&self) -> bool {
            (self.tail + 1) % self.capacity == self.head
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let mut deque = MyCircularDeque::new(3); // 设置容量大小为3
            assert!(deque.insert_last(1)); 
            assert!(deque.insert_last(2)); 
            assert!(deque.insert_front(3)); 
            assert!(!deque.insert_front(4));

            assert_eq!(2, deque.get_rear());
            assert!(deque.is_full());
            assert!(deque.delete_last());
            assert!(deque.insert_front(4));
            assert_eq!(4, deque.get_front());
        }
    }
}
