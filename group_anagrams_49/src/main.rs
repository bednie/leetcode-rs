fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn freqs(s: String) -> Vec<u8> {
            let mut v_s = vec![0; 26];

            for c in s.chars() {
                v_s[c as usize - 97] += 1;
            }

            //for c in t.chars() {
            //    v_s[c as usize - 97] -= 1;
            //}

            v_s
        }

        let mut ans: Vec<Vec<String>> = vec![];
        let mut map: std::collections::HashMap<Vec<u8>, Vec<usize>> = std::collections::HashMap::new();

        for i in strs.iter() {
            println!("{:?}", i);
            map.entry(freqs(i.to_string())).or_insert(vec![]);
        }

        println!("{:?}", map);

        vec![vec![String::from("")]]
    }
}