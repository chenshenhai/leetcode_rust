/*
题目: 二进制求和
给定两个二进制字符串，返回他们的和（用二进制表示）。

输入为非空字符串且只包含数字 1 和 0。

示例 1:

输入: a = "11", b = "1"
输出: "100"
示例 2:

输入: a = "1010", b = "1011"
输出: "10101"

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/add-binary
*/

struct Solution {}


impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_vec = a.as_bytes().to_vec();
        let mut b_vec = b.as_bytes().to_vec();
        let mut result_vec = vec![];
        a_vec.reverse();
        b_vec.reverse();
        let mut i = 0;
        let mut next_plus = 0;
        loop {
            if i >= a_vec.len() && i >= b_vec.len() {
                break;
            }
            let mut num = next_plus;
            if i < a_vec.len() {
                num += a_vec[i] - 48;
            }
            if i < b_vec.len() {
                num += b_vec[i] - 48;
            }
            if num >= 2 {
                result_vec.push(num - 2);
                next_plus = 1;
            } else {
                result_vec.push(num );
                next_plus = 0;
            }
            i += 1;
        }
        if next_plus == 1 {
            result_vec.push(1);
        }
        for i in 0..result_vec.len() as usize {
            result_vec[i] = result_vec[i] + 48;
        }
        result_vec.reverse();
        let result = std::str::from_utf8(&result_vec).unwrap().to_string();
        result
    }
}

fn main() {
    let result = Solution::add_binary("1010".to_string(), "1011".to_string());
    let expect = "10101".to_string();
    assert_eq!(result, expect);

    let result = Solution::add_binary("0".to_string(), "0".to_string());
    let expect = "0".to_string();
    assert_eq!(result, expect);
    
    println!("success!");
}
