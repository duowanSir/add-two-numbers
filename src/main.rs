fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let result = A::add_two_numbers(l1, l2);
    println!("add two numbers result = {:?}", result);
}

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub trait Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

pub struct A {}

impl Solution for A {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut header: Option<Box<ListNode>> = None;
        let mut tail = &mut header;
        let mut overflow = 0;
        while !l1.is_none() || !l2.is_none() {
            let l1_val = match l1 {
                Some(ref l1) => l1.val,
                None => 0,
            };
            let l2_val = match l2 {
                Some(ref l2) => l2.val,
                None => 0,
            };
            let mut val = l1_val + l2_val + overflow;
            overflow = 0;
            if val >= 10 {
                overflow = 1;
                val = val % 10;
            }
            let new_node = Some(Box::new(ListNode::new(val)));
            tail = match tail {
                Some(ref mut content) => {
                    content.next = new_node;
                    &mut content.next
                }
                None => {
                    header = new_node;
                    &mut header
                }
            };
            l1 = match l1 {
                Some(l1) => l1.next,
                None => None,
            };
            l2 = match l2 {
                Some(l2) => l2.next,
                None => None,
            };
        }
        if overflow > 0 {
            let new_node = Some(Box::new(ListNode::new(overflow)));
            if let Some(content) = tail {
                content.next = new_node;
            }
        }
        return header;
    }
}

pub fn reserve_list_node(list_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list_node = list_node;
    let mut result = None;
    while !list_node.is_none() {
        if let Some(val) = list_node {
            let mut new_node = ListNode::new(val.val);
            new_node.next = result;
            result = Some(Box::new(new_node));
            list_node = val.next;
        }
    }
    return result;
}
