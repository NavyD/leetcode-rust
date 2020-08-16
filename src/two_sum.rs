/// 总结
/// 
/// 只想到暴力法
pub mod solution_force {

    /// ### Submissions
    /// 
    /// date=20200816, mem=2.2, mem_beats=66.47, runtime=36, runtime_beats=21.42, url=https://leetcode-cn.com/submissions/detail/98495796/
    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut indices = vec![];
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if target == nums[i] + nums[j] {
                        indices.push(i as i32);
                        indices.push(j as i32);
                    }
                }
            }
            indices
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert!(
                (Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1])
                    || (Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![1, 0])
            )
        }
    }
}

pub mod solution_hash {

    /// # 思路
    ///
    /// 在进行迭代并将元素插入到表中的同时，我们还会回过头来检查表中是否已经存在当前元素所对应的目标元素。
    /// 如果它存在，那我们已经找到了对应解，并立即将其返回。
    ///
    /// 如果当前数没有进入hash map，target-时会无法获取，num存入后下次用到target-complement时可找到
    ///
    /// 参考
    /// 
    /// - [Accepted Java O(n) Solution](https://leetcode.com/problems/two-sum/discuss/3/Accepted-Java-O(n)-Solution)
    /// 
    /// ### Submissions
    ///
    /// date=20200816, mem=2.2, mem_beats=72.46, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/98495478/
    ///
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut num_indices = std::collections::HashMap::with_capacity(nums.len());
            let mut indices = vec![];
            for i in 0..nums.len() {
                let complement = target - nums[i];
                if let Some(complement_idx) = num_indices.get(&complement) {
                    // 不用再判断 complement_idx一定小于i
                    //     if i != *complement_idx {
                    indices.push(i as i32);
                    indices.push(*complement_idx as i32);
                    break;
                }
                num_indices.insert(nums[i], i);
            }
            indices
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert!(
                (Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1])
                    || (Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![1, 0])
            )
        }
    }
}
