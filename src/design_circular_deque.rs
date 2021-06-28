#![allow(dead_code)]

/// 总结
///
/// 实现简单的deque还是不难的，java std有个更复杂的无界ArrayDeque，
/// 这里有个linkedlist实现：
/// [Java doubly LinkedList  solution, very straightforward](https://leetcode.com/problems/design-circular-deque/discuss/149371/Java-doubly-LinkedList-solution-very-straightforward)
///
/// 20200926
///
/// 注意不能写成：`data: vec![-1, k + 1],`
pub mod solution_array {
    /// # 思路
    ///
    /// 定义两个数组下标head, tail
    ///
    /// 为了避免“队列为空”和“队列为满”的判别条件冲突
    ///
    /// - 判别队列为空的条件是：front == rear;；
    /// - 判别队列为满的条件是：(rear + 1) % capacity == front;。可以这样理解，当 rear 循环到数组的前面，要从后面追上 front，还差一格的时候，判定队列为满。
    ///
    /// 处理数组下标可能越界的情况
    ///
    /// - 指针后移的时候，索引 + 1，要取模；
    /// - 指针前移的时候，为了循环到数组的末尾，需要先加上数组的长度，然后再对数组长度取模。
    ///
    /// 参考：
    ///
    /// - [数组实现的循环双端队列](https://leetcode-cn.com/problems/design-circular-deque/solution/shu-zu-shi-xian-de-xun-huan-shuang-duan-dui-lie-by/)
    ///
    /// ### 注意
    ///
    /// data.length=k+1保证`data[tail]=null`
    ///
    /// 在指针移动运算时，必须保证不越界 `i < 0 || i >= data.length`，则有
    ///
    /// - (this.head + 1) % this.data.length;
    /// - (this.head - 1 + this.data.length) % this.data.length;
    ///
    /// tail也是一样的道理
    ///
    /// ```java
    /// @Submission(date = "20200915", memory = 40, memoryBeatRate = 91.58, runtime = 6, runtimeBeatRate = 99.93, url = "https://leetcode-cn.com/submissions/detail/108109125/")
    /// class MyCircularDeque {
    ///     private final int[] data;
    ///     private int head;
    ///     private int tail;
    ///
    ///     /** Initialize your data structure here. Set the size of the deque to be k. */
    ///     public MyCircularDeque(int k) {
    ///         this.data = new int[k + 1];
    ///         this.head = 0;
    ///         this.tail = 0;
    ///     }
    ///
    ///     /** Adds an item at the front of Deque. Return true if the operation is successful. */
    ///     public boolean insertFront(int value) {
    ///         if (this.isFull()) {
    ///             return false;
    ///         }
    ///
    ///         this.head = (this.head - 1 + this.data.length) % this.data.length;
    ///         this.data[this.head] = value;
    ///         return true;
    ///     }
    ///
    ///     /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    ///     public boolean insertLast(int value) {
    ///         if (this.isFull()) {
    ///             return false;
    ///         }
    ///         this.data[this.tail] = value;
    ///         this.tail = (this.tail + 1) % data.length;
    ///         return true;
    ///     }
    ///
    ///     /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    ///     public boolean deleteFront() {
    ///         if (this.isEmpty()) {
    ///             return false;
    ///         }
    ///         this.data[this.head] = -1;
    ///         this.head = (this.head + 1) % this.data.length;
    ///         return true;
    ///     }
    ///
    ///     /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    ///     public boolean deleteLast() {
    ///         if (this.isEmpty()) {
    ///             return false;
    ///         }
    ///         this.tail = (this.tail - 1 + this.data.length) % this.data.length;
    ///         this.data[this.tail] = -1;
    ///         return true;
    ///     }
    ///
    ///     /** Get the front item from the deque. */
    ///     public int getFront() {
    ///         if (this.isEmpty()) {
    ///             return -1;
    ///         }
    ///         return this.data[this.head];
    ///     }
    ///
    ///     /** Get the last item from the deque. */
    ///     public int getRear() {
    ///         if (this.isEmpty()) {
    ///             return -1;
    ///         }
    ///         return this.data[(this.tail - 1 + this.data.length) % this.data.length];
    ///     }
    ///
    ///     /** Checks whether the circular deque is empty or not. */
    ///     public boolean isEmpty() {
    ///         return this.head == this.tail;
    ///     }
    ///
    ///     /** Checks whether the circular deque is full or not. */
    ///     public boolean isFull() {
    ///         return this.head == (this.tail + 1) % this.data.length;
    ///     }
    ///
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20200915, mem=2.3, mem_beats=100, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/108122161/
    ///
    /// date=20200916, mem=2.4, mem_beats=100, runtime=12, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/108482454/
    ///
    /// date=20200926, mem=2.3, mem_beats=33.33, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/111479817/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(1)。所有操作都是O(1)的
    /// - 空间：O(n)
    pub struct MyCircularDeque {
        data: Vec<i32>,
        head: usize,
        tail: usize,
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
            Self {
                data: vec![-1; k as usize + 1],
                head: 0,
                tail: 0,
            }
        }

        /** Adds an item at the front of Deque. Return true if the operation is successful. */
        fn insert_front(&mut self, value: i32) -> bool {
            if self.is_full() {
                return false;
            }
            self.head = (self.head + self.data.len() - 1) % self.data.len();
            self.data[self.head] = value;
            true
        }

        /** Adds an item at the rear of Deque. Return true if the operation is successful. */
        fn insert_last(&mut self, value: i32) -> bool {
            if self.is_full() {
                return false;
            }
            self.data[self.tail] = value;
            self.tail = (self.tail + 1) % self.data.len();
            true
        }

        /** Deletes an item from the front of Deque. Return true if the operation is successful. */
        fn delete_front(&mut self) -> bool {
            if self.is_empty() {
                return false;
            }
            self.data[self.head] = -1;
            self.head = (self.head + 1) % self.data.len();
            true
        }

        /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
        fn delete_last(&mut self) -> bool {
            if self.is_empty() {
                return false;
            }
            self.tail = (self.tail + self.data.len() - 1) % self.data.len();
            self.data[self.tail] = -1;
            true
        }

        /** Get the front item from the deque. */
        fn get_front(&self) -> i32 {
            if self.is_empty() {
                -1
            } else {
                self.data[self.head]
            }
        }

        /** Get the last item from the deque. */
        fn get_rear(&self) -> i32 {
            if self.is_empty() {
                -1
            } else {
                self.data[(self.tail + self.data.len() - 1) % self.data.len()]
            }
        }

        /** Checks whether the circular deque is empty or not. */
        fn is_empty(&self) -> bool {
            self.tail == self.head
        }

        /** Checks whether the circular deque is full or not. */
        fn is_full(&self) -> bool {
            (self.tail + 1) % self.data.len() == self.head
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
