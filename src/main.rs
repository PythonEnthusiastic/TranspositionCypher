mod transposition {
    pub struct TranspositionCypher {
        pub key: usize,
    }

    impl TranspositionCypher {
        pub fn new(key: usize) -> Self {
            Self {
                key
            }
        }

        fn cypher(&self, phrase: &str) -> String {
            let mut cypher_string: String = String::new();
            let bytes: &[u8] = phrase.as_bytes();
            let row_iter = bytes[..self.key].len();
            let col_iter = bytes.len() / self.key;

            for row in 0..row_iter {
                match bytes.get(row) {
                    Some(val) => cypher_string.push(*val as char),
                    None => break,
                };

                for column in 1..col_iter {
                    let phrase_char = match bytes.get(row + self.key*column) {
                        Some(val) => *val as char,
                        None => break,
                    };

                    cypher_string.push(phrase_char as char);
                }
            }

            cypher_string
        }

        pub fn encrypt(&self, phrase: &str) -> String {
            self.cypher(phrase)
        }

        pub fn decrypt(&self, phrase: &str) -> String {
            self.cypher(phrase)
        }
    }
}

fn main() {
    use transposition::*;

    let cy: TranspositionCypher = TranspositionCypher::new(5);
    let encrypted: String = cy.encrypt("Let them eat cake or else");
    let decrypted: String = cy.decrypt(&encrypted);

    println!("{}", encrypted);
    println!("{}", decrypted);
}
