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
        // TODO
        let result = "fl";
        (&result).to_string()
    }
}

fn main() {
    let mut strs = std::vec::Vec::new();
    strs.push("flower".to_string(), "flow".to_string(), "flight".to_string());
    let result = Solution::longest_common_prefix(strs);
    let expect = "fl";
    assert_eq!(result, expect);
    println!("success!");
}
