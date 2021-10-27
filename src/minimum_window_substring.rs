pub mod solution_sliding_window {

    /// # 思路
    ///
    /// 滑动窗口思想
    ///
    /// 用i,j表示滑动窗口的左边界和右边界，通过改变i,j来扩展和收缩滑动窗口，可以想象成一个窗口在字符串上游走，当这个窗口包含的元素满足条件，即包含字符串T的所有元素，记录下这个滑动窗口的长度j-i+1，这些长度中的最小值就是要求的结果。
    ///
    /// 1. 不断增加j使滑动窗口增大，直到窗口包含了T的所有元素
    /// 1. 不断增加i使滑动窗口缩小，因为是要求最小字串，所以将不必要的元素排除在外，使长度减小，直到碰到一个必须包含的元素，这个时候不能再扔了，再扔就不满足条件了，记录此时滑动窗口的长度，并保存最小值
    /// 1. 让i再增加一个位置，这个时候滑动窗口肯定不满足条件了，那么继续从步骤一开始执行，寻找新的满足条件的滑动窗口，如此反复，直到j超出了字符串S范围。
    ///
    /// ![](https://assets.leetcode-cn.com/solution-static/76/76_fig1.gif)
    ///
    /// 参考：
    ///
    /// * [简简单单，非常容易理解的滑动窗口思想](https://leetcode-cn.com/problems/minimum-window-substring/solution/tong-su-qie-xiang-xi-de-miao-shu-hua-dong-chuang-k/)
    /// * [Here is a 10-line template that can solve most 'substring' problems](https://leetcode.com/problems/minimum-window-substring/discuss/26808/Here-is-a-10-line-template-that-can-solve-most-'substring'-problems)
    /// * [方法一：滑动窗口](https://leetcode-cn.com/problems/minimum-window-substring/solution/zui-xiao-fu-gai-zi-chuan-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20211015, mem=2, mem_beats=94, runtime=0, runtime_beats=100
    ///
    /// date=20211027, mem=2.1, mem_beats=95, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn min_window(s: String, t: String) -> String {
            let (s, t) = (s.as_bytes(), t.as_bytes());
            // count for t
            let (mut window_counts, mut window_size) = ([0; 128], 0);
            t.iter().for_each(|b| window_counts[*b as usize] += 1);

            let (mut start_idx, mut min_len) = (0, usize::MAX);

            let (mut left, mut right) = (0, 0);
            // sliding in s
            while right < s.len() {
                let i = s[right] as usize;
                // count if need c
                if window_counts[i] > 0 {
                    window_size += 1;
                }
                window_counts[i] -= 1;

                if window_size == t.len() {
                    // narrow left if window contains all t
                    while left <= right {
                        let i = s[left] as usize;
                        if window_counts[i] >= 0 {
                            break;
                        }
                        window_counts[i] += 1;
                        left += 1;
                    }

                    // get min window size and new left pointer
                    let len = right - left + 1;
                    if len < min_len {
                        min_len = len;
                        start_idx = left;
                    }

                    // slide next from last left, make it invald
                    window_counts[s[left] as usize] += 1;
                    window_size -= 1;
                    left += 1;
                }

                right += 1;
            }

            // res
            if min_len == usize::MAX {
                String::new()
            } else {
                unsafe { std::str::from_utf8_unchecked(&s[start_idx..start_idx + min_len]) }
                    .to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_sliding_window::Solution::min_window);
    }

    fn test<F: Fn(String, String) -> String>(f: F) {
        assert_eq!(
            f("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(f("a".to_string(), "a".to_string()), "a".to_string());
        assert_eq!(f("a".to_string(), "aa".to_string()), "".to_string());
    }
}
