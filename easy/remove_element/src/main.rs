/*
题目: 移除元素
给定一个数组 nums 和一个值 val，你需要原地移除所有数值等于 val 的元素，返回移除后数组的新长度。

不要使用额外的数组空间，你必须在原地修改输入数组并在使用 O(1) 额外空间的条件下完成。

元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。

示例 1:

给定 nums = [3,2,2,3], val = 3,

函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。

你不需要考虑数组中超出新长度后面的元素。
示例 2:

给定 nums = [0,1,2,2,3,0,4,2], val = 2,

函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。

注意这五个元素可为任意顺序。

你不需要考虑数组中超出新长度后面的元素。
说明:

为什么返回数值是整数，但输出的答案是数组呢?

请注意，输入数组是以“引用”方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。

你可以想象内部操作如下:

// nums 是以“引用”方式传递的。也就是说，不对实参作任何拷贝
int len = removeElement(nums, val);

// 在函数里修改输入数组对于调用者是可见的。
// 根据你的函数返回的长度, 它会打印出数组中该长度范围内的所有元素。
for (int i = 0; i < len; i++) {
    print(nums[i]);
}

题目来源：力扣（LeetCode）
题目链接：https://leetcode-cn.com/problems/remove-element
*/

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;
        while nums.len() > 0 && nums.len() > idx {
            if nums[idx] == val {
                nums.remove(idx);
            } else {
                idx += 1;
            }
        }
        let result = nums.len() as i32;
        return result;
    }
}

fn main() {
    let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let nums = &mut vec;
    let result = Solution::remove_element(nums, 2);
    let expect = 5;
    assert_eq!(result, expect);


    let mut vec = vec![3,2,2,3];
    let nums = &mut vec;
    let result = Solution::remove_element(nums, 3);
    let expect = 2;
    assert_eq!(result, expect);


    println!("success!");
}
