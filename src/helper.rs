#![allow(clippy::all)]
pub use std::cell::RefCell;
pub use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn convert_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
    vec_to_tree(str_to_vec(s))
}

/// 将array转为treeNode
///
/// 如果array.len==0则返回None
pub fn vec_to_tree(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if array.len() == 0 {
        return None;
    }
    // 从array中取出val并返回生成节点，调用一次i则i += 1，
    // 如果node.is some，则push back
    let get_node =
        |i: &mut usize, parents: &mut std::collections::VecDeque<Rc<RefCell<TreeNode>>>| {
            // 没有子节点
            if *i >= array.len() {
                return None;
            }
            let node = if let Some(val) = array[*i] {
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                parents.push_back(node.clone());
                Some(node)
            } else {
                None
            };
            *i += 1;
            node
        };
    // 保存上一层的节点
    let mut parents = std::collections::VecDeque::new();
    let root = Rc::new(RefCell::new(TreeNode::new(array[0].unwrap())));
    parents.push_back(root.clone());
    // 树高度
    let mut heigh = 1;
    // 一层一层构造节点
    while !parents.is_empty() {
        let mut i = 2usize.pow(heigh) - 1;
        for _ in 0..parents.len() {
            let root = parents.pop_front().unwrap();
            let mut root = root.borrow_mut();
            root.left = get_node(&mut i, &mut parents);
            root.right = get_node(&mut i, &mut parents);
        }
        heigh += 1;
    }
    Some(root)
}

/// 将`[1,null,2,2]`转化为`[Some(1), None, Some(2), Some(2)]`
///
/// # panic
///
/// 如果str数字间存在空格。如[1,   2 , 3]。
///
/// `  [] `两边空格不影响
pub fn str_to_vec(s: &str) -> Vec<Option<i32>> {
    let a = s
        .trim()
        .strip_prefix("[")
        .map(|s| s.strip_suffix("]"))
        .map(|s| {
            if let Some(s) = s {
                if s.contains(" ") {
                    panic!("存在空格: {}", s);
                }
                s.split(",").collect()
            } else {
                vec![]
            }
        });
    let mut res = vec![];
    if let Some(v) = a {
        for s in v {
            if let Ok(val) = s.parse::<i32>() {
                res.push(Some(val));
            } else {
                res.push(None);
            }
        }
    }
    res
}

/// 如果another中的所有元素都在cur中时返回true
pub fn is_included_vec<T: PartialEq>(cur: &Vec<T>, another: &Vec<T>) -> bool {
    for e in another {
        if !cur.contains(e) {
            return false;
        }
    }
    return true;
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_str_to_vec() {
        let r = str_to_vec("[1,null,2,2]");
        assert_eq!(r.len(), 4);
        assert_eq!(r[1], None);
        assert_eq!(r[0], Some(1));
        assert_eq!(r[3], Some(2));

        assert_eq!(r, str_to_vec("[1,null,2,2]"));
    }

    #[test]
    #[should_panic]
    fn str_to_vec_panic_with_space() {
        str_to_vec("[1,null  ,2,2]");
    }

    #[test]
    fn basic_vec_to_tree() {
        let r = vec_to_tree(vec![Some(1), None, Some(2), Some(2)]);
        assert_eq!(r, vec_to_tree(vec![Some(1), None, Some(2), Some(2)]));
        assert!(r.is_some());
        let r = r.unwrap();
        let root = r.borrow();
        assert_eq!(root.val, 1);

        let left = root.left.clone();
        let right = root.right.clone();
        assert_eq!(left, None);
        assert!(right.is_some() && right.clone().unwrap().borrow().val == 2);

        let left = right.clone().unwrap().borrow().left.clone();
        let right = right.clone().unwrap().borrow().right.clone();
        assert!(left.is_some() && left.clone().unwrap().borrow().val == 2);
        assert_eq!(right, None);
    }

    #[test]
    fn basic_is_included_vec() {
        assert!(is_included_vec(&vec![1, 2], &vec![1]));
        assert!(is_included_vec(&vec![1, 2], &vec![1, 2]));
        assert!(!is_included_vec(&vec![1], &vec![1, 2]));
    }
}

// Some(RefCell { value: TreeNode { val: 1, left: None, right: Some(RefCell { value: TreeNode { val: 2, left: Some(RefCell { value: TreeNode { val: 2, left: None, right: None } }), right: None } }) } })

pub mod utils {
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basic() {
            assert!(is_contains(&vec![2321, 2, 2, 2, 1], &vec![]));
            let a = vec![vec![1, 1], vec![2], vec![3], vec![2, 3]];
            let b = vec![vec![1], vec![2], vec![5]];
            assert!(!is_contains_vec2(&a, &b));

            assert!(is_contains(&vec![2321, 2, 2, 2, 1], &vec![]));
            assert!(is_contains_vec2(&vec![vec![1, 2], vec![4, 5]], &vec![]));
            assert!(!is_contains_vec2(
                &vec![vec![1, 2], vec![4, 5]],
                &vec![vec![10]]
            ));
            assert!(!is_contains_vec2(
                &vec![vec![1, 2], vec![4, 5]],
                &vec![vec![1], vec![10]]
            ));
        }
    }

    /// 如果o包含other则返回true。o的长度元素可以多于other，只要other的元素都在o中即可
    pub fn is_contains_vec2<T: Eq>(o: &Vec<Vec<T>>, other: &Vec<Vec<T>>) -> bool {
        if o.len() < other.len() {
            return false;
        }
        for a in other {
            if o.iter().find(|b| is_contains(*b, a)).is_none() {
                return false;
            }
        }
        true
    }

    pub fn is_contains<'a, I, T: 'a>(o: I, other: I) -> bool
    where
        T: Eq,
        I: IntoIterator<Item = &'a T> + Copy,
    {
        if o.into_iter().count() < other.into_iter().count() {
            return false;
        }
        let mut o_iter = o.into_iter();
        for b in other.into_iter() {
            if !o_iter.find(|a| *a == b).is_some() {
                return false;
            }
        }
        true
    }
}
