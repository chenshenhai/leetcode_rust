/*
题目: 加一
给定一个由整数组成的非空数组所表示的非负整数，在该数的基础上加一。

最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。

你可以假设除了整数 0 之外，这个整数不会以零开头。

示例 1:

输入: [1,2,3]
输出: [1,2,4]
解释: 输入数组表示数字 123。
示例 2:

输入: [4,3,2,1]
输出: [4,3,2,2]
解释: 输入数组表示数字 4321。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/plus-one
*/

struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        digits.reverse();
        let mut result_vec = vec![];
        let mut  next_plus = 0;
        for i in 0..digits.len() as usize {
            let mut num = digits[i] + next_plus;
            if i == 0 {
                num += 1;
            }
            if num >= 10 {
                next_plus = 1;
                result_vec.push(num % 10);
                continue;
            } else {
                next_plus = 0;
                result_vec.push(num);
                continue;
            }
        }
        if next_plus == 1 {
            result_vec.push(1);
        } 
        result_vec.reverse();
        result_vec
    }
}

fn main() {
    let result = Solution::plus_one(vec![9, 9, 9, 9]);
    let expect = vec![1, 0, 0, 0, 0];
    assert_eq!(result, expect);
    println!("success!");
}
