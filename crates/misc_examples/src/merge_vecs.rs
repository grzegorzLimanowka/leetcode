#![allow(unused)]

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
                println!("cur nums: {cur_nums1}");
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

    #[test]
    fn testing2() {
        //
        let mut x = vec![1, 3, 5];
        x.insert(1, 23);
        println!("{:?}", x);
    }
}

/*
Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
Output: [1,2,2,3,5,6]
Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
*/
