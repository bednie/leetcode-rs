fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let length: usize = (m + n) as usize;
        let mut i: usize = 1;
        let mut m: usize = m.try_into().unwrap();
        let mut n: usize = n.try_into().unwrap();

        while n > 0 && m > 0 {
            if nums2[n - 1] >= nums1[m - 1] {
                nums1[length - i] = nums2[n - 1];
                n -= 1;
            } else if nums2[n - 1] < nums1[m - 1] {
                nums1[length - i] = nums1[m - 1];
                m -= 1;
            }

            i += 1;
        }

        while n > 0 && m == 0 {
            nums1[length - i] = nums2[n - 1];
            n -= 1;
            i += 1;
        }

        while n == 0 && m > 0 {
            nums1[length - i] = nums1[m - 1];
            m -= 1;
            i += 1;
        }
    }
}
