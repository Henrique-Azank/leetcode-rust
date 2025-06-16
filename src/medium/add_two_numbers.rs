// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Public solution function
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Instantiate a carry value
    let mut carry = 0;

    // Instantiate the return node and its head
    let mut solution_node = Some(Box::new(ListNode::new(0)));
    let mut solution_head = solution_node.as_mut();

    // Allows us to access the value of l1 and l2 without changing ownership
    let (mut l1_node, mut l2_node) = (l1.as_ref(), l2.as_ref());

    // While either of the lists exists, continue
    while l1_node.is_some() || l2_node.is_some() || carry != 0 {
        // Instantiate a sum value
        let mut sum: i32 = 0;

        // First list value
        if let Some(node) = l1_node {
            // Add the value
            sum += node.val;
            // Switch up the l1 to the next
            l1_node = node.next.as_ref();
        }

        // Second list value
        if let Some(node) = l2_node {
            // Add the value
            sum += node.val;

            // Swicth the l2 to the next
            l2_node = node.next.as_ref();
        }

        // Add the carry value
        sum += carry;

        // Calculate the new carry value and the sum
        carry = sum / 10;
        sum %= 10;

        // Append the value to the head and go to it
        solution_head.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
        solution_head = solution_head.unwrap().next.as_mut();
    }

    // Return the result list
    return solution_node.unwrap().next;
}
