// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_elementary_math(l1, l2)
    }

    /// **Approach 1: Elementary Math**
    ///
    fn add_two_numbers_elementary_math(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut curr = &mut dummy_head;
        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let x = l1.as_ref().map(|node| node.val).unwrap_or(0);
            let y = l2.as_ref().map(|node| node.val).unwrap_or(0);

            let sum = carry + x + y;

            carry = sum / 10;

            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_numbers() {
        // l1 = [2, 4, 3] (represents 342)
        let mut l1 = Box::new(ListNode::new(2));
        let mut l1_2 = Box::new(ListNode::new(4));
        let l1_3 = Box::new(ListNode::new(3));
        l1_2.next = Some(l1_3);
        l1.next = Some(l1_2);

        // l2 = [5, 6, 4] (represents 465)
        let mut l2 = Box::new(ListNode::new(5));
        let mut l2_2 = Box::new(ListNode::new(6));
        let l2_3 = Box::new(ListNode::new(4));
        l2_2.next = Some(l2_3);
        l2.next = Some(l2_2);

        // Expected result = [7, 0, 8] (represents 807)
        let result = Solution::add_two_numbers(Some(l1), Some(l2));

        let mut expected = Box::new(ListNode::new(7));
        let mut expected_2 = Box::new(ListNode::new(0));
        let expected_3 = Box::new(ListNode::new(8));
        expected_2.next = Some(expected_3);
        expected.next = Some(expected_2);

        assert_eq!(result, Some(expected));
    }
}
