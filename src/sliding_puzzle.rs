pub mod solution_bfs {

    /// # 思路
    ///
    /// 参考：
    ///
    /// * [滑动谜题](https://leetcode.cn/problems/sliding-puzzle/solution/hua-dong-mi-ti-by-leetcode-solution-q8dn/)
    ///
    /// ### Submissions
    ///
    /// date=20220608, mem=2.1, mem_beats=80, runtime=4, runtime_beats=40
    ///
    /// date=20220609, mem=2.2, mem_beats=20, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
            const ZERO: u8 = b'0';
            const END: [u8; 6] = [1 + ZERO, 2 + ZERO, 3 + ZERO, 4 + ZERO, 5 + ZERO, ZERO];

            let neighbors: [Vec<usize>; 6] = [
                vec![1, 3],
                vec![0, 2, 4],
                vec![1, 5],
                vec![0, 4],
                vec![1, 3, 5],
                vec![2, 4],
            ];

            let init = board
                .into_iter()
                .flatten()
                .map(|e| e as u8 + ZERO)
                .collect::<Vec<_>>();
            if init == END {
                return 0;
            }

            let swap_statuses = |status: &[u8]| {
                let i = status.iter().position(|e| *e == ZERO).unwrap();
                neighbors[i]
                    .iter()
                    .map(|j| {
                        let mut next = status.to_vec();
                        next.swap(i, *j);
                        next
                    })
                    .collect::<Vec<_>>()
            };

            let mut queue = std::collections::VecDeque::new();
            let mut visited = std::collections::HashSet::new();
            queue.push_back(init.clone());
            visited.insert(init);

            let mut steps = 0;
            while !queue.is_empty() {
                steps += 1;
                for _ in 0..queue.len() {
                    let status = queue.pop_front().unwrap();
                    for next in swap_statuses(&status) {
                        if !visited.contains(&next) {
                            if next == END {
                                return steps;
                            }
                            queue.push_back(next.clone());
                            visited.insert(next);
                        }
                    }
                }
            }

            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_bfs::Solution::sliding_puzzle);
    }

    fn test(f: impl Fn(Vec<Vec<i32>>) -> i32) {
        fn arr<const N: usize>(a: &[[i32; N]]) -> Vec<Vec<i32>> {
            a.iter().map(|e| e.to_vec()).collect()
        }

        assert_eq!(f(arr(&[[1, 2, 3], [4, 0, 5]])), 1);
        assert_eq!(f(arr(&[[1, 2, 3], [5, 4, 0]])), -1);
        assert_eq!(f(arr(&[[4, 1, 2], [5, 0, 3]])), 5);
    }
}
