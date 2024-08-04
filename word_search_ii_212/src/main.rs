fn main() {
    println!("Hello, world!");
}

#[derive(Default, Debug)]
struct Trie {
    is_end: bool,
    next: [Option<Box<Trie>>; 27],
    word: Option<String>,
}

impl Trie {
    #[inline(always)]
    fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    fn insert(&mut self, word: String) {
        let whole_word = word.clone();
        let mut current = self;

        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            current = current.next[index].get_or_insert_with(|| Box::new(Trie::new()));
        }
        current.is_end = true;
        current.word = Some(whole_word);
    }
}

struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn search(
            i: i32,
            j: i32,
            node: &mut Trie,
            length: usize,
            m: i32,
            n: i32,
            board: &mut Vec<Vec<char>>,
            results: &mut Vec<String>,
        ) {
            if i < 0 || i >= m || j < 0 || j >= n || length > 10 {
                return;
            }

            let dirs = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
            let (i, j) = (i as usize, j as usize);

            // choose
            let c = board[i][j];

            let index = (c as u8 - b'a') as usize;

            if let Some(next) = &mut node.next[index] {
                let mut next = next.as_mut();

                if next.is_end {
                    if let Some(word) = next.word.take() {
                        results.push(word);
                    }
                }

                board[i][j] = '{';

                for (di, dj) in dirs.iter() {
                    search(
                        i as i32 + di,
                        j as i32 + dj,
                        &mut next,
                        length + 1,
                        m,
                        n,
                        board,
                        results,
                    );
                }

                //unchoose
                board[i][j] = c;
            }
        }

        let mut board = board;
        let (m, n) = (board.len() as i32, board[0].len() as i32);
        let mut results: Vec<String> = vec![];
        let mut trie = Trie::new();
        for w in words.iter() {
            trie.insert(w.clone());
        }

        for i in 0..m as usize {
            for j in 0..n as usize {
                let index = (board[i][j] as u8 - b'a') as usize;
                if trie.next[index].is_some() {
                    search(
                        i as i32,
                        j as i32,
                        &mut trie,
                        0,
                        m,
                        n,
                        &mut board,
                        &mut results,
                    );
                }
            }
        }
        results
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_words() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        assert_eq!(
            Solution::find_words(
                board,
                vec![
                    String::from("oath"),
                    "pea".into(),
                    "eat".into(),
                    "rain".into()
                ]
            ),
            vec![String::from("oath"), "eat".into()]
        );
    }
}
