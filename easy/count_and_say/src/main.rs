/*
题目: 外观数列
「外观数列」是一个整数序列，从数字 1 开始，序列中的每一项都是对前一项的描述。前五项如下：

1.     1
2.     11
3.     21
4.     1211
5.     111221
1 被读作  "one 1"  ("一个一") , 即 11。
11 被读作 "two 1s" ("两个一"）, 即 21。
21 被读作 "one 2",  "one 1" （"一个二" ,  "一个一") , 即 1211。

给定一个正整数 n（1 ≤ n ≤ 30），输出外观数列的第 n 项。

注意：整数序列中的每一项将表示为一个字符串。



示例 1:

输入: 1
输出: "1"
解释：这是一个基本样例。
示例 2:

输入: 4
输出: "1211"
解释：当 n = 3 时，序列是 "21"，其中我们有 "2" 和 "1" 两组，"2" 可以读作 "12"，也就是出现频次 = 1 而 值 = 2；类似 "1" 可以读作 "11"。所以答案是 "12" 和 "11" 组合在一起，也就是 "1211"。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/count-and-say
 */

struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result_vec = vec![1];
        let mut prev_vec = vec![1];
        for idx in 1..n as usize {
            let mut count = 1;
            result_vec = vec![];
            for i in 0..prev_vec.len() as usize {
                if i >= prev_vec.len() - 1 {
                    result_vec.push(count);
                    result_vec.push(prev_vec[i]);
                    break;
                }
                if prev_vec[i] == prev_vec[i + 1] {
                    count += 1;
                    continue;
                } else {
                    result_vec.push(count);
                    result_vec.push(prev_vec[i]);
                    count = 1;
                    continue;
                }
            }
            prev_vec = result_vec.clone();
        }

        prev_vec = result_vec.clone();
        result_vec = vec![];
        for i in 0..prev_vec.len() as usize {
            result_vec.push((prev_vec[i] + 48) as u8);
        }

        let result = std::str::from_utf8(&result_vec).unwrap().to_string();
        
        result
    }
}

fn main() {

    let result = Solution::count_and_say(1);
    let expect = "1".to_string();
    assert_eq!(result, expect);

    let result = Solution::count_and_say(2);
    let expect = "11".to_string();
    assert_eq!(result, expect);

    let result = Solution::count_and_say(3);
    let expect = "21".to_string();
    assert_eq!(result, expect);

    let result = Solution::count_and_say(4);
    let expect = "1211".to_string();
    assert_eq!(result, expect);

    let result = Solution::count_and_say(5);
    let expect = "111221".to_string();
    assert_eq!(result, expect);
    println!("success!");
}
