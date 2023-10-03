//! Ex: https://leetcode.com/problems/remove-element/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut collisions = 0;
        let mut j = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if nums[i] == val {
                if i != j {
                    // swap
                    let tmp = nums[i];
                    nums[i] = nums[j];
                    nums[j] = tmp;
                }

                collisions += 1;
                j -= 1;
            }
        }

        nums.len() as i32 - collisions
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = &mut [3, 2, 2, 3].to_vec();

        let x = Solution::remove_element(nums, 3);
    }

    #[test]
    fn test_02() {
        let nums = &mut [0, 1, 2, 2, 3, 0, 4, 2].to_vec();

        let x = Solution::remove_element(nums, 2);
    }

    #[test]
    fn test_03() {
        let nums = &mut [3, 2, 2, 3].to_vec();

        let x = Solution::remove_element(nums, 3);
    }

    #[test]
    fn test_04() {
        let nums = &mut [0, 1, 2, 2, 3, 0, 4, 2].to_vec();

        let x = Solution::remove_element(nums, 2);
    }

    #[test]
    fn test_05() {
        let nums = &mut [2, 5, 6, 6, 7].to_vec();

        let x = Solution::remove_element(nums, 6);
    }
}

// Some other attempts :)

// // Swap:
// let tmp = nums[i];
// nums[i] = nums[cur_last];
// nums[cur_last] = tmp;
// cur_last -= 1;

// x.len();

// .collect::<Vec<Option<&mut i32>>>();

// nums.iter_mut()
//     .map(|v| {
//         // if *v == val {
//         // }

//         if *v as i32 != val as i32 {
//             return Some(v);
//         } else {
//             return None;
//         }
//     })
//     .collect::<Vec<Option<&mut i32>>>();

// // nums.iter().flat_map(|v|)

// nums.len()

// nums.iter().filter_map(|v| if *)

// let x: Vec<&i32> = nums
//     .iter()
//     .map(|v| {
//         if *v as i32 != val as i32 {
//             return Some(v);
//         } else {
//             return None;
//         }
//     })
//     .flatten()
//     .collect();
