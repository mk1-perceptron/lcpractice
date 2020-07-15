#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return true;
    }

    let mut node_opt = &head;
    let mut vec = Vec::new();

    while let Some(node) = node_opt {
        vec.push(node.val);
        node_opt = &node.next;
    }

    vec.iter()
        .take(vec.len() / 2)
        .zip(vec.iter().rev().take(vec.len() / 2))
        .all(|(a, b)| a == b)
}

fn make_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut pre_head = Box::new(ListNode::new(0));
    let mut cur = &mut pre_head;

    for n in nums {
        cur.next = Some(Box::new(ListNode::new(*n)));
        cur = cur.next.as_mut().unwrap();
    }

    pre_head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = make_list(&[1, 2]);
        let result = is_palindrome(input);

        assert_eq!(result, false);
    }

    #[test]
    fn test_two() {
        let input = make_list(&[1, 2, 2, 1]);
        let result = is_palindrome(input);

        assert_eq!(result, true);
    }

    #[test]
    fn test_empty() {
        let input = make_list(&[]);
        let result = is_palindrome(input);

        assert_eq!(result, true);
    }

    #[test]
    fn test_negatives() {
        let input = make_list(&[-129, -129]);
        let result = is_palindrome(input);

        assert_eq!(result, true);
    }
}
