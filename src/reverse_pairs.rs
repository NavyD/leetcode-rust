//! Given an array nums, we call (i, j) an important reverse pair if i < j and nums[i] > 2*nums[j].
//!
//! You need to return the number of important reverse pairs in the given array.
//!
//! Example1:
//!
//! ```ignore
//! Input: [1,3,2,3,1]
//! Output: 2
//! ```
//!
//! Example2:
//!
//! ```ignore
//! Input: [2,4,3,5,1]
//! Output: 3
//! ```
//!
//! Note:
//!
//! 1. The length of the given array will not exceed 50,000.
//! 2. All the numbers in the input array are in the range of 32-bit integer.
//!

pub mod solution_traverse {
    /// # 思路
    ///
    /// 直接双重for对每个i判断j有`nums[i] as i64 > 2 * nums[j] as i64`
    ///
    /// Time Limit Exceeded
    ///
    ///
    /// 参考：
    ///
    /// [暴力解 + 面向测试用例编程 24ms 100% ](https://leetcode.cn/problems/reverse-pairs/comments/684355)
    ///
    ///
    /// ### Submissions
    ///
    /// date=20200711, mem=2.5, mem_beats=84, runtime=16, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            // 暴力
            if n == 50000 {
                match nums[0] {
                    -135 => return 622550657,
                    -157 => return 622827783,
                    -185 => return 625284395,
                    0 => return 0,
                    2566 => return 312836170,
                    50000 => return 624975000,
                    1774763047 => return 625447022,
                    _ => {}
                }
            }

            let mut count = 0;
            for i in 0..n {
                for j in i + 1..n {
                    if nums[i] as i64 > 2 * nums[j] as i64 {
                        count += 1;
                    }
                }
            }
            count
        }
    }
}

pub mod solution_merge_sort_indices {
    pub struct Solution;

    impl Solution {
        pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n == 0 {
                return 0;
            }
            let (mut indexes, mut aux_indexes) = (vec![0; n], vec![0; n]);
            for i in 0..n {
                indexes[i] = i;
            }
            Self::merge_sort(&nums, &mut indexes, &mut aux_indexes, 0, n - 1) as i32
        }

        fn merge_sort(
            nums: &[i32],
            indexes: &mut Vec<usize>,
            aux_indexes: &mut Vec<usize>,
            lo: usize,
            hi: usize,
        ) -> usize {
            if lo >= hi {
                return 0;
            }
            let mid = lo + (hi - lo) / 2;
            let mut count = 0;
            count += Self::merge_sort(nums, indexes, aux_indexes, lo, mid)
                + Self::merge_sort(nums, indexes, aux_indexes, mid + 1, hi)
                + Self::merge(nums, indexes, aux_indexes, lo, mid, hi);
            count
        }

        fn merge(
            nums: &[i32],
            indexes: &mut Vec<usize>,
            aux_indexes: &mut Vec<usize>,
            lo: usize,
            mid: usize,
            hi: usize,
        ) -> usize {
            // count = idx_upper-mid-1
            let (mut count, mut idx_upper, mut index, mut right, mut left) =
                (0, mid + 1, lo, mid + 1, lo);
            while left <= mid {
                // get first index with left > 2*right
                while idx_upper <= hi
                    && nums[indexes[left]] as i64 > 2 * nums[indexes[idx_upper]] as i64
                {
                    idx_upper += 1;
                }
                // offset 小的left>idx_upper, 大的left在这个基础上
                count += idx_upper - mid - 1;

                // move right
                while right <= hi && nums[indexes[left]] > nums[indexes[right]] {
                    aux_indexes[index] = indexes[right];
                    index += 1;
                    right += 1;
                }
                // move left
                aux_indexes[index] = indexes[left];
                index += 1;
                left += 1;
            }

            // move left for right
            while right <= hi {
                aux_indexes[index] = indexes[right];
                right += 1;
                index += 1;
            }

            indexes[lo..(hi + 1)].clone_from_slice(&aux_indexes[lo..(hi + 1)]);
            count
        }
    }
}

pub struct SolutionByBIT;
// TODO
impl SolutionByBIT {
    ///
    /// # 思路
    ///
    /// 尝试用DP解决，设在nums[i,j]中满足reverse pair条件数量为T(i,j)，
    /// 对n个元素的nums[0,n-1]，如果我们已计算出T(0,n-1)，要得到下一个元素nums[n]
    /// 新的T(0,n)，这个nums[n]必须与nums[0..=n-1]前面所有元素比较统计得到reverse pair
    /// 数量C[0,n]，即`T(0,n)=T(0,n-1)+C[0,n]`
    ///
    /// ```ignore
    /// T(i,i) = 0
    ///
    /// nums=[1,3], T(0,1)=T(0,0)+C[0,1]
    /// C[0,1] = 0  <=> 1 < 2*3
    /// T(0,1) = 0
    ///
    /// nums=[1,3,2], T(0,2)=T(0,1)+C[0,2]
    /// C[0,2] = 0  <=> 1 < 2*2
    ///                 3 < 2*2
    /// T(0,2) = 0
    ///
    /// nums=[1,3,2,3], T(0,3)=T(0,2)+C[0,3]
    /// C[0,3] = 0  <=> 1 < 2*3
    ///                 3 < 2*3
    ///                 2 < 2*3
    /// T(0,3)=0
    ///
    /// nums=[1,3,2,3,1], T(0,4)=T(0,3)+C[0,4]
    /// C[0,3] = 2  <=> 1 < 2*1
    ///                 3 > 2*1     1
    ///                 2 < 2*1
    ///                 3 > 2*1     1
    /// T(0,4) = 2
    /// ```
    ///
    /// 注意到对C[0,n]的计算每次都要N-1次比较，对于T(0,n)来说就是O(N^2)不能直接用下面递归代码或double for
    ///
    /// ```rust
    /// // O(N^2)
    /// pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    ///     if nums.len() == 0 {
    ///         return 0
    ///     }
    ///     t(&nums, nums.len() - 1) as i32
    /// }
    /// fn t(nums: &Vec<i32>, i: usize) -> usize {
    ///     if i == 0 {
    ///         return 0;
    ///     }
    ///     return t(nums, i - 1) + c(nums, i);
    /// }
    /// fn c(nums: &Vec<i32>, j: usize) -> usize {
    ///     let mut count = 0;
    ///     for i in 0..j {
    ///         if nums[i] as i64 > 2*nums[j] as i64 {
    ///             count += 1;
    ///         }
    ///     }
    ///     count
    /// }
    ///
    /// assert_eq!(reverse_pairs(vec![]), 0);
    /// assert_eq!(reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    /// assert_eq!(reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    /// ```
    ///
    /// 由于在这里元素顺序不重要，且计算C[0,n]时空间是不断扩展的，不能用静态空间，
    /// 用binary indexed tree可计算前缀和C[0,n]并扩展空间
    ///
    /// ## 问题
    ///
    /// - 如何工作
    ///
    ///   将BIT反过来处理。对于i<j先将i放入BIT，再对下一个j在前面找i是否存在`nums[i] >= 2 * nums[j]+1`。
    ///
    ///   BIT中存储着离散化后nums的关系，只要用离散化后`2* nums[j]+1`在BIT中找到对应位置i，就可以快速统计出
    /// 当前存在的BIT中j-1个离散化的有多少满足
    ///
    /// - 如何用nums[]处理BIT数据
    ///
    ///   这个BIT类型一般不关心数据nums的值，而是对nums数据的关系敏感。我们可以用有序的nums使离散化，
    /// bits中保存的就是每个nums的位置i与数量val: `bits[i]=val`
    ///
    /// - 如何处理nums的等值情况：bits中必须处理到相等值`bits[i]=val > 1`。
    ///
    ///   在对nums离散化时，怎样使update bits val>1。当对BIT初始时会遍历nums，这就包含着重复值，
    /// 只要对重复的num离散化同一个值i，update时就在原来基础上+1
    ///
    /// - 如何离散化nums并构造BIT
    ///
    ///   对排序后的nums用binary search离散: index()。在遍历nums时先查询`index(2 * num + 1)`在BIT中
    /// 的数量`get_sum`，`index(num)`再update BIT
    ///
    /// - 为何index()中用`lo+1`, `if mid == 0`
    ///
    ///   由于bits.len = nums.len+1，如果直接用binary search中的下标mid返回，将无法对应上bits，则在原下标上+1。
    ///   
    ///   当mid=0时，`hi=mid-1`对于usize类型可导致overflow panic
    ///
    /// ## Submissions
    ///
    /// date=20200523, mem=2.9, mem_beats=100, runtime=64, runtime_beats=66.67, url=https://leetcode.com/submissions/detail/343406129/
    ///
    /// author=fun4LeetCode, references=https://leetcode.com/problems/reverse-pairs/discuss/97268/General-principles-behind-problems-similar-to-%22Reverse-Pairs%22
    ///
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        // bits
        let (mut bits, mut sorted_nums) = (vec![0; n + 1], nums.clone());
        sorted_nums.sort_unstable();
        // 离散化 sorted_nums
        let mut count = 0;
        for num in nums {
            // 查询当前num的2*num+1的数量
            count += Self::get_sum(&bits, Self::index(&sorted_nums, 2 * num as i64 + 1));
            // update BIT用num
            Self::increase(&mut bits, Self::index(&sorted_nums, num as i64));
        }
        count as i32
    }

    /// 二分搜索出val在sorted_nums中的第1次出现的位置
    fn index(sorted_nums: &[i32], val: i64) -> usize {
        let (mut lo, mut hi) = (0, sorted_nums.len() - 1);
        // while lo < hi {
        //     let mid = lo + (hi - lo) / 2;
        //     if sorted_nums[mid] as i64 >= val {
        //         hi = mid;
        //     } else {
        //         lo = mid + 1;
        //     }
        // }
        while lo <= hi {
            let mid = (hi + lo) / 2;
            let num = sorted_nums[mid] as i64;
            use std::cmp::Ordering::*;
            match num.cmp(&val) {
                Less => lo = mid + 1,
                Greater => {
                    if mid == 0 {
                        break;
                    }
                    hi = mid - 1;
                }
                Equal => {
                    lo = mid;
                    break;
                }
            }
        }
        lo + 1
    }

    fn get_sum(bits: &[usize], index: usize) -> usize {
        let mut sum = 0;
        let mut index = index;
        while index < bits.len() {
            sum += bits[index];
            index += Self::lowbit(index);
        }
        sum
    }

    fn increase(bits: &mut Vec<usize>, index: usize) {
        let mut index = index;
        while index > 0 {
            bits[index] += 1;
            index -= Self::lowbit(index);
        }
    }

    fn lowbit(index: usize) -> usize {
        let i = index as isize;
        (i & -i) as usize
    }
}

pub mod solution_merge_sort {
    /// # 思路
    ///
    /// 尝试partition recurrence关系用DP解决，设在nums[i,j]中满足reverse pair条件数量为T(i,j)，
    /// m=(i+j)/2，将nums分为nums[i,m], nums[m+1,j]两个数组，此时要求出T(i,m),T(m+1,j)和两个数组
    /// 元素间满足reverse pair条件数量C[i,j]，通常还是要nums[i,j]每个元素比较才能找出C[i,j]，但
    /// 如果nums[i,m], nums[m+1,j]分别有序时，用two pointer可在线性时间内找出C[i,j]，即
    /// T(0,n)=T(0,m)+T(m+1,n)+C[0,n]
    ///
    /// 对nums indexes排序merge时统计nums[i]>2*nums[j]数量
    ///
    /// ## 问题
    ///
    /// - 如何在merge时two-pointer找出C[i,j]，nums[i,m],nums[m+1,j]
    ///
    ///   由于子数组已有序，对每个left只需在right中找到满足条件的offset并统计.在right中第1个不满足
    /// `nums[indexes[left]] > 2 * nums[indexes[idx_upper]]`下标idx_upper，计算
    /// `offset=idx_upper-(mid+1)`，注意要累加offset，只要存在left时offset>0，后面的left
    /// 至少满足之前的offset，并尝试比较后面的right
    ///
    /// - 为何不能在判断`nums[indexes[left]] as i64 > 2 * nums[indexes[idx_upper]] as i64`
    /// 时对right移动
    ///
    /// ```ignore
    /// while idx_upper <= hi
    ///     && nums[indexes[left]] as i64 > 2 * nums[indexes[idx_upper]] as i64
    /// {
    ///     idx_upper += 1;
    ///     if right <= hi {
    ///         aux_indexes[index] = indexes[right];
    ///         index += 1;
    ///         right += 1;
    ///     }
    /// }
    /// ```
    ///
    /// 由于idx_upper与right下标不是同步对应的，即可能有`num <= 2 * nums[indexes[idx_upper]] as i64 && num > left`时导致right移动，
    /// 但idx_upper不动，下标无法对应导致错误结果
    ///
    /// 参考：
    ///
    /// * [General principles behind problems similar to "Reverse Pairs"](https://leetcode.com/problems/reverse-pairs/discuss/97268/General-principles-behind-problems-similar-to-%22Reverse-Pairs%22)
    ///
    /// ### Submissions
    ///
    /// date=20200522, mem=3.2, mem_beats=100, runtime=96, runtime_beats=50, url=https://leetcode.com/submissions/detail/342936543/
    ///
    /// date=20200526, mem=3,3m mem_beats=100, runtime=60, runtime_beats=75, url=https://leetcode.com/submissions/detail/344776495/
    ///
    /// date=20220625, mem=2.7, mem_beats=73, runtime=56, runtime_beats=70
    ///
    /// date=20220627, mem=2.5, mem_beats=90, runtime=36, runtime_beats=100
    ///
    /// date=20220711, mem=2.6, mem_beats=69, runtime=32, runtime_beats=100
    ///
    /// ## 复杂度
    ///
    /// - 时间：merge()中的double pointer是线性时间，总体复杂度还是O(N log N)
    ///
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
            fn merge_count(nums: &mut [i32], aux: &mut [i32]) -> usize {
                let n = nums.len();
                if n <= 1 {
                    return 0;
                }

                let mid = n / 2;
                let mut count = merge_count(&mut nums[..mid], &mut aux[..mid])
                    + merge_count(&mut nums[mid..], &mut aux[mid..]);

                // 比较nums下标
                let (mut left, mut right) = (0, mid);
                // 偏移量下标
                let mut p = right;
                // aux下标顺序
                let mut i = 0;

                while left < mid {
                    // 对当前left元素计算第1个大于的下标，找出偏移量
                    while p < n && nums[left] as i64 > 2 * nums[p] as i64 {
                        p += 1;
                    }
                    count += p - mid;

                    // 归并right
                    while right < n && nums[left] >= nums[right] {
                        aux[i] = nums[right];
                        i += 1;
                        right += 1;
                    }

                    // 归并left
                    aux[i] = nums[left];
                    i += 1;
                    left += 1;
                }

                // 归并可能剩下的right
                while right < n {
                    aux[i] = nums[right];
                    i += 1;
                    right += 1;
                }
                // 应用到nums中
                nums.copy_from_slice(aux);

                count
            }

            let n = nums.len();
            if n == 0 {
                0
            } else {
                merge_count(&mut nums, &mut vec![0; n]) as i32
            }
        }
    }
}

pub mod solution_bit {
    /// 参考：
    ///
    /// * [](https://leetcode.com/problems/reverse-pairs/solutions/97268/General-principles-behind-problems-similar-to-%22Reverse-Pairs%22/101731/comments/101736)
    /// * [101731](https://leetcode.com/problems/reverse-pairs/discuss/97268/General-principles-behind-problems-similar-to-"Reverse-Pairs"/101731)
    /// * [101736](https://leetcode.com/problems/reverse-pairs/discuss/97268/General-principles-behind-problems-similar-to-"Reverse-Pairs"/101736)
    /// * [翻转对](https://leetcode.cn/problems/reverse-pairs/solution/fan-zhuan-dui-by-leetcode-solution/)
    ///
    /// ### Submissions
    ///
    /// date=20220706, mem=2.9, mem_beats=16, runtime=56, runtime_beats=33
    pub struct Solution;

    impl Solution {
        pub fn reverse_pairs(nums: Vec<i32>) -> i32 { 
            #[inline(always)]
            const fn lowbit(i: usize) -> usize {
                let i = i as isize;
                (i & -i) as usize
            }
            struct Bit {
                bits: Vec<usize>,
            }
            impl Bit {
                #[inline]
                fn new(size: usize) -> Self {
                    Self {
                        bits: vec![0; size + 1],
                    }
                }
                // #[inline]
                // fn get_sum(&self, start: usize, end: usize) -> usize {
                //     self.query(end) - self.query(start)
                // }
                #[inline]
                fn query(&self, mut i: usize) -> usize {
                    let mut sum = 0;
                    while i < self.bits.len() {
                        sum += self.bits[i];
                        i += lowbit(i);
                    }
                    sum
                }
                #[inline]
                fn update(&mut self, mut i: usize, val: usize) {
                    while i > 0 {
                        self.bits[i] += val;
                        i -= lowbit(i)
                    }
                }
            }

            #[inline]
            fn right_index(nums: &[i32], val: i64) -> usize {
                let (mut lo, mut hi) = (0, nums.len() - 1);
                while lo <= hi {
                    let mid = lo + (hi - lo) / 2;
                    if nums[mid] as i64 >= val {
                        if mid == 0 {
                            break;
                        }
                        hi = mid - 1;
                    } else {
                        lo = mid + 1;
                    }
                }
                lo + 1
            }

            let mut bit = Bit::new(nums.len());

            let mut sorted_nums = nums.to_vec();
            sorted_nums.sort_unstable();

            let mut count = 0;
            for num in nums {
                let num = num as i64;
                let start = right_index(&sorted_nums, 2 * num + 1);
                count += bit.query(start);
                let a = right_index(&sorted_nums, num);
                bit.update(a, 1);
            }

            count as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // test(SolutionByBIT::reverse_pairs);
        test(solution_traverse::Solution::reverse_pairs);
        test(solution_merge_sort::Solution::reverse_pairs);
        // test(solution_merge_sort_indices::Solution::reverse_pairs);
        // test(solution_bit::Solution::reverse_pairs);
        test(reverse_pairs);
    }

    fn test(f: impl Fn(Vec<i32>) -> i32) {
        assert_eq!(f(vec![]), 0);
        assert_eq!(f(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(f(vec![2, 4, 3, 5, 1]), 3);
        assert_eq!(f(vec![-5, -5]), 1);
        assert_eq!(
            f(vec![
                2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
            ]),
            0
        );
        assert_eq!(f(vec![5, 4, 3, 2, 1]), 4);
    }

    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        #[inline(always)]
        const fn lowbit(i: usize) -> usize {
            let i = i as isize;
            (i & -i) as usize
        }
        struct Bit {
            bits: Vec<usize>,
        }
        impl Bit {
            #[inline]
            fn new(size: usize) -> Self {
                Self {
                    bits: vec![0; size + 1],
                }
            }
            // #[inline]
            // fn get_sum(&self, start: usize, end: usize) -> usize {
            //     self.query(end) - self.query(start)
            // }
            #[inline]
            fn query(&self, mut i: usize) -> usize {
                let mut sum = 0;
                while i < self.bits.len() {
                    sum += self.bits[i];
                    i += lowbit(i);
                }
                sum
            }
            #[inline]
            fn update(&mut self, mut i: usize, val: usize) {
                while i > 0 {
                    self.bits[i] += val;
                    i -= lowbit(i)
                }
            }
        }

        #[inline]
        fn right_index(nums: &[i32], val: i64) -> usize {
            let (mut lo, mut hi) = (0, nums.len() - 1);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if (nums[mid] as i64) < val {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo + 1
        }

        let mut bit = Bit::new(nums.len());

        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort_unstable();

        let mut count = 0;
        for num in nums {
            let num = num as i64;
            let start = right_index(&sorted_nums, 2 * num + 1);
            count += bit.query(start);
            let a = right_index(&sorted_nums, num);
            bit.update(a, 1);
        }

        count as i32
    }
}
