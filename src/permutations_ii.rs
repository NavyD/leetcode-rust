pub mod solution_used {
    /// ### submissions
    /// 
    /// date=20201106, mem=2, mem_beats=70.67, runtime=4, runtime_beats=94, url=https://leetcode-cn.com/submissions/detail/121404268/
    pub struct Solution;

    impl Solution {
        pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut used = vec![];
            nums.sort_unstable();
            nums.iter().for_each(|_| used.push(false));
            let mut res = vec![];
            Self::helper(&mut nums, 0, &mut used, &mut vec![], &mut res);
            res
        }

        fn helper(nums: &[i32], start: usize, used: &mut [bool], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                res.push(path.to_vec());
                return;
            }
            for i in 0..nums.len() {
                if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                    continue;
                }
                path.push(nums[i]);
                used[i] = true;
                Self::helper(nums, start + 1, used, path, res);
                used[i] = false;
                path.pop();
            }
        }
    }
}