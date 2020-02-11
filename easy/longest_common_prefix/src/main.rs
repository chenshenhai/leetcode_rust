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
        
        if strs.len() != 0 {
            let mut result_vec: Vec<u8> = vec![];
            let mut idx = 0;
            let mut over = false;
            loop {
                for i in 0..strs.len() as usize {
                    if over {
                        break;
                    }
                    let bytes = strs[i].as_bytes();
                    if i == 0 {
                        result_vec.push(bytes[idx] as u8);
                        continue;
                    }
                    if result_vec[idx] != bytes[idx] {
                        over = true;
                        result_vec.pop();
                        break;
                    }
                }
                if over {
                    break;
                }
                idx += 1;
            }

            let result = std::str::from_utf8(&result_vec).unwrap().to_string();
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

    println!("success!");
}
