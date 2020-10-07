pub mod solution_recursive {
    /// # 思路
    /// 
    /// ```java
    /// @Submission(date = "20201007", memory = 40, memoryBeatRate = 19.34, runtime = 2, runtimeBeatRate = 46.89, url = "https://leetcode-cn.com/submissions/detail/113763877/")
    /// public List<Integer> postorder(Node root) {
    ///     List<Integer> res = new ArrayList<>();
    ///     postorderRecursive(root, res);
    ///     return res;
    /// }
    /// void postorderRecursive(Node root, List<Integer> res) {
    ///     if (root != null) {
    ///         root.children.forEach(child -> postorderRecursive(child, res));
    ///         res.add(root.val);
    ///     }
    /// }
    /// ```
    pub struct Solution;
}

pub mod solution_dfs {
    /// # 思路
    /// 
    /// ```java
    /// @Submission(date = "20201007", memory = 39.8, memoryBeatRate = 45.30, runtime = 4, runtimeBeatRate = 40.71, url = "https://leetcode-cn.com/submissions/detail/113783882/")
    /// public List<Integer> postorder(Node root) {
    ///     // Avoid using Collections.reverse()
    ///     LinkedList<Integer> res = new LinkedList<>();
    ///     Deque<Node> stack = new ArrayDeque<>();
    ///     stack.push(root);
    ///     while (!stack.isEmpty()) {
    ///         Node node = stack.pop();
    ///         // Traverse val in reverse order
    ///         res.addFirst(node.val);
    ///         // Add child to stack in reverse order
    ///         node.children.forEach(stack::addFirst);
    ///     }
    ///     return res;
    /// }
    /// ```
    pub struct Solution;
}
