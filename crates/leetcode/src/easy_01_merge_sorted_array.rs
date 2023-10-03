//! Ex: https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn merge<'a>(
        nums1: &'a mut Vec<i32>,
        m: i32,
        nums2: &'a mut Vec<i32>,
        n: i32,
    ) -> &'a mut Vec<i32> {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);

        let mut idx1 = 0;
        for cur_nums2 in nums2 {
            while let Some(cur_nums1) = nums1.get_mut(idx1) {
                if cur_nums2 > cur_nums1 {
                    idx1 += 1;
                } else {
                    break;
                }
            }

            nums1.insert(idx1, *cur_nums2);
        }

        nums1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn testing() {
        let mut vec1 = vec![1, 2, 3, 0, 0, 0];
        let mut vec2 = vec![2, 5, 6];

        Solution::merge(&mut vec1, 3, &mut vec2, 3);
        println!("{:?}", vec1);

        assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);
    }
}
