#![allow(dead_code)]

/// 总结
/// 
/// 这里有个用链表实现的stack：[Clean 6ms Java solution](https://leetcode.com/problems/min-stack/discuss/49010/Clean-6ms-Java-solution)
/// 相对于用std库，每个Node保存了当前的min值。
/// 
/// 20200905
/// 
/// 注意one stack时push的顺序，不要在push调用了self.push递归了。。。。
/// push时先old min, 再push new x，当min==x时也是min值入栈
pub mod solution_two_stack {
    /// # 思路
    /// 
    /// 如何保留最小值
    /// 
    /// 一个栈去保存正常的入栈出栈的值，另一个栈去存最小值，也就是用栈顶保存当前所有元素的最小值，
    /// 对min_stack只有当：
    /// 
    /// - 新加入的元素如果小于等于栈顶元素，新元素入栈
    /// - 出栈元素等于栈顶元素，栈顶元素出栈
    /// 
    /// 可保证min_stack与data的一致性
    /// 
    /// ```ignore
    /// data = 1, min_stack = 1
    /// push 3 => data = 1,3, min_stack = 1
    /// pop => data = 1, min_stack = 1
    /// pop => data = None, min_stack = None
    /// ```
    /// 
    /// 参考：
    /// 
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/min-stack/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-38/)
    /// - [Rust 单栈、双栈实现](https://leetcode-cn.com/problems/min-stack/solution/rust-dan-zhan-shuang-zhan-shi-xian-by-forsworns/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200904, mem=6.5, mem_beats=9.09, runtime=8, runtime_beats=85.37, url=https://leetcode-cn.com/submissions/detail/104658333/
    /// 
    /// date=20200905, mem=6.5, mem_beats=9.09, runtime=8, runtime_beats=85.37, url=https://leetcode-cn.com/submissions/detail/104911435/
    /// 
    /// ### 复杂度
    /// 
    /// - 空间：O(n)
    /// - 时间：O(1)
    struct MinStack {
        data: Vec<i32>,
        min_stack: Vec<i32>,
    }
    
    
    /** 
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MinStack {
    
        /** initialize your data structure here. */
        fn new() -> Self {
            Self { 
                data: vec![],
                min_stack: vec![],
            }
        }
        
        fn push(&mut self, x: i32) {
            self.data.push(x);
            if self.min_stack.is_empty() || self.min_stack.last().unwrap() >= &x {
                self.min_stack.push(x);
            }
        }
        
        fn pop(&mut self) {
            if &self.data.pop().unwrap() == self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        }
        
        fn top(&self) -> i32 {
            *self.data.last().unwrap()
        }
        
        fn get_min(&self) -> i32 {
            *self.min_stack.last().unwrap()
        }
    }
}

pub mod solution_one_stack {
    /// # 思路
    /// 
    /// 如何保留最小值？
    /// 
    /// 在之前的最小值入栈，当前更小的值再入栈即可。当这个最小值要出栈的时候，下一个值便是之前的最小值了。
    /// 
    /// - push时将old min值入栈在当前值min前
    /// - pop时遇到min值，再pop就是old min值
    /// 
    /// 参考：
    /// 
    /// - [Java accepted solution using one stack](https://leetcode.com/problems/min-stack/discuss/49014/Java-accepted-solution-using-one-stack)
    /// - [Rust 单栈、双栈实现](https://leetcode-cn.com/problems/min-stack/solution/rust-dan-zhan-shuang-zhan-shi-xian-by-forsworns/)
    /// 
    /// ### submissions
    /// 
    /// date=20200904, mem=6.4, mem_beats=9.09, runtime=8, runtime_beats=85.37, url=https://leetcode-cn.com/submissions/detail/104663220/
    /// 
    /// date=20200905, mem=6.4, mem_beats=9.09, runtime=4, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/104904830/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(1)
    /// - 空间：O(n)
    struct MinStack {
        data: Vec<i32>,
        min: i32,
    }
    
    
    /** 
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MinStack {
    
        /** initialize your data structure here. */
        fn new() -> Self {
            Self { 
                data: Vec::new(),
                min: std::i32::MAX,
            }
        }
        
        fn push(&mut self, x: i32) {
            // push old val and update min
            if x <= self.min {
                self.data.push(self.min);
                self.min = x;
            }
            self.data.push(x);
        }
        
        fn pop(&mut self) {
            if self.data.pop().unwrap() == self.min {
                self.min = self.data.pop().unwrap();
            }
        }
        
        fn top(&self) -> i32 {
            *self.data.last().unwrap()
        }
        
        fn get_min(&self) -> i32 {
            self.min
        }
    }
}
