pub mod solution_loop {
    /// # 思路
    ///
    /// 每次把 res 左移，把 n 的二进制末尾数字，拼接到结果 res 的末尾。然后把 n 右移。
    ///
    /// 参考：
    ///
    /// * [【负雪明烛】「循环」与「分治」解法](https://leetcode.cn/problems/reverse-bits/solution/fu-xue-ming-zhu-xun-huan-yu-fen-zhi-jie-hoakf/)
    ///
    /// ### Submissions
    ///
    /// date=20220612, mem=2.3, mem_beats=6, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn reverse_bits(mut x: u32) -> u32 {
            let mut rev = 0;
            for _ in 0..32 {
                rev = (rev << 1) | (x & 1);
                x >>= 1;
            }
            rev
        }
    }
}

pub mod solution_bit {
    /// # 思路
    ///
    /// ![][a]
    ///
    /// ```
    /// pub fn reverse_bits(mut n: u32) -> u32 {
    ///     n = (n >> 16) | (n << 16); //低16位与高16位交换
    ///     n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8); //每16位中低8位和高8位交换; 1111是f
    ///     n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4); //每8位中低4位和高4位交换;
    ///     n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2); //每4位中低2位和高2位交换; 1100是c,0011是3
    ///     n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1); //每2位中低1位和高1位交换; 1010是a,0101是5
    ///     return n;
    /// }
    /// assert_eq!(reverse_bits(0b11111111111111111111111111111101), 0b10111111111111111111111111111111);
    /// ```
    ///
    /// 参考：
    ///
    /// * [【负雪明烛】「循环」与「分治」解法](https://leetcode.cn/problems/reverse-bits/solution/fu-xue-ming-zhu-xun-huan-yu-fen-zhi-jie-hoakf/)
    /// * [颠倒二进制位](https://leetcode.cn/problems/reverse-bits/solution/dian-dao-er-jin-zhi-wei-by-leetcode-solu-yhxz/)
    ///
    /// ### Submissions
    ///
    /// date=20220612, mem=2.1, mem_beats=13, runtime=0, runtime_beats=100
    #[embed_doc_image::embed_doc_image("a", "docs/images/2022-06-12-09-32-55.png")]
    pub struct Solution;

    impl Solution {
        pub fn reverse_bits(mut x: u32) -> u32 {
            x = (x >> 16) | (x << 16);
            x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
            x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
            x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
            x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_loop::Solution::reverse_bits);
        test(solution_bit::Solution::reverse_bits);
    }

    fn test(f: impl Fn(u32) -> u32) {
        assert_eq!(
            f(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
        assert_eq!(
            f(0b11111111111111111111111111111101),
            0b10111111111111111111111111111111
        );
    }
}
