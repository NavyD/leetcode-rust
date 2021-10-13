pub mod solution_bucket {
    /// # 思路
    ///
    /// 我们设计桶的大小为 n+1，则相同的任务恰好不能放入同一个桶，最密也只能放入相邻的桶。对于重复的任务，我们只能将每个都放入不同的桶中，因此桶的个数就是重复次数最多的任务的个数。
    ///
    /// 1. 当冷却时间长，任务种类很少时idle：总排队时间 = (桶个数 - 1) * (n + 1) + 最后一桶的任务数
    ///
    ///     ![](https://pic.leetcode-cn.com/8993d48fb4109d5d207f135bf73e10fd22c898c25113e5fa09bc91829790f9a0-image.png)
    ///
    /// 2. 当冷却时间短，任务种类很多时busy：执行任务所需的时间，就是任务的数量。
    ///
    ///     刚好排满了任务，如果现在我还要执行两次任务 F，此时我们可以临时扩充某些桶子的大小，插进任务 F。
    ///     我们在第一个、第二个桶子里插入了任务F，不难发现无论再继续插入多少任务，我们都可以类似处理，而且新插入元素肯定满足冷却要求
    ///
    ///     ![](https://pic.leetcode-cn.com/893c01db5923889a865d7a4fe71de22b9519fc5a673473196ab58f26c1073ed2-image.png)
    ///
    /// 最后只需要比较这两种情况，如果是idle时存在空闲时间，idle时间长 idle > busy，而busy时排满不存在空闲时间busy时间短idle<=busy
    /// TODO 为何要输出busy.max(idle)，按理是Min？
    ///
    /// 在找last_tasks时通过计数的方式，只有数量与max_tasks相同的task才能放到最后的桶中
    ///
    /// 参考：
    ///
    /// * [【任务调度器】C++ 桶子_配图理解](https://leetcode-cn.com/problems/task-scheduler/solution/tong-zi-by-popopop/)
    /// * [桶思想-简洁高效](https://leetcode-cn.com/problems/task-scheduler/solution/tong-si-xiang-jian-ji-gao-xiao-by-hzhu212/)
    ///
    /// ### Submissions
    ///
    /// date=20211013, mem=2.7, mem_beats=27, runtime=12, runtime_beats=100
    ///
    pub struct Solution;

    impl Solution {
        pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
            let mut counts = [0; 26];
            let mut buckets = 0;
            let busy = tasks.len() as i32;
            for task in tasks {
                let i = task as usize - 'A' as usize;
                counts[i] += 1;
                // find the buckets by the max tasks
                buckets = buckets.max(counts[i]);
            }

            // find tasks in last bucket
            let last_tasks = counts.iter().filter(|count| **count == buckets).count() as i32;
            let idle = (buckets - 1) * (n + 1) + last_tasks;
            busy.max(idle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_bucket::Solution::least_interval);
    }

    fn test<F: Fn(Vec<char>, i32) -> i32>(f: F) {
        assert_eq!(f(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
        assert_eq!(f(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
        assert_eq!(
            f(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            ),
            16
        );
    }
}
