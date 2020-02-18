/*
题目: 合并两个有序链表

将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 

示例：

输入：1->2->4, 1->3->4
输出：1->1->2->3->4->4

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/merge-two-sorted-lists
*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head_node = Some(Box::new(ListNode::new(0)));
        let mut result_node = &mut head_node;   

        while l2.is_some() && l1.is_some() {
            let val1 = l1.as_mut().unwrap().val;
            let val2 = l2.as_mut().unwrap().val;
            if val1 < val2 {
                result_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(val1)));
                l1 = l1.as_mut().unwrap().next.take();
            } else {
                result_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(val2)));
                l2 = l2.as_mut().unwrap().next.take();
            }
            result_node = &mut result_node.as_mut().unwrap().next;
        }

        if l1.is_none() {
            result_node.as_mut().unwrap().next = l2;
        } else {
            result_node.as_mut().unwrap().next = l1;
        } 
        head_node.as_mut().unwrap().next.take()
    }
}

fn create_list(start: i32, count: i32, unit: i32) -> Option<Box<ListNode>> {
    let mut head_node = Some(Box::new(ListNode::new(start)));
    let mut prev_node = &mut head_node;
    for i in 1..count as usize {
        match prev_node {
            Some(node) => {
                let idx = i as i32;
                node.next = Some(Box::new(ListNode::new(start + idx * unit)));
                prev_node = &mut prev_node.as_mut().unwrap().next;
            },
            None => {},
        }
    }
    head_node
}

fn main() {
    // 1->2->4, 1->3->4
    let l1 = create_list(1, 4, 2); // 1->3->5->7
    let l2 = create_list(2, 4, 2); // 2->4->6->8
    let expect = create_list(1, 8, 1); // 1->2->3->4->5->6->7->8

    let result = Solution::merge_two_lists(l1, l2);
    assert_eq!(format!("{:?}", result), format!("{:?}", expect));
    println!("success!");
}
