/*
5.题目: 最长回文子串

给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

示例 1：
输入: "babad"
输出: "bab"
注意: "aba" 也是一个有效答案。

示例 2：
输入: "cbbd"
输出: "bb"

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/longest-palindromic-substring
*/

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let vec_chars = s.as_bytes();
        let len = vec_chars.len();
        if len <= 1 || len > 1000 || (len == 2 && vec_chars[0] == vec_chars[1] ) {
            return s;
        } 
        let mut max_start = 0;
        let mut max_end = 0;
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 0;
        let vec_chars = s.as_bytes();
        
        while start < len - 1 {

            if start != end && vec_chars[start] == vec_chars[end] {
                // 如果首尾相等，就判断首尾内部是否为回文
                let size: u8 = ((end - start)/2) as u8;
                for i in 0..(size+1) as usize {
                    if vec_chars[start + i] != vec_chars[end - i] {
                        break;
                    }
                    if (start + i == end - i) || (start + i + 1 == end - i) {
                        if (end - start) > max_len {
                            max_start = start;
                            max_end = end;
                            max_len = max_end - max_start;
                        }
                        break;
                    }
                }
            }
            if end < len - 1 {
                end += 1;
                continue;
            } else if start < len - 1 {
                start += 1;
                end = start + 1;
            }
        }

        let result = s[max_start..(max_end+1)].to_string();
        return result;
    }
}

fn main() {
    // let result = Solution::longest_palindrome("abb".to_string());
    // let expect = "bb".to_string();
    // assert_eq!(result, expect);

    // let result = Solution::longest_palindrome("bb".to_string());
    // let expect = "bb".to_string();
    // assert_eq!(result, expect);

    // let result = Solution::longest_palindrome("xaba".to_string());
    // let expect = "aba".to_string();
    // assert_eq!(result, expect);

    // let result = Solution::longest_palindrome("xabayaabbaaz".to_string());
    // let expect = "aabbaa".to_string();
    // assert_eq!(result, expect);

    // let result = Solution::longest_palindrome("cbbd".to_string());
    // let expect = "bb".to_string();
    // assert_eq!(result, expect);

    let result = Solution::longest_palindrome("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa".to_string());
    let expect = "aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa".to_string();
    assert_eq!(result, expect);
    
    println!("success!");
}
