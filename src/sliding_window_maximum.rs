/// 总结
///
/// 开始想用堆priority queue，rust BinaryHeap，但是这个题的意思是
/// 要在一个k范围找最值，同时移动出k外元素，priority不能直接移出k外
/// 的元素
///
/// 20200909
///
/// solution_monotonic_queue要注意先delete再push后add res，与下标对
/// 应起来，删除时只有当`last + k - 1 < cur_idx`才行
///
/// 20200917
///
/// 这是第3次写出的，很明显没记得了，只记得push要移动，没有考虑如何移除与
/// 获取res，尝试一次`res.push(queue.pop_front().unwrap())`解决。
/// 要注意下标的问题。。。。
///
/// ```ignore
/// while let Some(back) = queue.back() {
///     if back >= num {
///         break;
///     }
///     queue.pop_back();
/// }
/// queue.push(num);
/// count += 1;
/// if count >= k {
/// }
/// ```
///
#[allow(dead_code)]
pub mod my_solution {
    struct Solution;

    impl Solution {
        pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
            let k = k as usize;
            let mut queue = std::collections::binary_heap::BinaryHeap::with_capacity(k);
            let mut res = Vec::with_capacity(nums.len());
            for num in nums {
                if queue.len() >= k {
                    res.push(queue.pop().unwrap());
                } else {
                    queue.push(num);
                }
            }
            res
        }
    }
}

pub mod solution_violent {
    /// # 思路
    ///
    /// 最简单直接的方法是遍历每个滑动窗口，找到每个窗口的最大值。
    /// 一共有 N - k + 1 个滑动窗口，每个有 k 个元素，
    ///
    /// 时间复杂度为 O(n*k)，超出时间限制
    pub struct Solution;

    impl Solution {
        pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
            let k = k as usize;
            let mut res = vec![];
            for i in 0..nums.len() - k + 1 {
                let mut max = std::i32::MIN;
                for j in i..i + k {
                    max = max.max(nums[j]);
                }
                res.push(max);
            }
            res
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert_eq!(
                vec![3, 3, 5, 5, 6, 7],
                Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
            );
        }
    }
}

pub mod solution_monotonic_queue {
    /// # 思路
    ///
    /// 每个窗口前进的时候，要添加一个数同时减少一个数
    ///
    /// 在一堆数字中，已知最值，如果给这堆数添加一个数，那么比较一下就可以很快算出最值；
    /// 但如果减少一个数，就不一定能很快得到最值了，而要遍历所有数重新找最值。
    ///
    /// - 单调队列的 push 方法依然在队尾添加元素，但是要把前面比新元素小的元素都删掉：
    /// - pop() 在队头删除元素 n，移除k范围外
    ///
    /// 参考：
    ///
    /// - [Java O(n) solution using deque with explanation](https://leetcode.com/problems/sliding-window-maximum/discuss/65884/Java-O(n)-solution-using-deque-with-explanation)
    /// - [单调队列解题详解](https://leetcode-cn.com/problems/sliding-window-maximum/solution/dan-diao-dui-lie-by-labuladong/)
    ///
    /// ### Submissions
    ///
    /// date=20200908, mem=2.9, mem_beats=60, runtime=16, runtime_beats=72.97, url=https://leetcode-cn.com/submissions/detail/105862456/
    ///
    /// date=20200909, mem=2.9, mem_beats=60, runtime=16, runtime_beats=72.97, url=https://leetcode-cn.com/submissions/detail/106286541/
    ///
    /// date=20200917, mem=2.7, mem_beats=87.50, runtime=20, runtime_beats=30, url=https://leetcode-cn.com/submissions/detail/108856621/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    ///
    pub struct Solution;

    impl Solution {
        pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
            let k = k as usize;
            let mut res = Vec::with_capacity(nums.len() - k + 1);
            let mut queue = std::collections::VecDeque::with_capacity(k + 1);
            for (i, num) in nums.iter().enumerate() {
                // 删除out of rang k 元素
                if let Some(last) = queue.front() {
                    // 下标last在k范围外  不可直接删pop front，last可能是push前面压过来的
                    if *last + k - 1 < i {
                        queue.pop_front();
                    }
                }
                // push 删除比num小的元素
                while let Some(cur) = queue.back() {
                    if nums[*cur] >= *num {
                        break;
                    }
                    queue.pop_back();
                }
                queue.push_back(i);
                // queue k个范围时开始
                if i >= k - 1 {
                    res.push(nums[*queue.front().unwrap()]);
                }
            }
            res
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert_eq!(
                vec![3, 3, 5, 5, 6, 7],
                Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
            );
            assert_eq!(
                vec![10, 10, 9, 2],
                Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5)
            );
            assert_eq!(vec![1, -1], Solution::max_sliding_window(vec![1, -1], 1));
        }
    }
}
