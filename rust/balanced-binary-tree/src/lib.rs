/// Balanced Binary Tree
///
/// Given a binary tree, determine if it is height-balanced.
/// For this problem, a height-balanced binary tree is defined as:
/// a binary tree in which the left and right subtrees of  every node differ in height by no more
/// than 1
use std::{cell::RefCell, rc::Rc};

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_balanced_helper(root).1
}

fn is_balanced_helper(node_opt: Option<Rc<RefCell<TreeNode>>>) -> (usize, bool) {
    if let Some(node) = node_opt {
        let (left_height, left_balanced) = is_balanced_helper(node.borrow().left.clone());
        let (right_height, right_balanced) = is_balanced_helper(node.borrow().right.clone());
        let max = left_height.max(right_height);
        let min = left_height.min(right_height);
        let height = max + 1;
        let height_diff = max - min;
        let balanced = (left_balanced && right_balanced) && height_diff <= 1;

        (height, balanced)
    } else {
        (0, true)
    }
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

    pub fn from_array(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let vec: Vec<Option<Rc<RefCell<TreeNode>>>> = vals
            .iter()
            .map(|o| o.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))))
            .collect();

        for (i, node_opt) in vec.iter().enumerate() {
            if let Some(node) = node_opt {
                node.borrow_mut().left = vec.get(i * 2 + 1).cloned().flatten();
                node.borrow_mut().right = vec.get(i * 2 + 2).cloned().flatten();
            }
        }

        vec.get(0).cloned().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input =
            TreeNode::from_array(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let result = is_balanced(input);

        assert_eq!(result, true);
    }

    #[test]
    fn example_two() {
        let input = TreeNode::from_array(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        let result = is_balanced(input);

        assert_eq!(result, false);
    }
}
