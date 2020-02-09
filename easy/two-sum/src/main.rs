// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

// 示例:
// 给定 nums = [2, 7, 11, 15], target = 9
// 因为 nums[0] + nums[1] = 2 + 7 = 9
// 所以返回 [0, 1]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/two-sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::vec;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let len = nums.len();

        for i in 0..len as usize {
            let num1 = nums[i];
            let next = i + 1;
            for j in next..len as usize {
                let num2 = nums[j];
                if num1 + num2 == target {
                    result.push(i as i32);
                    result.push(j as i32);
                    break;
                }
            }  
            if result.len() > 0 {
                break;
            }
        }

        result
    }
}

fn main() {
    // let nums = vec![2, 7, 11, 15];
    // let target = 9;
    // let expect = vec![0, 1];

    let nums = vec![3,2,4];
    let target = 6;
    let expect = vec![1, 2];

    let result = Solution::two_sum(nums, target);
    assert_eq!(result, expect);
    println!("success!");  
}
