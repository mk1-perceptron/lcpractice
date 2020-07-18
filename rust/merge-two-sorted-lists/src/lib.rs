/// Merge Two Sorted Lists
///
/// Merge two sorted linked lists and return it as a new sorted list. The new list should be made
/// by splicing together the nodes of the first two lists.

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;
    let mut pre_head = Box::new(ListNode::new(0));
    let mut cur = &mut pre_head;

    while l1.is_some() && l2.is_some() {
        let v1 = l1.as_ref().unwrap().val;
        let v2 = l2.as_ref().unwrap().val;

        if v1 < v2 {
            cur.next = Some(Box::new(ListNode::new(v1)));
            l1 = &l1.as_ref().unwrap().next;
        } else {
            cur.next = Some(Box::new(ListNode::new(v2)));
            l2 = &l2.as_ref().unwrap().next;
        }
        cur = cur.next.as_mut().unwrap();
    }

    while let Some(node) = l1 {
        cur.next = Some(Box::new(ListNode::new(node.val)));
        l1 = &node.next;
        cur = cur.next.as_mut().unwrap();
    }

    while let Some(node) = l2 {
        cur.next = Some(Box::new(ListNode::new(node.val)));
        l2 = &node.next;
        cur = cur.next.as_mut().unwrap();
    }

    pre_head.next
}

pub fn make_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut pre_head = Box::new(ListNode::new(0));
    let mut cur = &mut pre_head;

    for val in nums {
        cur.next = Some(Box::new(ListNode::new(*val)));
        cur = cur.next.as_mut().unwrap();
    }

    pre_head.next
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input_one = make_list(&[1, 2, 4]);
        let input_two = make_list(&[1, 3, 4]);
        let result = merge_two_lists(input_one, input_two);

        assert_eq!(result, make_list(&[1, 1, 2, 3, 4, 4]));
    }
}
