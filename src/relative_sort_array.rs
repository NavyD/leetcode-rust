pub mod solution_count {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [数组的相对排序](https://leetcode.cn/problems/relative-sort-array/solution/shu-zu-de-xiang-dui-pai-xu-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20220623, mem=2.2, mem_beats=7, runtime=0, runtime_beats=100
    ///
    /// date=20220624, mem=2, mem_beats=92, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
            let mut counts = [0; 1001];
            for num in &arr1 {
                counts[*num as usize] += 1;
            }

            let mut i = 0;
            for num in arr2 {
                while counts[num as usize] > 0 {
                    counts[num as usize] -= 1;
                    arr1[i] = num as i32;
                    i += 1;
                }
            }

            for num in 0..counts.len() {
                for _ in 0..counts[num] {
                    arr1[i] = num as i32;
                    i += 1;
                }
            }

            arr1
        }
    }
}

pub mod solution_sort {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [数组的相对排序](https://leetcode.cn/problems/relative-sort-array/solution/shu-zu-de-xiang-dui-pai-xu-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20220624, mem=2, mem_beats=92, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
            let mut counts = [-1; 1001];
            for (i, num) in arr2.into_iter().enumerate() {
                counts[num as usize] = i as i32;
            }

            arr1.sort_by_key(|num| {
                let num = *num as usize;
                if counts[num] >= 0 {
                    (0, counts[num])
                } else {
                    (1, num as i32)
                }
            });
            arr1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_count::Solution::relative_sort_array);
        test(solution_sort::Solution::relative_sort_array);
    }

    fn test(f: impl Fn(Vec<i32>, Vec<i32>) -> Vec<i32>) {
        assert_eq!(
            f(
                [2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19].to_vec(),
                [2, 1, 4, 3, 9, 6].to_vec()
            ),
            [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19].to_vec()
        );
        assert_eq!(
            f([28, 6, 22, 8, 44, 17].to_vec(), [22, 28, 8, 6].to_vec()),
            [22, 28, 8, 6, 17, 44].to_vec()
        );
    }
}
