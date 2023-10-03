#![allow(unused)]

// TODO: POSSBILE BUG IN RUST !!! (rotate left) mid issue.

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = nums.len();
        let mut swaps = 0;
        let mut max_rotations_left = nums.len() - 2;

        for i in 2..nums.len() {
            while nums[i - 2] == nums[i] && max_rotations_left > 0 {
                let mut end_idx = nums.len();
                if nums.len() - swaps > 0 {
                    end_idx = nums.len() - swaps;
                }

                if end_idx > i {
                    nums[i..end_idx].rotate_left(1);
                    k -= 1;
                    swaps += 1;
                    max_rotations_left -= 1;
                } else {
                    break;
                }
            }
        }

        k as i32
    }
}

fn main() {
    let x = Solution::remove_duplicates(&mut [0, 0, 1, 1, 2, 3, 3, 4, 5].to_vec());
}

//

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = &mut [0, 0, 0, 0, 1, 1, 1, 3, 3, 5, 6, 7, 9, 12, 12].to_vec();

        assert_eq!(12, Solution::remove_duplicates(nums));
    }

    #[test]
    fn test_02() {
        let nums = &mut [1, 1, 1, 2, 2, 3].to_vec();

        assert_eq!(5, Solution::remove_duplicates(nums));
    }

    #[test]
    fn test_03() {
        let nums = &mut [0, 0, 1, 1, 1, 1, 2, 3, 3].to_vec();

        assert_eq!(7, Solution::remove_duplicates(nums));
    }

    #[test]
    fn test_04() {
        let nums = &mut [1, 2, 2, 2].to_vec();

        assert_eq!(3, Solution::remove_duplicates(nums));
    }
}
