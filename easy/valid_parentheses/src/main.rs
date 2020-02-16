/*
题目: 有效括号
给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。

有效字符串需满足：

左括号必须用相同类型的右括号闭合。
左括号必须以正确的顺序闭合。
注意空字符串可被认为是有效字符串。

示例 1:

输入: "()"
输出: true
示例 2:

输入: "()[]{}"
输出: true
示例 3:

输入: "(]"
输出: false
示例 4:

输入: "([)]"
输出: false
示例 5:

输入: "{[]}"
输出: true

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/valid-parentheses
*/

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut result = true;
        let min_start = 40; // 小括号开始 "("
        let min_end = 41; // 小括号闭合  ")"
        let mid_start = 91; // 中括号开始 "["
        let mid_end = 93; // 中括号闭合 "]"
        let max_start = 123; // 中括号开始 "{"
        let max_end = 125; // 中括号闭合 "}"

        let str_bytes = s.as_bytes();
        let mut stack = std::vec::Vec::new();

        for i in 0..str_bytes.len() as usize {
            if str_bytes[i] == min_start || str_bytes[i] == mid_start || str_bytes[i] == max_start {
                if str_bytes[i] == min_start {
                    stack.push(min_end);
                } else if str_bytes[i] == mid_start {
                    stack.push(mid_end);
                } if str_bytes[i] == max_start {
                    stack.push(max_end);
                }
            } else {
                let out = stack.pop();
                if out == Some(min_end) && str_bytes[i] == min_end {
                    continue;
                } else if out == Some(mid_end) && str_bytes[i] == mid_end {
                    continue; 
                } else if out == Some(max_end) && str_bytes[i] == max_end {
                    continue; 
                } else {
                    result = false;
                    break;
                }
            }
        }

        if stack.len() > 0 {
            result = false;
        } 

        result
    }
}

fn main() {
    let result = Solution::is_valid("[".to_string());
    let expect = false;
    assert_eq!(result, expect);

    let result = Solution::is_valid("()[]{}".to_string());
    let expect = true;
    assert_eq!(result, expect);

    let result = Solution::is_valid("{[]}".to_string());
    let expect = true;
    assert_eq!(result, expect);

    let result = Solution::is_valid("([)]".to_string());
    let expect = false;
    assert_eq!(result, expect);

    let result = Solution::is_valid("[([]])".to_string());
    let expect = false;
    assert_eq!(result, expect);

    println!("success!");
}
