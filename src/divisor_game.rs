//! Alice and Bob take turns playing a game, with Alice starting first.
//! 
//! Initially, there is a number N on the chalkboard.  On each player's turn, that player makes a move consisting of:
//! 
//! Choosing any x with 0 < x < N and N % x == 0.
//! Replacing the number N on the chalkboard with N - x.
//! Also, if a player cannot make a move, they lose the game.
//! 
//! Return True if and only if Alice wins the game, assuming both players play optimally.
//! 
//!  
//! 
//! Example 1:
//! 
//! Input: 2
//! Output: true
//! Explanation: Alice chooses 1, and Bob has no more moves.
//! Example 2:
//! 
//! Input: 3
//! Output: false
//! Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
//!  
//! 
//! Note:
//! 
//! 1 <= N <= 1000


pub mod solution_induction {
    /// # 思路
    /// 
    /// 归纳法
    /// 
    /// - 数字N如果是奇数，它的约数必然都是奇数；若为偶数，则其约数可奇可偶。
    /// - 无论N初始为多大的值，游戏最终只会进行到N=2时结束，那么谁轮到N=2时谁就会赢。
    /// - 因为爱丽丝先手，N初始若为偶数，爱丽丝则只需一直选1，使鲍勃一直面临N为奇数的情况，这样爱丽丝稳赢；
    /// N初始若为奇数，那么爱丽丝第一次选完之后N必为偶数，那么鲍勃只需一直选1就会稳赢。
    /// 
    /// 因此，奇则输，偶则赢
    /// 
    /// ## Submissions
    /// 
    /// date=20200629, mem=1.9, mem_beats=100, runtime=0, runtime=100, url=https://leetcode.com/submissions/detail/359636963/
    /// 
    /// author=coder233, references=https://leetcode-cn.com/problems/divisor-game/solution/qi-shi-shi-yi-dao-shu-xue-ti-by-coder233/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(1)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn divisor_game(n: i32) -> bool {
            (n & 1) == 0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            assert!(Solution::divisor_game(2));
            assert!(!Solution::divisor_game(3));
        }
    }
}

// TODO
#[allow(dead_code, unused_variables)]
pub mod solution_dp {
    /// # 思路
    /// 
    /// 设dp[i]为在给定数字i谁先开始的游戏结果，j满足`i%j==0`，
    /// 如果dp[i-j]=false，则会在使next=dp[i-j]=false，表示
    /// dp[i]=true
    pub struct Solution;

    impl Solution {
        pub fn divisor_game(n: i32) -> bool {
            false
        }
    }
}