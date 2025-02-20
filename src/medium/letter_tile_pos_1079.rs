// You have n  tiles, where each tile has one letter tiles[i] printed on it.
//
// Return the number of possible non-empty sequences of letters you can make using the letters printed on those tiles.
//
//
//
// Example 1:
//
// Input: tiles = "AAB"
// Output: 8
// Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
// Example 2:
//
// Input: tiles = "AAABBC"
// Output: 188
// Example 3:
//
// Input: tiles = "V"
// Output: 1
//
//
// Constraints:
//
// 1 <= tiles.length <= 7
// tiles consists of uppercase English letters.
struct Solution;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut char_count: [usize; 26] = [0; 26];
        for ch in tiles.chars() {
            char_count[(ch as u8 - b'A') as usize] += 1;
        }
        Solution::find_seq(&mut char_count)
    }

    fn find_seq(char_count: &mut [usize]) -> i32 {
        let mut count = 0;

        for i in 0..26 {
            if char_count[i] == 0 {
                continue;
            }

            count += 1;
            char_count[i] -= 1;
            count += Solution::find_seq(char_count);
            char_count[i] += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    }
}
