/**
 题目: 回文数
 判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

示例 1:

输入: 121
输出: true
示例 2:

输入: -121
输出: false
解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
示例 3:

输入: 10
输出: false
解释: 从右向左读, 为 01 。因此它不是一个回文数。

进阶:
你能不将整数转为字符串来解决这个问题吗？

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/palindrome-number
 */

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut result = false;
        if x < 0 {
            result
        } else if x < 10 {
            result = true;
            result
        } else {
            let mut temp = x;
            let mut nums = vec![];
            loop {
                let remain = temp % 10;
                temp = (temp - remain) / 10;
                nums.push(remain as u8);
                if temp <= 0 {
                    break;
                }
            }

            let size = (nums.len() - nums.len() % 2) / 2;
            for i in 0..size as usize {
                result = true;
                if nums[i] != nums[nums.len() - i - 1] {
                    result = false;
                    break;
                }
            }
            result
        }
    }
}

fn main() {
    let result = Solution::is_palindrome(12321);
    let expect = true;
    assert_eq!(result, expect);

    let result = Solution::is_palindrome(-121);
    let expect = false;
    assert_eq!(result, expect);

    let result = Solution::is_palindrome(10);
    let expect = false;
    assert_eq!(result, expect);

    let result = Solution::is_palindrome(12344566);
    let expect = false;
    assert_eq!(result, expect);

    println!("success");
}
