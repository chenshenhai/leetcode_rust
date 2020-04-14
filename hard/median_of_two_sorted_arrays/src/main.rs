/*
题目: 寻找两个有序数组的中位数
给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。

请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。

你可以假设 nums1 和 nums2 不会同时为空。

示例 1:

nums1 = [1, 3]
nums2 = [2]

则中位数是 2.0
示例 2:

nums1 = [1, 2]
nums2 = [3, 4]

则中位数是 (2 + 3)/2 = 2.5

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
*/

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // TODO
        let result = 2.5;
        return result;
    }
}

fn main() {
    let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
    let expect = 2.5;
    assert_eq!(result, expect);
    println!("Hello, world!");
}
