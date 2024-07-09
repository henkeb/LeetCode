// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        unsafe { s.as_bytes_mut() }
            .chunks_mut(k as usize)
            .step_by(2)
            .for_each(|chunk| chunk.reverse());
        s
        // s.as_bytes()
        //     .rchunks(k as usize)
        //     .enumerate()
        //     .step_by(2)
        //     // .filter(|(i, _)| i % 2 == 1)
        //     .rev()
        //     .map(|(_, slice)| String::from_utf8_lossy(slice))
        //     .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd");
    }
}
