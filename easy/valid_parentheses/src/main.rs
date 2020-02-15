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
        // let min_start = 40; // 小括号开始 "("
        // let min_end = 41; // 小括号闭合  ")"
        // let mid_start = 91; // 中括号开始 "["
        // let mid_end = 93; // 中括号闭合 "]"
        // let max_start = 123; // 中括号开始 "{"
        // let max_end = 125; // 中括号闭合 "}"

        let mut min_count = 0;
        let mut mid_count = 0;
        let mut max_count = 0;

        let str_bytes = s.as_bytes();
        let stack = std::vec::Vec::new();

        for i in 0..str_bytes.len() as usize {
            // match str_bytes[i] {
            //     40 => min_count += 1, 
            //     41 => min_count -= 1, 
            //     91 => mid_count += 1, 
            //     93 => mid_count -= 1, 
            //     123 => max_count += 1, 
            //     125 => max_count -= 1, 
            //     _=> continue,
            // }
            if str_bytes[i] == 40 || str_bytes[i] == 91 || str_bytes[i] == 123 {
                if str_bytes[i] == 40 {
                    stack.push(41);
                } else if str_bytes[i] == 91 {
                    stack.push(93);
                } if str_bytes[i] == 123 {
                    stack.push(125);
                }
            } else {
                
            }
        }

        if min_count > 0 || mid_count > 0 || max_count > 0 {
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
