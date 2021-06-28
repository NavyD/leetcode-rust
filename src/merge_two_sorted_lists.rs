/// 总结
///
/// rust在while时会out of moved，不能在`tail.next = l1|l2`使用l1,l2，
/// 可用while &l1,&l2引用，将l1,l2所有权转给tail，l1,l2 next所有权转给
/// 原l1,l2
///
/// ```ignore
/// while let (Some(head1), Some(head2)) = (l1, l2) {
/// }
/// ```
/// rust模式匹配可有不用再写(None, None) => None
///
/// ```ignore
/// (Some(l1), Some(l2)) => {},
/// (l1, None) => l1,
/// (None, l2) => l2,
/// ```
///
/// 20200909
///
/// rust链表还是要有对应的思维，一步步切断
pub mod solution_iterative {
    use crate::prelude::*;

    /// # 思路
    ///
    /// 较小值的节点添加到结果里，当一个节点被添加到结果里之后，将对应链表中的节点向后移一位。
    ///
    /// 参考：
    ///
    /// - [rust](https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/rust-by-tryfor_-23/)
    /// - [合并两个有序链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/he-bing-liang-ge-you-xu-lian-biao-by-leetcode-solu/)
    ///
    /// ### submissions
    ///
    /// date=20200901, mem=2.1, mem_beats=11.11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103581670/
    ///
    /// date=20200902, mem=1.9, mem_beats=96.30, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103965284/
    ///
    /// date=20200909, mem=2, mem_beats=91.18, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/106297679/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n + m)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn merge_two_lists(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let (mut l1, mut l2) = (l1, l2);
            let mut dummy = Box::new(ListNode::new(0));
            let mut tail = &mut dummy;
            // 不可用 while let ... = (l1, l2) 会导致tail.next使用l1,l2时所有权out of moved
            while let (Some(head1), Some(head2)) = (&l1, &l2) {
                if head1.val <= head2.val {
                    tail.next = l1;
                    tail = tail.next.as_mut().unwrap();
                    l1 = tail.next.take();
                } else {
                    tail.next = l2;
                    tail = tail.next.as_mut().unwrap();
                    l2 = tail.next.take();
                }
            }
            tail.next = if l1.is_some() { l1 } else { l2 };
            dummy.next
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                linkedlist![1, 1, 2, 3, 4, 4],
                Solution::merge_two_lists(linkedlist![1, 2, 4], linkedlist![1, 3, 4])
            );
        }
    }
}

pub mod solution_recursive {
    use crate::prelude::*;
    /// # 思路
    ///
    /// 两个链表头部值较小的一个节点与剩下元素的 merge 操作结果合并。
    ///
    /// 参考：
    ///
    /// - [Rust 递归合并双链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/rust-di-gui-he-bing-shuang-lian-biao-by-sniper_mar/)
    /// - [合并两个有序链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/he-bing-liang-ge-you-xu-lian-biao-by-leetcode-solu/)
    ///
    /// ### Submissions
    ///
    /// date=20200901, mem=2.1, mem_beats=11.11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103593396/
    ///
    /// date=20200902, mem=2.1, mem_beats=48.15, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/103958845/
    ///
    /// date=20200909, mem=2.1, mem_beats=32.35, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/106293054/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n + m)
    /// - 空间：O(n + m)：结束递归调用时 merge_two_lists 函数最多调用 n+m 次
    pub struct Solution;

    impl Solution {
        pub fn merge_two_lists(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (Some(mut l1), Some(mut l2)) => {
                    if l1.val <= l2.val {
                        l1.next = Self::merge_two_lists(l1.next, Some(l2));
                        Some(l1)
                    } else {
                        l2.next = Self::merge_two_lists(Some(l1), l2.next);
                        Some(l2)
                    }
                }
                (l1, None) => l1,
                (None, l2) => l2,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(
                linkedlist![1, 1, 2, 3, 4, 4],
                Solution::merge_two_lists(linkedlist![1, 2, 4], linkedlist![1, 3, 4])
            );
        }
    }
}
