/*
题目: x 的平方根
实现 int sqrt(int x) 函数。

计算并返回 x 的平方根，其中 x 是非负整数。

由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。

示例 1:

输入: 4
输出: 2
示例 2:

输入: 8
输出: 2
说明: 8 的平方根是 2.82842..., 
     由于返回类型是整数，小数部分将被舍去。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/sqrtx
*/

struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut result = 0;
        loop {
            if result >= 46340 {
                break;
            }
            if result * result <= x && (result + 1) * (result + 1) > x {
                break;
            }
            result += 1;
        }
        result
    }
}

fn main() {
    let result = Solution::my_sqrt(1024);
    let expect = 32;
    assert_eq!(result, expect);

    let result = Solution::my_sqrt(2147395600);
    let expect = 46340;
    assert_eq!(result, expect);
    println!("success!");
}
