// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.
//
// In the American keyboard:
//
//     the first row consists of the characters "qwertyuiop",
//     the second row consists of the characters "asdfghjkl", and
//     the third row consists of the characters "zxcvbnm".
struct Solution;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        words
            .iter()
            .filter(|word| {
                word.chars().all(|char| {
                    "qwertyuiop".contains(char.to_ascii_lowercase())
                        || word.chars().all(|char| {
                            "asdfghjkl".contains(char.to_ascii_lowercase())
                                || word
                                    .chars()
                                    .all(|char| "zxcvbnm".contains(char.to_ascii_lowercase()))
                        })
                })
            })
            .for_each(|word| output.push(word.clone()));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_words(vec![
                String::from("Hello"),
                String::from("Alaska"),
                String::from("Dad"),
                String::from("Peace")
            ]),
            vec![String::from("Alaska"), String::from("Dad")]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_words(vec![String::from("omk")]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_words(vec![String::from("adsdf"), String::from("sfd")]),
            vec![String::from("adsdf"), String::from("sfd")]
        );
    }
}
