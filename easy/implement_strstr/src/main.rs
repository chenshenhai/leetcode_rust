/*
题目: 实现 strStr()

实现 strStr() 函数。

给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从0开始)。如果不存在，则返回  -1。

示例 1:

输入: haystack = "hello", needle = "ll"
输出: 2
示例 2:

输入: haystack = "aaaaa", needle = "bba"
输出: -1
说明:

当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。

对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与C语言的 strstr() 以及 Java的 indexOf() 定义相符。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/implement-strstr

*/

struct Solution {}


impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut result = -1;
        let haystack_vec = haystack.as_bytes();
        let needle_vec = needle.as_bytes();
        if haystack_vec.len() > 0 && needle_vec.len() > 0 && needle_vec.len() < haystack_vec.len() {
            let char2 = needle_vec[0];
            for i in 0..haystack_vec.len() as usize {
                let char1 = haystack_vec[i];
                if char1 == char2 {
                    for j in 1..needle_vec.len() as usize {
                        if i + j < haystack_vec.len() {
                            if haystack_vec[i + j] == needle_vec[j] {
                                if j == needle_vec.len() - 1 {
                                    result = i as i32;
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                } 
                if result > 0 {
                    break;
                }
            }
        }

        result
    }
}


fn main() {
    let haystack = String::from("hello");
    let needle = String::from("ll");
    let result = Solution::str_str(haystack, needle);
    let expect = 2;
    assert_eq!(result, expect);

    let haystack = String::from("aaaaa");
    let needle = String::from("bba");
    let result = Solution::str_str(haystack, needle);
    let expect = -1;
    assert_eq!(result, expect);
    println!("success!");
}
