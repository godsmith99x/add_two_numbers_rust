fn main() {
    let l1 = ListNode {
        val: 2,
        next: Some(Box::new(ListNode { val: 3, next: None })),
    };
    print!("{:#?}", &l1);

    let l2 = ListNode::new(5)
}

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

// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//    l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let l1 = l1.unwrap();
//     let l2 = l2.unwrap();
// }
