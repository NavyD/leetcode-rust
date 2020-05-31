//! Given a blacklist B containing unique integers from [0, N), write a function to return a uniform random integer from [0, N) which is NOT in B.
//! 
//! Optimize it such that it minimizes the call to system’s Math.random().
//! 
//! Note:
//! 
//! 1 <= N <= 1000000000
//! 0 <= B.length < min(100000, N)
//! [0, N) does NOT include N. See interval notation.
//! Example 1:
//! 
//! Input: 
//! ["Solution","pick","pick","pick"]
//! [[1,[]],[],[],[]]
//! Output: [null,0,0,0]
//! Example 2:
//! 
//! Input: 
//! ["Solution","pick","pick","pick"]
//! [[2,[]],[],[],[]]
//! Output: [null,1,1,1]
//! Example 3:
//! 
//! Input: 
//! ["Solution","pick","pick","pick"]
//! [[3,[1]],[],[],[]]
//! Output: [null,0,0,2]
//! Example 4:
//! 
//! Input: 
//! ["Solution","pick","pick","pick"]
//! [[4,[2]],[],[],[]]
//! Output: [null,1,3,1]
//! Explanation of Input Syntax:
//! 
//! The input is two lists: the subroutines called and their arguments. Solution's constructor has two arguments, N and the blacklist B. pick has no arguments. Arguments are always wrapped with a list, even if there aren't any.

// leetcode支持使用`https://crates.io/crates/rand`
use rand::prelude::*;
use std::collections::HashMap;


struct Solution {
    map: HashMap<usize, usize>,
    whitelist_len: usize,
    rand: ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// leetcode的example不能直接用在test上
#[allow(dead_code)]
impl Solution {

    /// # 思路
    /// 
    /// 将白名单`0..N-blacklist.len()`中存在于blacklist中的数映射到`N-blacklist.len()..N`中不在
    /// 黑名单中的元素
    /// 
    /// ## 问题
    /// 
    /// - 如何映射
    /// 
    ///   将blacklist分为两部份，`left=0..N-blacklist.len(),right=N-blacklist.len()..N`，将left
    /// 作为全部的whitelist，可能存在blacklist的元素吋，对应的blacklist部份也存在着whitelist元素，
    /// 从blacklist right后面开始n--, 一旦right中元素不在blacklist中，则将blacklist中<whitelist.len的
    /// 元素映射到right中的n。whitelist+blacklist=N，不要担心越界
    /// 
    /// ```ignore
    /// blacklist=[3,5,8,9], N=10, whitelist.len=N-4=6
    /// left=[0..5] | right=[6..9]
    /// ```
    /// 
    /// ![](https://s3-lc-upload.s3.amazonaws.com/users/cafebaby/image_1530657902.png)
    /// 
    /// ## Submissions
    /// 
    /// date=20200531, mem=6.4, mem_beats=100, runtime=52, runtime_beats=100, url=https://leetcode.com/submissions/detail/346998931/
    /// 
    /// author=CAFEBABY, references=https://leetcode.com/problems/random-pick-with-blacklist/discuss/144624/Java-O(B)-O(1)-HashMap
    /// 
    /// author=leetcode, references=https://leetcode-cn.com/problems/random-pick-with-blacklist/solution/hei-ming-dan-zhong-de-sui-ji-shu-by-leetcode-2/
    /// 
    /// ## 复杂度
    /// 
    /// 设blacklist.len()为b
    /// 
    /// - 时间：在new()中为O(b)，虽然存在for中有while，但是while是线性时间只会到`N..whitelist_len`。pick()为O(1)
    /// 
    /// - 空间：O(b)
    /// 
    #[allow(non_snake_case)]
    fn new(N: i32, blacklist: Vec<i32>) -> Self {
        let mut n = N as usize;
        let (whitelist_len, mut map) = (n - blacklist.len() as usize, HashMap::new());
        // put blacklist
        for num in &blacklist {
            map.insert(*num as usize, 0);
        }

        for num in &blacklist {
            let num = *num as usize;
            // find blacklist element in whitelist
            if num < whitelist_len {
                // remap to right not in blacklist
                while map.contains_key(&(n - 1)) {
                    n -= 1;
                }
                map.insert(num, n - 1);
                n -= 1;
            }
        }
        Solution{ 
            map, 
            whitelist_len,
            rand: thread_rng(),
        }
    }
    
    fn pick(&mut self) -> i32 {
        let k = self.rand.gen_range(0, self.whitelist_len);
        let v = match self.map.get(&k) {
            Some(v) => *v,
            None => k
        };
        v as i32
    }
}
