//! Ex: https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    /// Works, but it is not optimal in terms of speed
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cur_max = nums[0];
        let mut unique = 1;

        for i in 1..nums.len() {
            if nums[i] > cur_max {
                unique += 1;
                cur_max = nums[i];
            } else {
                for j in i..nums.len() {
                    if nums[j] > cur_max {
                        cur_max = nums[j];
                        let tmp = nums[i];
                        nums[i] = nums[j];
                        nums[j] = tmp;
                        unique += 1;
                        break;
                    }
                }
            }
        }

        unique
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = &mut [0, 0, 0, 0, 1, 1, 1, 3, 3, 5, 6, 7, 9, 12, 12].to_vec();

        assert_eq!(8, Solution::remove_duplicates(nums));
    }

    #[test]
    fn test_02() {
        let nums = &mut [0, 1, 1].to_vec();

        assert_eq!(2, Solution::remove_duplicates(nums));
    }
}
