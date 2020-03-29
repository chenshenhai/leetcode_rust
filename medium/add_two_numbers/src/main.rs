/**
题目: 两数相加

给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

示例：

输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
原因：342 + 465 = 807

题目来源：力扣（LeetCode）
题目连接: https://leetcode-cn.com/problems/add-two-numbers/
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

struct Solution {}

fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head_node = Some(Box::new(ListNode::new(nums[0])));
    if nums.len() > 1 {
        let mut prev_node = &mut head_node;
        for i in 1..nums.len() as usize {
            match prev_node {
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(nums[i])));
                    prev_node = &mut node.next;
                },
                None => {}
            }
        }
    }
    return head_node;
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut list = Some(Box::new(ListNode::new(0)));

        let mut node1 = &mut l1;
        let mut node2 = &mut l2;
        let mut result_node = &mut list;
        let mut next_add = 0;
        while !(node1.is_none() && node2.is_none()) {
            let mut result_num = 0;
            result_num += next_add;
            if node1.is_some() {
                result_num += node1.as_ref().unwrap().val;
                match node1 {
                    Some(node) => {
                        node1 = &mut node.next;
                    },
                    None => {}
                }
            }
            if node2.is_some() {
                result_num += node2.as_ref().unwrap().val;
                match node2 {
                    Some(node) => {
                        node2 = &mut node.next;
                    },
                    None => {}
                }
            }
            if result_num >= 10 {
                result_num = result_num % 10;
                next_add = 1;
            } else {
                next_add = 0;
            }
            match result_node {
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(result_num)));
                    result_node = &mut node.next;
                },
                None => {}
            }
        }
        if next_add > 0 {
            match result_node {
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(next_add)));
                },
                None => {}
            }
        }
        return list.as_mut().unwrap().next.take();
    }
}

fn main() {
    let nums1 = vec![2, 4, 3];
    let nums2 = vec![5, 6, 4];
    let list1 = create_list(nums1);
    let list2 = create_list(nums2);
    let result = Solution::add_two_numbers(list1, list2);
    let nums_expect = vec![7, 0, 8];
    let expect = create_list(nums_expect);
    assert_eq!(result, expect);

    let nums1 = vec![5];
    let nums2 = vec![5];
    let list1 = create_list(nums1);
    let list2 = create_list(nums2);
    let result = Solution::add_two_numbers(list1, list2);
    let nums_expect = vec![0, 1];
    let expect = create_list(nums_expect);
    assert_eq!(result, expect);
    
    println!("success!");
    
}