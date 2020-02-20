/*
题目: 搜索插入位置
给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

你可以假设数组中无重复元素。

示例 1:

输入: [1,3,5,6], 5
输出: 2

示例 2:
输入: [1,3,5,6], 2
输出: 1


示例 3:
输入: [1,3,5,6], 7
输出: 4

示例 4:
输入: [1,3,5,6], 0
输出: 0

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/search-insert-position
*/

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() as usize {
            if nums[i] == target {
                result = i as i32;
                break;
            } else if i < nums.len() - 1 {
                if nums[i] < target && nums[i + 1] > target {
                    result = (i + 1) as i32;
                    break;
                } 
            }
            if i == nums.len() - 1 {
                result = nums.len() as i32;
            }
        }
        result
    }
}

fn main() {
    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 5);
    let expect = 2;
    assert_eq!(result, expect);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 2);
    let expect = 1;
    assert_eq!(result, expect);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 7);
    let expect = 4;
    assert_eq!(result, expect);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 0);
    let expect = 0;
    assert_eq!(result, expect);

    println!("success!");
}
