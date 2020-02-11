/*
题目: 最长公共前缀
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。

示例 1:

输入: ["flower","flow","flight"]
输出: "fl"
示例 2:

输入: ["dog","racecar","car"]
输出: ""
解释: 输入不存在公共前缀。
说明:

所有输入只包含小写字母 a-z 。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/longest-common-prefix
*/

struct  Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let result = "";
        
        if strs.len() > 1 {
            let mut result_vec: Vec<u8> = vec![];
            let mut idx = 0;
            let mut over = false;
            let mut current_byte = 0;
            loop {
                for i in 0..strs.len() as usize {
                    if over {
                        break;
                    }
                    let bytes = strs[i].as_bytes();
                    if bytes.len() == 0 || bytes.len() <= idx {
                        over = true;
                        break;
                    } else if i == 0 {
                        current_byte = bytes[idx];
                        continue;
                    } else if current_byte != bytes[idx] {
                        over = true;
                        break;
                    }
                    if i + 1 == strs.len() {
                        result_vec.push(current_byte);
                    }
                }
                if over {
                    break;
                }
                idx += 1;
            }

            let result = std::str::from_utf8(&result_vec).unwrap().to_string();
            result 
        } else if strs.len() == 1 {
            let result = strs[0].to_string();
            result
        } else {
            (&result).to_string()
        }
    }
}

fn main() {
    let mut strs = std::vec::Vec::new();
    strs.push("flower".to_string());
    strs.push("flow".to_string());
    strs.push("flight".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "fl";
    assert_eq!(result, expect);



    let mut strs = std::vec::Vec::new();
    strs.push("aabbccdd".to_string());
    strs.push("1122345".to_string());
    strs.push("xxxxxxx".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "";
    assert_eq!(result, expect);

    let mut strs = std::vec::Vec::new();
    strs.push("".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "";
    assert_eq!(result, expect);

    let mut strs = std::vec::Vec::new();
    strs.push("a".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "a";
    assert_eq!(result, expect);

    let mut strs = std::vec::Vec::new();
    strs.push("c".to_string());
    strs.push("c".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "c";
    assert_eq!(result, expect);

    let mut strs = std::vec::Vec::new();
    strs.push("aa".to_string());
    strs.push("a".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "a";
    assert_eq!(result, expect);


    println!("success!");
}
