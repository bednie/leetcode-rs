fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![]; 
        if s.len() == 0 || words.len() == 0 || s.len() < words.len() * words[0].len() {
            return ans;
        }

        if s.len() == 1 && words.len() == 1 {
            ans.push(0);
            return ans;
        }

        let (mut w_len, mut words_len) = (words[0].len(), words.len());
        let mut word_map: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for w in words.iter() {
            *word_map.entry(w).or_insert(0) += 1;
        }

        let (mut start, mut left, mut right) = (0, 0, 0);
        let mut seen_map: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        
        // slide window
        while right + w_len <= s.len() {
            // fill window
            while start + right + w_len <= start + w_len * (words_len-1) {
                let r_word = &s[right..right+w_len];
                if word_map.contains_key(&r_word) {
                    *seen_map.entry(r_word).or_insert(0) += 1;
                }
                right += 1;
            } 
            
            println!("iterate");
            if seen_map == word_map {
                ans.push(start as i32);
            }
            if right + w_len > s.len() { break; }
            let r_word = &s[right..right+w_len];
            if word_map.contains_key(&r_word) {
                *seen_map.entry(r_word).or_insert(0) += 1;
            } 
            let l_word = &s[left..left + w_len];
            if seen_map.contains_key(&l_word) {
                if let Some(x) = seen_map.get_mut(&l_word) {
                    *x -= 1;
                }
            }
            right += w_len;
            left += w_len;
            start += w_len;
        }
        ans
    }
}