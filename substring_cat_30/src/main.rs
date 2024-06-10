fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len() == 0 || words.len() == 0 {
            return vec![];
        }

        let mut ans: Vec<i32> = vec![]; 
        let (mut w_len, mut words_len) = (words[0].len(), words.len());
        let mut word_map: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for w in words.iter() {
            *word_map.entry(w).or_insert(0) += 1;
        }

        for offset in 0..w_len {
            // launch with offset
            // at worst O(n*30)
            let (mut left, mut right) = (offset, offset);
            let mut seen_map: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
            println!("{:?}", left);
            println!("{:?}", right);
            while right < w_len * words_len {
                
            }

        }
        println!("{:?}", s);
        println!("{:?}", word_map);

        ans
    }
}

