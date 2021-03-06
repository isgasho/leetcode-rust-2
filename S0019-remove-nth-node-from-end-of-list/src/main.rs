// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut step_cnt = 0;
        let mut t = head.as_ref().unwrap();

        while step_cnt < n {
            if let Some(_) = t.next {
                t = t.next.as_ref().unwrap();
                step_cnt += 1;
            } else {
                return head.unwrap().next;
            }
        }
        let mut cur = head.as_ref().unwrap();
        let mut res = Some(Box::new(ListNode::new(cur.val)));
        let mut res_cur = res.as_mut().unwrap();

        while t.next.is_some() {
            t = t.next.as_ref().unwrap();
            cur = cur.next.as_ref().unwrap();
            res_cur.next = Some(Box::new(ListNode::new(cur.val)));
            res_cur = res_cur.next.as_mut().unwrap();
        }

        cur = cur.next.as_ref().unwrap();
        // skip cur, and to the end
        while cur.next.is_some() {
            if let Some(ref n) = cur.next {
                res_cur.next = Some(Box::new(ListNode::new(n.val)));
            }
            cur = cur.next.as_ref().unwrap();
            res_cur = res_cur.next.as_mut().unwrap();
        }

        return res;
    }
}
fn main() {
    let mut a = Some(Box::new(ListNode::new(1)));
    a.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    a.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    a.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    println!("{:?}",Solution::remove_nth_from_end(a, 4));
}

/*
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn rebuild(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            if let Some(mut node) = head {
                let (tail, m) = rebuild(node.next, n);
                if m == n {
                    (tail, m + 1)
                } else {
                    node.next = tail;
                    (Some(node), m + 1)
                }
            } else {
                (None, 1)
            }
        }
        rebuild(head, n).0
    }
}
*/

/*
pub fn remove(head: Option<Box<ListNode>>, n: i32) -> (i32, Option<Box<ListNode>>) {
    if let Some(boxed) = head {
        let (mut tail_index, tail) = remove(boxed.next, n);
        tail_index += 1;

        if tail_index == n {
            (tail_index, tail)
        } else {
            (tail_index, Some(Box::new(ListNode {
                val: boxed.val,
                next: tail
            })))
        }
    } else {
        (0, None)
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove(head, n).1
    }
}
*/
