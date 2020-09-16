pub mod solution_line {
    /// # 思路
    /// 
    /// 求第 h 层的水，遍历每个位置，如果当前的高度小于 h，并且两边有高度大于等于 h 的，
    /// 说明这个地方一定有水，水就可以加 1
    /// 
    /// 从左到右遍历墙的高度，遇到高度height > 当前层高度为h的水，表示可能在高度height形成h高的水，
    /// 可以尝试更新形成h高水的数量
    /// 
    /// 当height<h时存在一个容量为1*1的水，更新count+=1。当height>=h表示完成之前到当前的count数量，
    /// count更新到res答案中，并count=0
    /// 
    /// 由于h表示水的高度在向最高max_height迭代，每次height<h更新是表示这是一个1*1，不会重复计算
    /// 
    /// 参考：
    /// 
    /// - [详细通俗的思路分析，多解法](https://leetcode-cn.com/problems/trapping-rain-water/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-w-8/)
    /// 
    /// ### Submissions
    /// 
    /// date=20200916, mem=2.1, mem_beats=11.11, runtime=560, runtime_beats=560, url=https://leetcode-cn.com/submissions/detail/108470415/
    /// 
    /// ### 复杂度
    /// 
    /// - 时间：O(n^2)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn trap(height: Vec<i32>) -> i32 {
            let mut res = 0;
            // find max height
            let mut max_height = 0;
            for h in &height {
                if *h > max_height {
                    max_height = *h;
                }
            }
            // for h in max height
            for h in 1..=max_height {
                let mut is_started = false;
                let mut count = 0;
                for i in 0..height.len() {
                    // update count line if is started and h > height
                    if is_started && height[i] < h {
                        count += 1;
                    }
                    // update sum if height >= h
                    if height[i] >= h {
                        is_started = true;
                        res += count;
                        count = 0;
                    }
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
            assert_eq!(6, Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
        }
    }
}