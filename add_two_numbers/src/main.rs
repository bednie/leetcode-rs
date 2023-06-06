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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next),
                }))
            }
        }
    }
}

fn main() {
    // create new ListNodes
    // Input: l1 = [2,4,3], l2 = [5,6,4]
    let mut l1 = Some(Box::new(ListNode::new(2)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    //l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    println!("{:?}", l1);

    let mut l2 = Some(Box::new(ListNode::new(5)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
    //l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    println!("{:?}", l2);

    // call add_two_numbers
    println!("{:?}",add_two_numbers(l1, l2));
}
