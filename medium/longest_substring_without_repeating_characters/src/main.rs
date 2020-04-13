/**
题目: 无重复字符的最长子串

给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。

示例 1:

输入: "abcabcbb"
输出: 3
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。

示例 2:
输入: "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。

示例 3:
输入: "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

题目来源：力扣（LeetCode）
题目连接: https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
*/
use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 0;
        let mut map: HashMap<u8, bool> = HashMap::new();
        let char_codes = s.as_bytes();
        let len = char_codes.len();
        while start < len && end < len {
            let end_val = map.get(&char_codes[end]);
            if end_val != Some(&true) {
                map.insert(char_codes[end], true);
                end += 1;
                if max_len < end - start {
                    max_len = end - start
                }
            } else {
                map.remove(&char_codes[start]);
                start += 1;
            }
        }
        return max_len as i32;
    }
}

fn main() {
    let result = Solution::length_of_longest_substring("abcabcbb".to_string());
    let expect = 3;
    assert_eq!(result, expect);

    let result = Solution::length_of_longest_substring("bbbbb".to_string());
    let expect = 1;
    assert_eq!(result, expect);

    let result = Solution::length_of_longest_substring("pwwkew".to_string());
    let expect = 3;
    assert_eq!(result, expect);

    println!("success!");
}
