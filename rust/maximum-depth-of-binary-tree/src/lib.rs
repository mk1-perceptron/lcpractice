/// Maximum Depth of Binary Tree
///
/// Given a binary tree, find its maximum depth.
/// The maximum depth is the number of nodes along the longest path from the root node down to the
/// farthest leaf node.
///
/// Note:
/// A leaf is a node with no children.
use std::{cell::RefCell, cmp::max, rc::Rc};

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_depth = 0;
    let mut stack = Vec::new();

    stack.push((root, 0));

    while let Some((node_opt, depth)) = stack.pop() {
        if let Some(node) = node_opt {
            max_depth = max(max_depth, depth + 1);
            stack.push((node.borrow().left.clone(), depth + 1));
            stack.push((node.borrow().right.clone(), depth + 1));
        }
    }

    max_depth
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_array(vals: &[Option<i32>]) -> Option<Rc<RefCell<Self>>> {
        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vals
            .iter()
            .map(|o| o.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))))
            .collect();

        for (i, node_opt) in nodes.iter().enumerate() {
            if let Some(node) = node_opt {
                let left = nodes.get(i * 2 + 1).cloned().flatten();
                let right = nodes.get(i * 2 + 2).cloned().flatten();
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
            }
        }

        nodes.get(0).cloned().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input =
            TreeNode::from_array(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let result = max_depth(input);

        assert_eq!(result, 3);
    }
}
