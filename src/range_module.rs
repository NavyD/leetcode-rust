pub mod solution {
    use super::RangeModule;

    use std::collections::BTreeMap;

    /// # 思路
    ///
    /// 注意：
    ///
    /// * 在remove_range时必须先`self.map.insert(right, end_right);`，
    /// 因为如果先插入left时会把`map.insert(start, left)`之前的start对应的rigth给用left覆盖掉
    /// * 在retain时应该针对的是key，而不是value. `key>left||key<right`
    /// * 对应java：`map.floorKey(left);`的rust：`map.range(..=left).next_back()`
    ///
    /// 参考：
    ///
    /// * [Java TreeMap](https://leetcode.com/problems/range-module/discuss/108910/Java-TreeMap)
    /// * [Range 模块](https://leetcode.cn/problems/range-module/solution/range-mo-kuai-by-leetcode-solution-4utf/)
    ///
    /// ### Submissions
    ///
    /// date=20220622, mem=6.2, mem_beats=84, runtime=56, runtime_beats=66
    #[derive(Debug)]
    pub struct Solution {
        map: BTreeMap<i32, i32>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl RangeModule for Solution {
        fn new() -> Self {
            Self {
                map: BTreeMap::new(),
            }
        }

        fn add_range(&mut self, left: i32, right: i32) {
            let (mut left, mut right) = (left, right);

            // start<=left<=start_right
            if let Some((start, _)) = self
                .map
                .range(..=left)
                .next_back()
                .filter(|(_, start_right)| **start_right >= left)
            {
                left = *start;
            }

            // end<=right<=end_right
            if let Some(end_right) = self
                .map
                .range(..=right)
                .next_back()
                .map(|(_, end_right)| end_right)
                .filter(|end_right| **end_right > right)
            {
                right = *end_right;
            }

            self.map.insert(left, right);

            self.map.retain(|k, _| *k <= left || *k > right);
        }

        fn query_range(&self, left: i32, right: i32) -> bool {
            self.map
                .range(..=left)
                .next_back()
                .map(|(_, end)| *end >= right)
                .unwrap_or(false)
        }

        fn remove_range(&mut self, left: i32, right: i32) {
            // right must be overridden first
            // end <=   right<=end_right
            if let Some(end_right) = self
                .map
                .range(..=right)
                .next_back()
                .map(|(_, v)| *v)
                .filter(|end_right| *end_right > right)
            {
                self.map.insert(right, end_right);
            }

            // start<= left     <=start_right
            if let Some(start) = self
                .map
                .range(..=left)
                .next_back()
                .filter(|(_, start_right)| **start_right > left)
                .map(|(k, _)| *k)
            {
                self.map.insert(start, left);
            }

            self.map.retain(|k, _| *k < left || *k >= right);
        }
    }
}

trait RangeModule {
    fn new() -> Self;

    fn add_range(&mut self, left: i32, right: i32);

    fn query_range(&self, left: i32, right: i32) -> bool;

    fn remove_range(&mut self, left: i32, right: i32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn basics() {
        test::<solution::Solution>();
    }

    fn test<T: RangeModule + Debug>() {
        let mut range_module = T::new();
        range_module.add_range(10, 20);
        range_module.remove_range(14, 16);

        assert!(range_module.query_range(10, 14)); // 返回 true （区间 [10, 14) 中的每个数都正在被跟踪）
        assert!(!range_module.query_range(13, 15)); // 返回 false（未跟踪区间 [13, 15) 中像 14, 14.03, 14.17 这样的数字）
        assert!(range_module.query_range(16, 17)); // 返回 true （尽管执行了删除操作，区间 [16, 17) 中的数字 16 仍然会被跟踪）

        let mut range_module = T::new();
        range_module.add_range(10, 180);
        range_module.add_range(150, 200);
        range_module.add_range(250, 500);
        assert!(range_module.query_range(50, 100));
        assert!(!range_module.query_range(180, 300));
        assert!(!range_module.query_range(600, 1000));
        range_module.remove_range(50, 150);
        assert!(!range_module.query_range(50, 100));
    }
}
