/*
题目: 最大子序和
给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

示例:

输入: [-2,1,-3,4,-1,2,1,-5,4],
输出: 6
解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
进阶:

如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的分治法求解。

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/maximum-subarray
*/

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_result = nums[0];
        let mut cur_result = nums[0];
        for i in 1..nums.len() as usize {
            if nums[i] > cur_result + nums[i] {
                cur_result = nums[i];
            } else {
                cur_result = cur_result + nums[i];
            }

            if cur_result > max_result {
                max_result = cur_result;
            }
        }
        max_result
    }
}

fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    let result = Solution::max_sub_array(nums);
    let expect = 6;
    assert_eq!(result, expect);

    let nums = vec![4,-1,2,1];
    let result = Solution::max_sub_array(nums);
    let expect = 6;
    assert_eq!(result, expect);

    println!("success!");
}
