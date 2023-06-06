struct Solution;

// adapted from stanislav-iablokov's Rust solution
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len: usize = 0;
        let mut positions: [usize; 128] = [0; 128]; // slot for each possible ASCII char, init. to 0
        let mut start: usize = 0;

        // for each iteration:
        // if we have seen the char before, move start to the right,
        // up the the previous char's position + 1.
        // then compare the distance from end - start + 1 to max_len,
        // updating if greater than previous max_len.
        // finally, add end index + 1  to this char's slot
        // for the next iteration.
        s.chars().enumerate().for_each(|(end, chars)| {
            start = start.max(positions[chars as usize]); // if the current character has been seen before,
                                                          // move start to the position after the previous
                                                          // occurrence of this character ("end + 1" seen below)

            max_len = max_len.max(end - start + 1); // compare current substring length to max_len

            positions[chars as usize] = end + 1; // updates the position of the current
                                                 // character to one plus its current position (end + 1)
        });
        max_len as i32
    }
}

fn main() {
    let s = Solution::length_of_longest_substring(String::from("abcdabcabc"));
    println!("{:#?}", &s);
}
