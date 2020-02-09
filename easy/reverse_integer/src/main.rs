// 题目 整数反转
/*
给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

示例 1:

输入: 123
输出: 321
 示例 2:

输入: -123
输出: -321
示例 3:

输入: 120
输出: 21
注意:

假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31,  2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/reverse-integer
*/

pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let num_str = x.to_string();
        let nums = num_str.as_bytes();
        let mut start = 0;
        let len = nums.len();
        let mut result_vec = vec![];

        let max_num: i32 = 2147483647;
        let min_num: i32 = -2147483648;
        let result = 0;

        if !(x >= min_num && x <= max_num) || x == 0 {
            result
        } else {
            // 如果第一个字符是"-", 就是为负数
            if nums[0] == 45 {
                start = 1;
                // 存入负号
                result_vec.push(nums[0])
            }
            
            let mut is_start = false; // 是否开始
            for i in start..len as usize {
                let idx = len - i - 1 + start;
                if is_start == true {
                    result_vec.push(nums[idx]);
                } else {
                    if nums[idx] > 48 {
                        is_start = true;
                        result_vec.push(nums[idx]);
                    }
                }
            }
            let result_str = std::str::from_utf8(&result_vec).unwrap().to_string();
            let mut result_i64 = result_str.parse::<i64>().unwrap();
            let max_num: i64 = 2147483647;
            let min_num: i64 = -2147483648;
            if !(result_i64 >= min_num && result_i64 <= max_num) || result_i64 == 0 {
                result_i64 = 0;
            }
            let result_str = result_i64.to_string();
            let result = result_str.parse::<i32>().unwrap();
            result
        }   
    }
}



fn main() {
    let result = Solution::reverse(0);
    let expect = 0;
    assert_eq!(result, expect);

    let result = Solution::reverse(-123);
    let expect = -321;
    assert_eq!(result, expect);

    let result = Solution::reverse(123);
    let expect = 321;
    assert_eq!(result, expect);

    let result = Solution::reverse(120);
    let expect = 21;
    assert_eq!(result, expect);

    let result = Solution::reverse(1534236469);
    let expect = 0; // 9646324351;
    assert_eq!(result, expect);

    let result = Solution::reverse(-2147483648);
    let expect = 0;
    assert_eq!(result, expect);

    println!("success!");
}
