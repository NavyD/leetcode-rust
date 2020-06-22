pub struct Solution;
use std::i32;

impl Solution {
    // pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    //     if nums.len() < 2 {
    //         return 0;
    //     }
    //     let mut nums = nums;
    //     nums.sort_unstable();
    //     let mut max_gap = 0;
    //     for i in 1..nums.len() {
    //         max_gap = i32::max(max_gap, &nums[i] - &nums[i-1]);
    //     }
    //     max_gap
    // }
    

    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        // 0. find max and min num
        let mut min_num = nums[0];
        let mut max_num = nums[0];
        for num in &nums {
            let num = *num;
            if min_num > num {
                min_num = num;
            } else if max_num < num {
                max_num = num;
            }
        }
        // 1. create buckets with gap/size
        // gap is bucket size not buckets.length
        // [1,1,1,1,1,5,5,5,5,5] (5-1)/(10-1) = 0 ==> gap = 1
        let gap = usize::max(1, (max_num - min_num) as usize / (nums.len() - 1));
        let bucket_len = nums.len();
        let mut min_buckets = vec![i32::MAX; bucket_len];
        let mut max_buckets = vec![i32::MIN; bucket_len];
        for num in nums {
            if num == max_num {
                continue;
            }
            let i = (num - min_num) as usize / gap;
            // 2. find each other bucket min/max
            min_buckets[i] = i32::min(min_buckets[i], num);
            max_buckets[i] = i32::max(max_buckets[i], num);
        }
        // 3. find max gap with buckets adjacent
        let mut max_gap = 0;
        let mut prev_max = max_buckets[0];
        for i in 1..bucket_len {
            if max_buckets[i] == i32::MIN {
                continue;
            }
            max_gap = i32::max(max_gap, min_buckets[i] - prev_max);
            // max_gap = i32::max(max_buckets[i-1] - min_buckets[i], max_gap);
            prev_max = max_buckets[i];
        }

        i32::max(max_gap, max_num - prev_max)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::maximum_gap(vec![3,6,9,1]), 3);
        eprintln!("gap: {}", Solution::maximum_gap(vec![12115,10639,2351,29639,31300,11245,16323,24899,8043,4076,17583,15872,19443,12887,5286,6836,31052,25648,17584,24599,13787,24727,12414,5098,26096,23020,25338,28472,4345,25144,27939,10716,3830,13001,7960,8003,10797,5917,22386,12403,2335,32514,23767,1868,29882,31738,30157,7950,20176,11748,13003,13852,19656,25305,7830,3328,19092,28245,18635,5806,18915,31639,24247,32269,29079,24394,18031,9395,8569,11364,28701,32496,28203,4175,20889,28943,6495,14919,16441,4568,23111,20995,7401,30298,2636,16791,1662,27367,2563,22169,1607,15711,29277,32386,27365,31922,26142,8792]));
        assert_eq!(Solution::maximum_gap(vec![1,1,1,1,1,5,5,5,5,5]), 4);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
