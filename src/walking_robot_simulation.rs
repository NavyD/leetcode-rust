pub mod solution_greedy {
    /// # 思路
    /// 
    /// 机器人如何行走
    /// 
    /// 机器人行走command步可以分解为在4个方向走，走i步时可能遇到阻碍，等下个命令换向走
    /// 
    /// 如何转向？
    /// 
    /// 要在当前方向上右左转90c，在坐标上看，只要定义4个方向的偏移，右转走到下一步就表示
    /// 移动到当前方向的下个偏移就行。左转要到上个偏移
    /// 
    /// ```ignore
    /// let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    /// ```
    /// 
    /// 如开始坐标向上方向为(0,0)，右转下一步走(1,0)=(1,0)，再左转(1,0)+3%4=(0,1)+(1,0)=(1,1)
    /// 
    /// 参考：
    /// 
    /// - [模拟行走机器人](https://leetcode-cn.com/problems/walking-robot-simulation/solution/mo-ni-xing-zou-ji-qi-ren-by-intgrp/)
    /// 
    /// // 还有一个更精简的方法：由于只有左右两个方向，定义初始方向向上(x,y)=(0,1)，在当前方向上偏移：
    /// // 右转(x,y)=(y,x)，左转(x,y)=(-y,x)，
    /// 
    /// 参考：[12ms]
    /// 
    /// ```ignore
    /// pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    ///     let mut ob_set = obstacles.iter().map(|v| (v[0], v[1])).collect::<HashSet<_>>();
    ///     let mut x = 0;
    ///     let mut y = 0;
    ///     let mut dir = (0,1);
    ///     let  mut max_dis = 0;
    ///     for com in commands{
    ///         if com == -1{
    ///             dir = (dir.1, -dir.0);
    ///         }
    ///         if com == -2{
    ///             dir = (-dir.1, dir.0);
    ///         } else{
    ///             for i in 0..com{
    ///                 if !ob_set.contains(&(x + dir.0,y + dir.1)){
    ///                     x += dir.0;
    ///                     y += dir.1;
    ///                 }
    ///             }
    ///             max_dis = std::cmp::max( max_dis, x * x + y * y);
    ///         }
    ///     }
    ///     max_dis
    /// }
    /// ```
    /// 
    /// ### Submissions
    /// 
    /// date=20210113, mem=2.9, mem_beats=10, runtime=16, runtime_beats=67, url=https://leetcode-cn.com/submissions/detail/138042161/
    pub struct Solution;

    impl Solution {
        pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
            // 下一步可走的方向(x,y)：上 右 下 左；右转: +1 % 4, 左转：+3 % 4
            let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let obstacles = obstacles
                .into_iter()
                .map(|a| (a[0], a[1]))
                .collect::<std::collections::HashSet<_>>();
            let mut max_distance = 0;
            let (mut x, mut y) = (0, 0);
            // 初始方向 向上 北方
            let mut dir_idx = 0;
            for mut command in commands {
                match command {
                    // 右转
                    -1 => dir_idx = (dir_idx + 1) % 4,
                    // 左转
                    -2 => dir_idx = (dir_idx + 3) % 4,
                    _ => {
                        while command > 0 {
                            let (x_offset, y_offset) = directions[dir_idx];
                            // 走下一步
                            let (next_x, next_y) = (x + x_offset, y + y_offset);
                            // 遇到阻碍
                            if obstacles.contains(&(next_x, next_y)) {
                                break;
                            }
                            x = next_x;
                            y = next_y;
                            // 最大欧式距离
                            max_distance = max_distance.max(x * x + y * y);

                            command -= 1;
                        }
                    }
                }
            }
            max_distance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::robot_sim);
    }

    fn test<F: Fn(Vec<i32>, Vec<Vec<i32>>) -> i32>(func: F) {
        assert_eq!(func(vec![4, -1, 3], vec![]), 25);
        assert_eq!(func(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
    }
}
