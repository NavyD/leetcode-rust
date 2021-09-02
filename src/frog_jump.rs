pub mod solution_backtracking {
    /// # 思路
    ///
    /// ### Submissions
    ///
    /// date=20210902, mem=5.8, mem_beats=66, runtime=8, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn can_cross(stones: Vec<i32>) -> bool {
            fn backtrack(stones: &[i32], k: i32, cache: &mut [Vec<Option<bool>>]) -> bool {
                if stones.len() == 1 {
                    return true;
                }
                if let Some(v) = cache[0][k as usize] {
                    return v;
                }
                for i in -1..=1 {
                    let next_k = k + i;
                    if next_k != 0
                        && stones
                            .binary_search(&(stones[0] + next_k))
                            .map(|next| backtrack(&stones[next..], next_k, &mut cache[next..]))
                            .unwrap_or(false)
                    {
                        cache[0][k as usize] = Some(true);
                        return true;
                    }
                }
                cache[0][k as usize] = Some(false);
                false
            }

            if stones[0] + 1 < stones[1] {
                false
            } else {
                backtrack(
                    &stones[1..],
                    1,
                    &mut vec![vec![None; stones.len()]; stones.len()],
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_backtracking::Solution::can_cross);
    }

    fn test<F: Fn(Vec<i32>) -> bool>(f: F) {
        assert!(f(vec![0, 1, 3, 5, 6, 8, 12, 17]));
        assert!(!f(vec![0, 1, 2, 3, 4, 8, 9, 11]));
    }
}
